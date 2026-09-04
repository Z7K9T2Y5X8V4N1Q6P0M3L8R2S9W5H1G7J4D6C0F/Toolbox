//! [`WindowVisualStylesPage`] — the public handle for the window visual styles tab page.
//!
//! Owns all controls and exposes only the operations the rest of the
//! application needs: construction and locale-driven text updates.

use winsafe::{gui, prelude::*};

/// The window visual styles tab page.
///
/// Cloning is cheap — all inner fields are reference-counted WinSafe handles.
/// The [`From`] impl allows this to be handed directly to [`gui::Tab`] pages.
#[derive(Clone)]
pub struct WindowVisualStylesPage {
    tab_page: gui::TabPage,
    edit: gui::Edit,
    listview: gui::ListView,
}

impl From<WindowVisualStylesPage> for gui::TabPage {
    fn from(window_visual_styles_page: WindowVisualStylesPage) -> gui::TabPage {
        window_visual_styles_page.tab_page.clone()
    }
}

impl WindowVisualStylesPage {
    /// Construct the window visual styles page, creating all controls and registering all events.
    ///
    /// Must be called before the message loop starts, on the same thread as
    /// the parent window.
    pub fn new(parent_window: &(impl GuiParent + 'static)) -> Self {
        let tab_page = super::build::create_tab_page(parent_window);
        let edit = super::build::create_edit(&tab_page);
        let listview = super::build::create_listview(&tab_page);

        super::event::setup_all_events(&tab_page, &edit, &listview);

        Self {
            tab_page,
            edit,
            listview,
        }
    }
}
