use crate::transport::{self, Events, Payload, Request, Transport};
use anyhow::{anyhow, Context, Result};

use dap_types::{
    requests::{
        ConfigurationDone, Continue, Initialize, Launch, Next, Pause, SetBreakpoints, StepBack,
        StepIn, StepOut,
    },
    ConfigurationDoneArguments, ContinueArguments, InitializeRequestArgumentsPathFormat,
    LaunchRequestArguments, NextArguments, PauseArguments, SetBreakpointsArguments,
    SetBreakpointsResponse, Source, SourceBreakpoint, StepBackArguments, StepInArguments,
    StepOutArguments, SteppingGranularity,
};
use futures::{
    channel::mpsc::{channel, unbounded, UnboundedReceiver, UnboundedSender},
    AsyncBufRead, AsyncReadExt, AsyncWrite, SinkExt as _, StreamExt,
};
use gpui::{AppContext, AsyncAppContext};
use serde_json::Value;
use smol::{
    io::BufReader,
    net::TcpStream,
    process::{self, Child},
};
use std::{
    net::{Ipv4Addr, SocketAddrV4},
    path::PathBuf,
    process::Stdio,
    sync::atomic::{AtomicU64, Ordering},
    time::Duration,
};
use util::ResultExt;

pub enum TransportType {
    TCP,
    STDIO,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct DebugAdapterClientId(pub usize);

#[derive(Debug)]
pub struct DebugAdapterClient {
    _process: Option<Child>,
    server_tx: UnboundedSender<Payload>,
    request_count: AtomicU64,
    capabilities: Option<dap_types::Capabilities>,
}

impl DebugAdapterClient {
    pub async fn new<F>(
        transport_type: TransportType,
        command: &str,
        args: Vec<&str>,
        port: u16,
        project_path: PathBuf,
        cx: &mut AsyncAppContext,
        event_handler: F,
    ) -> Result<Self>
    where
        F: FnMut(Events, &mut AppContext) + 'static + Send + Sync + Clone,
    {
        match transport_type {
            TransportType::TCP => {
                Self::create_tcp_client(command, args, port, project_path, cx, event_handler).await
            }
            TransportType::STDIO => {
                Self::create_stdio_client(command, args, port, project_path, cx).await
            }
        }
    }

    async fn create_tcp_client<F>(
        command: &str,
        args: Vec<&str>,
        port: u16,
        project_path: PathBuf,
        cx: &mut AsyncAppContext,
        event_handler: F,
    ) -> Result<Self>
    where
        F: FnMut(Events, &mut AppContext) + 'static + Send + Sync + Clone,
    {
        let mut command = process::Command::new(command);
        command
            .current_dir(project_path)
            .args(args)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .kill_on_drop(true);

        let process = command
            .spawn()
            .with_context(|| "failed to spawn command.")?;

        // give the adapter some time to spin up the tcp server
        cx.background_executor()
            .timer(Duration::from_millis(1000))
            .await;

        let address = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), port);

        let (rx, tx) = TcpStream::connect(address).await?.split();

