use rust_i18n::t;
use winsafe::{gui, prelude::*};

use super::builder::checkboxes::CheckboxId;
use super::events;

#[derive(Clone)]
pub struct SettingsPage {
    tab_page: gui::TabPage,
    group_box: gui::Button,
    scrollable_panel: gui::WindowControl,
    content_panel: gui::WindowControl,
    checkboxes: Vec<(CheckboxId, gui::Button)>,
    button_select_all_toggle: gui::Button,
    button_apply: gui::Button,
}

impl From<SettingsPage> for gui::TabPage {
    fn from(settings_page: SettingsPage) -> gui::TabPage {
        settings_page.tab_page.clone()
    }
}

impl SettingsPage {
    pub fn new(parent_window: &(impl GuiParent + 'static), status_bar: gui::StatusBar) -> Self {
        let tab_page = super::builder::create_tab_page(parent_window);
        let group_box = super::builder::create_group_box(&tab_page);
        let scrollable_panel = super::builder::create_scrollable_panel(&tab_page);
        let content_panel = super::builder::create_content_panel(&scrollable_panel);
        let checkboxes = super::builder::checkboxes::create_checkboxes(&content_panel);
        let button_select_all_toggle = super::builder::create_button_select_all_toggle(&tab_page);
        let button_apply = super::builder::create_button_apply(&tab_page);

        events::setup_all_events(
            &tab_page,
            &group_box,
            &scrollable_panel,
            &content_panel,
            &checkboxes,
            &button_select_all_toggle,
            &button_apply,
            &status_bar,
        );

        Self {
            tab_page,
            group_box,
            scrollable_panel,
            content_panel,
            checkboxes,
            button_select_all_toggle,
            button_apply,
        }
    }

    pub fn update_texts(&self) -> winsafe::AnyResult<()> {
        self.group_box
            .hwnd()
            .SetWindowText(&t!("GROUP_BOX_SETTINGS_TITLE"))?;

        for (checkbox_id, checkbox) in &self.checkboxes {
            checkbox.hwnd().SetWindowText(&t!(checkbox_id.i18n_key()))?;
        }

        self.button_select_all_toggle
            .hwnd()
            .SetWindowText(&t!("BUTTON_SELECT_ALL_TOGGLE"))?;

        self.button_apply
            .hwnd()
            .SetWindowText(&t!("BUTTON_APPLY"))?;

        Ok(())
    }
}
