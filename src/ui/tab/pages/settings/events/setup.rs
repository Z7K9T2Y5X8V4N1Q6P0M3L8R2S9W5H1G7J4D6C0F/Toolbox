use winsafe::gui;

use crate::ui;
use crate::ui::tab::pages::settings::builder::checkboxes::CheckboxId;

use super::{action_buttons, resize, scroll};

pub(in crate::ui::tab::pages::settings) fn setup_all_events(
    tab_page: &gui::TabPage,
    group_box: &gui::Button,
    scrollable_panel: &gui::WindowControl,
    content_panel: &gui::WindowControl,
    checkboxes: &[(CheckboxId, gui::Button)],
    button_select_all_toggle: &gui::Button,
    button_apply: &gui::Button,
) {
    ui::tab::utils::setup_tab_page_background_events(tab_page);

    resize::setup_resize_event(
        tab_page,
        group_box,
        scrollable_panel,
        content_panel,
        checkboxes,
        button_select_all_toggle,
        button_apply,
    );

    scroll::setup_scroll_event(scrollable_panel, content_panel);
    scroll::setup_mousewheel_event(scrollable_panel, content_panel);

    action_buttons::setup_button_select_all_toggle_event(button_select_all_toggle, checkboxes);
    action_buttons::setup_button_apply_event(button_apply);
}
