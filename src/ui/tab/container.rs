use winsafe::gui;
use winsafe::prelude::*;

use super::{build, layout};
use crate::ui::pages::{settings::SettingsPage, window_visual_styles::WindowVisualStylesPage};

#[derive(Clone)]
pub struct TabContainer {
    tab_control: gui::Tab,
    tab_pages: Vec<gui::TabPage>,
    settings_page: SettingsPage,
    window_visual_styles_page: WindowVisualStylesPage,
}

impl TabContainer {
    pub fn new(parent_window: &(impl GuiParent + 'static), status_bar: gui::StatusBar) -> Self {
        let settings_page = SettingsPage::new(parent_window, status_bar);
        let window_visual_styles_page = WindowVisualStylesPage::new(parent_window);

        let tab_pages = vec![
            settings_page.clone().into(),
            window_visual_styles_page.clone().into(),
        ];

        let tab_control =
            build::create_tab_control(parent_window, &settings_page, &window_visual_styles_page);

        Self {
            tab_control,
            tab_pages,
            settings_page,
            window_visual_styles_page,
        }
    }

    pub fn resize(
        &self,
        window_client_width: i32,
        window_client_height: i32,
    ) -> winsafe::AnyResult<()> {
        layout::resize_tab_control(&self.tab_control, window_client_width, window_client_height)?;
        layout::resize_current_tab_page(&self.tab_control, &self.tab_pages)?;
        Ok(())
    }

    pub fn update_tab_control_titles(&self) -> winsafe::AnyResult<()> {
        let tab_control_titles = build::get_tab_control_titles();
        for (tab_control_index, tab_control_title) in tab_control_titles.iter().enumerate() {
            let target_tab_control_item = self.tab_control.items().get(tab_control_index as u32);
            target_tab_control_item.set_text(tab_control_title)?;
        }
        Ok(())
    }

    pub fn update_page_contents(&self) -> winsafe::AnyResult<()> {
        self.settings_page.update_texts()?;
        Ok(())
    }
}
