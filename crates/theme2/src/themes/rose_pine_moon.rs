use gpui2::rgba;

use crate::{PlayerTheme, SyntaxTheme, Theme, ThemeMetadata};

pub fn rose_pine_moon() -> Theme {
    Theme {
        metadata: ThemeMetadata {
            name: "Rosé Pine Moon".into(),
            is_light: false,
        },
        transparent: rgba(0x00000000).into(),
        mac_os_traffic_light_red: rgba(0xec695eff).into(),
        mac_os_traffic_light_yellow: rgba(0xf4bf4eff).into(),
        mac_os_traffic_light_green: rgba(0x61c553ff).into(),
        border: rgba(0x504c68ff).into(),
        border_variant: rgba(0x504c68ff).into(),
        border_focused: rgba(0x435255ff).into(),
        border_transparent: rgba(0x00000000).into(),
        elevated_surface: rgba(0x38354eff).into(),
        surface: rgba(0x28253cff).into(),
        background: rgba(0x38354eff).into(),
        filled_element: rgba(0x38354eff).into(),
        filled_element_hover: rgba(0xffffff1e).into(),
        filled_element_active: rgba(0xffffff28).into(),
        filled_element_selected: rgba(0x2f3639ff).into(),
        filled_element_disabled: rgba(0x00000000).into(),
        ghost_element: rgba(0x00000000).into(),
        ghost_element_hover: rgba(0xffffff14).into(),
        ghost_element_active: rgba(0xffffff1e).into(),
        ghost_element_selected: rgba(0x2f3639ff).into(),
        ghost_element_disabled: rgba(0x00000000).into(),
        text: rgba(0xe0def4ff).into(),
        text_muted: rgba(0x85819eff).into(),
        text_placeholder: rgba(0xea6e92ff).into(),
        text_disabled: rgba(0x605d7aff).into(),
        text_accent: rgba(0x9bced6ff).into(),
        icon_muted: rgba(0x85819eff).into(),
        syntax: SyntaxTheme {
            highlights: vec![
                ("type.builtin".into(), rgba(0x9ccfd8ff).into()),
                ("variable".into(), rgba(0xe0def4ff).into()),
                ("punctuation".into(), rgba(0x908caaff).into()),
                ("number".into(), rgba(0x5cc1a3ff).into()),
                ("comment".into(), rgba(0x6e6a86ff).into()),
                ("string.special".into(), rgba(0xc4a7e6ff).into()),
                ("string.escape".into(), rgba(0x8682a0ff).into()),
                ("function.method".into(), rgba(0xea9a97ff).into()),
                ("predictive".into(), rgba(0x516b83ff).into()),
                ("punctuation.delimiter".into(), rgba(0xaeabc6ff).into()),
                ("primary".into(), rgba(0xe0def4ff).into()),
                ("link_text".into(), rgba(0x9ccfd8ff).into()),
                ("string.regex".into(), rgba(0xc4a7e6ff).into()),
                ("constructor".into(), rgba(0x9bced6ff).into()),
                ("constant".into(), rgba(0x5cc1a3ff).into()),
                ("emphasis.strong".into(), rgba(0x9bced6ff).into()),
                ("function".into(), rgba(0xea9a97ff).into()),
                ("hint".into(), rgba(0x728aa2ff).into()),
                ("preproc".into(), rgba(0xe0def4ff).into()),
                ("property".into(), rgba(0x9bced6ff).into()),
                ("punctuation.list_marker".into(), rgba(0xaeabc6ff).into()),
                ("emphasis".into(), rgba(0x9bced6ff).into()),
                ("attribute".into(), rgba(0x9bced6ff).into()),
                ("title".into(), rgba(0xf5c177ff).into()),
                ("keyword".into(), rgba(0x3d8fb0ff).into()),
                ("string".into(), rgba(0xf5c177ff).into()),
                ("text.literal".into(), rgba(0xc4a7e6ff).into()),
                ("embedded".into(), rgba(0xe0def4ff).into()),
                ("comment.doc".into(), rgba(0x8682a0ff).into()),
                ("variant".into(), rgba(0x9bced6ff).into()),
                ("label".into(), rgba(0x9bced6ff).into()),
                ("punctuation.special".into(), rgba(0xaeabc6ff).into()),
                ("string.special.symbol".into(), rgba(0xc4a7e6ff).into()),
                ("tag".into(), rgba(0x9ccfd8ff).into()),
                ("enum".into(), rgba(0xc4a7e6ff).into()),
                ("boolean".into(), rgba(0xea9a97ff).into()),
                ("punctuation.bracket".into(), rgba(0xaeabc6ff).into()),
                ("operator".into(), rgba(0x3d8fb0ff).into()),
                ("type".into(), rgba(0x9ccfd8ff).into()),
                ("link_uri".into(), rgba(0xea9a97ff).into()),
            ],
        },
        status_bar: rgba(0x38354eff).into(),
        title_bar: rgba(0x38354eff).into(),
        toolbar: rgba(0x232136ff).into(),
        tab_bar: rgba(0x28253cff).into(),
        editor: rgba(0x232136ff).into(),
        editor_subheader: rgba(0x28253cff).into(),
        editor_active_line: rgba(0x28253cff).into(),
        terminal: rgba(0x232136ff).into(),
        image_fallback_background: rgba(0x38354eff).into(),
        git_created: rgba(0x5cc1a3ff).into(),
        git_modified: rgba(0x9bced6ff).into(),
        git_deleted: rgba(0xea6e92ff).into(),
        git_conflict: rgba(0xf5c177ff).into(),
        git_ignored: rgba(0x605d7aff).into(),
        git_renamed: rgba(0xf5c177ff).into(),
        players: [
            PlayerTheme {
                cursor: rgba(0x9bced6ff).into(),
                selection: rgba(0x9bced63d).into(),
            },
            PlayerTheme {
                cursor: rgba(0x5cc1a3ff).into(),
                selection: rgba(0x5cc1a33d).into(),
            },
            PlayerTheme {
                cursor: rgba(0xa683a0ff).into(),
                selection: rgba(0xa683a03d).into(),
            },
            PlayerTheme {
                cursor: rgba(0xc4a7e6ff).into(),
                selection: rgba(0xc4a7e63d).into(),
            },
            PlayerTheme {
                cursor: rgba(0xc4a7e6ff).into(),
                selection: rgba(0xc4a7e63d).into(),
            },
            PlayerTheme {
                cursor: rgba(0x3e8fb0ff).into(),
                selection: rgba(0x3e8fb03d).into(),
            },
            PlayerTheme {
                cursor: rgba(0xea6e92ff).into(),
                selection: rgba(0xea6e923d).into(),
            },
            PlayerTheme {
                cursor: rgba(0xf5c177ff).into(),
                selection: rgba(0xf5c1773d).into(),
            },
        ],
    }
}
