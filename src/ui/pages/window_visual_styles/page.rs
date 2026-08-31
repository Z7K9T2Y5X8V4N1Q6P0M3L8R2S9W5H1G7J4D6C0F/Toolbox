use winsafe::{gui, prelude::*};

use crate::ui::tab::layout as tab_layout;

#[derive(Clone)]
pub struct WindowVisualStylesPage {
    tab_page: gui::TabPage,
}

impl From<WindowVisualStylesPage> for gui::TabPage {
    fn from(window_visual_styles_page: WindowVisualStylesPage) -> gui::TabPage {
        window_visual_styles_page.tab_page.clone()
    }
}

impl WindowVisualStylesPage {
    pub fn new(parent_window: &(impl GuiParent + 'static)) -> Self {
        let tab_page = gui::TabPage::new(parent_window, gui::TabPageOpts::default());
        tab_layout::paint_tab_page_background(&tab_page);
        Self { tab_page }
    }
}
