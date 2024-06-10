use anyhow::Result;
use dap::{
    client::{Client, TransportType},
    requests::{Initialize, Launch, LaunchRequestArguments},
    transport::Payload,
};
use futures::channel::mpsc::UnboundedReceiver;
use gpui::{
    actions, Action, AppContext, AsyncWindowContext, EventEmitter, FocusHandle, FocusableView,
    View, ViewContext, WeakView,
};
use ui::{
    div, h_flex,
    prelude::{IntoElement, Pixels, WindowContext},
    px, ButtonCommon, Element, IconButton, IconName, ParentElement, Render, Styled, Tooltip,
    VisualContext,
};
use workspace::{
    dock::{DockPosition, Panel, PanelEvent},
    Workspace,
};

actions!(debug, [TogglePanel]);

pub struct DebugPanel {
    pub position: DockPosition,
    pub zoomed: bool,
    pub active: bool,
    pub focus_handle: FocusHandle,
    pub size: Pixels,
    debug_client: (Client, UnboundedReceiver<Payload>),
}

impl DebugPanel {
    pub fn new(
        position: DockPosition,
        debug_client: (Client, UnboundedReceiver<Payload>),
        cx: &mut WindowContext,
    ) -> Self {
        Self {
            position,
            zoomed: false,
            active: false,
            focus_handle: cx.focus_handle(),
            size: px(300.),
            debug_client,
        }
    }

    pub async fn load(
        workspace: WeakView<Workspace>,
        cx: AsyncWindowContext,
    ) -> Result<View<Self>> {
        let mut cx = cx.clone();
        let (mut debug_client, events) = Client::new(
            TransportType::TCP,
            "python3",
            vec![
                "-m",
                "debugpy",
                "--listen",
                "localhost:5668",
                "--wait-for-client",
                "test.py",
            ],
            None,
            &mut cx,
        )
        .await?;

        let mut cx = cx.clone();

        let args = dap::requests::InitializeArguments {
            client_id: Some("zed".to_owned()),
            client_name: Some("zed".to_owned()),
            adapter_id: "debugpy".into(),
            locale: Some("en-us".to_owned()),
            lines_start_at_one: Some(true),
            columns_start_at_one: Some(true),
            path_format: Some("path".to_owned()),
            supports_variable_type: Some(true),
            supports_variable_paging: Some(false),
            supports_run_in_terminal_request: Some(false),
            supports_memory_references: Some(false),
            supports_progress_reporting: Some(false),
            supports_invalidated_event: Some(false),
        };

        let capabilities = debug_client.request::<Initialize>(args).await;

        dbg!(capabilities);

        // launch/attach
        let launch = debug_client
            .request::<Launch>(LaunchRequestArguments {
                no_debug: Some(true),
                __restart: None,
            })
            .await;

        dbg!(launch);

        // configure request

        workspace.update(&mut cx, |_, cx| {
            cx.new_view(|cx| DebugPanel::new(DockPosition::Bottom, (debug_client, events), cx))
        })
    }
}

impl EventEmitter<PanelEvent> for DebugPanel {}

impl FocusableView for DebugPanel {
    fn focus_handle(&self, _cx: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Panel for DebugPanel {
    fn persistent_name() -> &'static str {
        "DebugPanel"
    }

    fn position(&self, _cx: &WindowContext) -> DockPosition {
        self.position
    }

    fn position_is_valid(&self, _position: DockPosition) -> bool {
        true
    }

    fn set_position(&mut self, position: DockPosition, _cx: &mut ViewContext<Self>) {
        self.position = position;
        // TODO:
        // cx.update_global::<SettingsStore>(f)
    }

    fn size(&self, _cx: &WindowContext) -> Pixels {
        self.size
    }

    fn set_size(&mut self, size: Option<Pixels>, _cx: &mut ViewContext<Self>) {
        self.size = size.unwrap();
    }

    fn icon(&self, _cx: &WindowContext) -> Option<IconName> {
        None
    }

    fn icon_tooltip(&self, _cx: &WindowContext) -> Option<&'static str> {
        None
    }

    fn toggle_action(&self) -> Box<dyn Action> {
        Box::new(TogglePanel)
    }

    fn icon_label(&self, _: &WindowContext) -> Option<String> {
        None
    }

    fn is_zoomed(&self, _cx: &WindowContext) -> bool {
        false
    }

    fn starts_open(&self, _cx: &WindowContext) -> bool {
        false
    }

    fn set_zoomed(&mut self, _zoomed: bool, _cx: &mut ViewContext<Self>) {}

    fn set_active(&mut self, _active: bool, _cx: &mut ViewContext<Self>) {}
}

impl Render for DebugPanel {
    fn render(&mut self, _: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .child(
                h_flex()
                    .p_2()
                    .gap_2()
                    .child(
                        IconButton::new("debug-play", IconName::Play)
                            // .on_click(|_, cx| {
                            //     self.debug_client.0.request("continue", None);
                            // })
                            .tooltip(move |cx| Tooltip::text("Start debug", cx)),
                    )
                    .child(
                        IconButton::new("debug-step-over", IconName::Play)
                            .tooltip(move |cx| Tooltip::text("Step over", cx)),
                    )
                    .child(
                        IconButton::new("debug-go-in", IconName::Play)
                            .tooltip(move |cx| Tooltip::text("Go in", cx)),
                    )
                    .child(
                        IconButton::new("debug-go-out", IconName::Play)
                            .tooltip(move |cx| Tooltip::text("Go out", cx)),
                    )
                    .child(
                        IconButton::new("debug-restart", IconName::Play)
                            .tooltip(move |cx| Tooltip::text("Restart", cx)),
                    )
                    .child(
                        IconButton::new("debug-stop", IconName::Play)
                            .tooltip(move |cx| Tooltip::text("Stop", cx)),
                    ),
            )
            .into_any()
    }
}