        Self::handle_transport(
            Box::new(BufReader::new(rx)),
            Box::new(tx),
            None,
            Some(process),
            cx,
            event_handler,
        )
    }

    async fn create_stdio_client(
        command: &str,
        args: Vec<&str>,
        port: u16,
        project_path: PathBuf,
        cx: &mut AsyncAppContext,
    ) -> Result<Self> {
        todo!("not implemented")
    }

    pub fn handle_transport<F>(
        rx: Box<dyn AsyncBufRead + Unpin + Send>,
        tx: Box<dyn AsyncWrite + Unpin + Send>,
        err: Option<Box<dyn AsyncBufRead + Unpin + Send>>,
        process: Option<Child>,
        cx: &mut AsyncAppContext,
        event_handler: F,
    ) -> Result<Self>
    where
        F: FnMut(Events, &mut AppContext) + 'static + Send + Sync + Clone,
    {
        let (server_rx, server_tx) = Transport::start(rx, tx, err, cx);
        let (client_tx, client_rx) = unbounded::<Payload>();

        let client = Self {
            server_tx: server_tx.clone(),
            _process: process,
            request_count: AtomicU64::new(0),
            capabilities: None,
        };

        cx.spawn(move |cx| Self::handle_events(client_rx, event_handler, cx))
            .detach();

        cx.spawn(move |_| Self::handle_recv(server_rx, server_tx, client_tx))
            .detach();

        Ok(client)
    }

    async fn handle_events<F>(
        mut client_rx: UnboundedReceiver<Payload>,
        mut event_handler: F,
        cx: AsyncAppContext,
    ) -> Result<()>
    where
        F: FnMut(Events, &mut AppContext) + 'static + Send + Sync + Clone,
    {
        while let Some(payload) = client_rx.next().await {
            cx.update(|cx| match payload {
                Payload::Event(event) => event_handler(*event, cx),
                _ => unreachable!(),
            })?;
        }

        anyhow::Ok(())
    }

    async fn handle_recv(
        mut server_rx: UnboundedReceiver<Payload>,
        mut server_tx: UnboundedSender<Payload>,
        mut client_tx: UnboundedSender<Payload>,
    ) {
        while let Some(payload) = server_rx.next().await {
            match payload {
                Payload::Event(ev) => client_tx.send(Payload::Event(ev)).await.log_err(),
                Payload::Response(res) => server_tx.send(Payload::Response(res)).await.log_err(),
                Payload::Request(req) => client_tx.send(Payload::Request(req)).await.log_err(),
            };
        }
    }

    pub async fn request<R: dap_types::requests::Request>(
        &self,
        arguments: R::Arguments,
    ) -> Result<R::Response> {
        let serialized_arguments = serde_json::to_value(arguments)?;

        let (callback_tx, mut callback_rx) = channel::<Result<transport::Response>>(1);

        let request = Request {
            back_ch: Some(callback_tx),
            seq: self.next_request_id(),
            command: R::COMMAND.to_string(),
            arguments: Some(serialized_arguments),
        };

        self.server_tx
            .clone()
            .send(Payload::Request(request))
            .await?;

        let response = callback_rx.next().await.ok_or(anyhow!("no response"))??;

        match response.success {
            true => Ok(serde_json::from_value(response.body.unwrap_or_default())?),
            false => Err(anyhow!("Request failed")),
        }
    }

    pub fn next_request_id(&self) -> u64 {
        self.request_count.fetch_add(1, Ordering::Relaxed)
    }

    pub async fn initialize(&mut self) -> Result<dap_types::Capabilities> {
        let args = dap_types::InitializeRequestArguments {
            client_id: Some("zed".to_owned()),
            client_name: Some("Zed".to_owned()),
            adapter_id: "xdebug".into(), // TODO: read from config
            locale: Some("en-us".to_owned()),
            path_format: Some(InitializeRequestArgumentsPathFormat::Path),
            supports_variable_type: Some(true),
            supports_variable_paging: Some(false),
            supports_run_in_terminal_request: Some(false), // TODO: we should support this
            supports_memory_references: Some(false),
            supports_progress_reporting: Some(false),
            supports_invalidated_event: Some(false),
            lines_start_at1: Some(true),
            columns_start_at1: Some(true),
            supports_memory_event: None,
            supports_args_can_be_interpreted_by_shell: None,
            supports_start_debugging_request: None,
        };

        let capabilities = self.request::<Initialize>(args).await?;

        self.capabilities = Some(capabilities.clone());

        Ok(capabilities)
    }

    pub async fn launch(&self, custom: Value) -> Result<()> {
        self.request::<Launch>(LaunchRequestArguments { raw: custom })
            .await
    }

    pub async fn resume(&self, thread_id: u64) {
        self.request::<Continue>(ContinueArguments {
            thread_id,
            single_thread: None,
        })
        .await
        .log_err();
    }

    pub async fn step_over(&self, thread_id: u64) {
        self.request::<Next>(NextArguments {
            thread_id,
            granularity: Some(SteppingGranularity::Statement),
            single_thread: None,
        })
        .await
        .log_err();
    }

    pub async fn step_in(&self, thread_id: u64) {
        self.request::<StepIn>(StepInArguments {
            thread_id,
            target_id: None,
            granularity: Some(SteppingGranularity::Statement),
            single_thread: None,
        })
        .await
        .log_err();
    }

    pub async fn step_out(&self, thread_id: u64) {
        self.request::<StepOut>(StepOutArguments {
            thread_id,
            granularity: Some(SteppingGranularity::Statement),
            single_thread: None,
        })
        .await
        .log_err();
    }

    pub async fn step_back(&self, thread_id: u64) {
        self.request::<StepBack>(StepBackArguments {
            thread_id,
            single_thread: None,
            granularity: Some(SteppingGranularity::Statement),
        })
        .await
        .log_err();
    }

    pub async fn restart(&self, thread_id: u64) {
        self.request::<StepBack>(StepBackArguments {
            thread_id,
            single_thread: None,
            granularity: Some(SteppingGranularity::Statement),
        })
        .await
        .log_err();
    }

    pub async fn pause(&self, thread_id: u64) {
        self.request::<Pause>(PauseArguments { thread_id })
            .await
            .log_err();
    }

    pub async fn set_breakpoints(
        &self,
        path: PathBuf,
        breakpoints: Option<Vec<SourceBreakpoint>>,
    ) -> Result<SetBreakpointsResponse> {
        self.request::<SetBreakpoints>(SetBreakpointsArguments {
            source: Source {
                path: Some(String::from(path.to_string_lossy())),
                name: None,
                source_reference: None,
                presentation_hint: None,
                origin: None,
                sources: None,
                adapter_data: None,
                checksums: None,
            },
            breakpoints,
            source_modified: None,
            lines: None,
        })
        .await
    }

    pub async fn configuration_done(&self) -> Result<()> {
        self.request::<ConfigurationDone>(ConfigurationDoneArguments)
            .await
    }
}