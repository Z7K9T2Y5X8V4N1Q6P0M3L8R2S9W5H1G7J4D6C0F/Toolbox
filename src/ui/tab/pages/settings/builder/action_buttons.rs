use rust_i18n::t;
use winsafe::gui;

use crate::ui::tab::pages::settings::layout::{BUTTON_HEIGHT, BUTTON_WIDTH};

pub(in crate::ui::tab::pages::settings) fn create_button_select_all_toggle(
    parent_window: &gui::TabPage,
) -> gui::Button {
    gui::Button::new(
        parent_window,
        gui::ButtonOpts {
            text: &t!("BUTTON_SELECT_ALL_TOGGLE"),
            width: BUTTON_WIDTH,
            height: BUTTON_HEIGHT,
            ..Default::default()
        },
    )
}

pub(in crate::ui::tab::pages::settings) fn create_button_apply(
    parent_window: &gui::TabPage,
) -> gui::Button {
    gui::Button::new(
        parent_window,
        gui::ButtonOpts {
            text: &t!("BUTTON_APPLY"),
            width: BUTTON_WIDTH,
            height: BUTTON_HEIGHT,
            ..Default::default()
        },
    )
}
