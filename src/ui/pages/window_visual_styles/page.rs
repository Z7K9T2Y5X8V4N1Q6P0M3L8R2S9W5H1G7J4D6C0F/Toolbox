//! [`WindowVisualStylesPage`] — the public handle for the window visual styles tab page.

use winsafe::{gui, prelude::*};

use crate::ui::tab::layout as tab_layout;

/// The window visual styles tab page.
///
/// Cloning is cheap — the inner [`gui::TabPage`] is a reference-counted WinSafe handle.
/// The [`From`] impl allows this to be handed directly to [`gui::Tab`] pages.
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
    /// Construct the page and register its background paint event.
    ///
    /// Must be called before the message loop starts, on the same thread
    /// as the parent window.
    pub fn new(parent_window: &(impl GuiParent + 'static)) -> Self {
        let tab_page = gui::TabPage::new(parent_window, gui::TabPageOpts::default());
        tab_layout::paint_tab_page_background(&tab_page);
        Self { tab_page }
    }
}
