use dioxus::prelude::*;
use freya_elements::elements as dioxus_elements;
use freya_elements::events::MouseEvent;

use freya_hooks::{use_applied_theme, ScrollBarThemeWith};

#[derive(Props, Clone, PartialEq)]
pub struct ScrollThumbProps {
    /// Theme override.
    pub theme: Option<ScrollBarThemeWith>,
    pub clicking_scrollbar: bool,
    pub onmousedown: EventHandler<MouseEvent>,
    #[props(into)]
    pub width: String,
    #[props(into)]
    pub height: String,
}

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum ScrollThumbState {
    #[default]
    Idle,
    // Thumb is being hovered
    Hovering,
}

#[allow(non_snake_case)]
pub fn ScrollThumb(
    ScrollThumbProps {
        theme,
        clicking_scrollbar,
        onmousedown,
        width,
        height,
    }: ScrollThumbProps,
) -> Element {
    let theme = use_applied_theme!(&theme, scroll_bar);
    let mut state = use_signal(ScrollThumbState::default);
    let thumb_background = match *state.read() {
        _ if clicking_scrollbar => theme.active_thumb_background,
        ScrollThumbState::Idle => theme.thumb_background,
        ScrollThumbState::Hovering => theme.hover_thumb_background,
    };

    rsx!(
        rect {
            onmouseenter: move |_| { state.set(ScrollThumbState::Hovering) },
            onmouseleave: move |_| { state.set(ScrollThumbState::Idle) },
            onmousedown: move |e| {
                onmousedown.call(e);
            },
            width: "{width}",
            height: "{height}",
            padding: "2",
            rect {
                width: "100%",
                height: "100%",
                corner_radius: "8",
                background: "{thumb_background}",
            }
        }
    )
}
