//! Window visual styles page event registration.
//!
//! [`setup_all_events`] is the single entry point called from
//! [`WindowVisualStylesPage::new`]. It wires up every event handler for
//! this page in the correct order.

use winsafe::gui;
use winsafe::prelude::{GuiEventsWindow, GuiWindow};

use super::layout::WindowVisualStylesPageLayout;
use crate::ui::tab::layout as tab_layout;

/// Wire up all event handlers for the window visual styles page.
///
/// Must be called once during [`WindowVisualStylesPage::new`], after all
/// controls are constructed but before the message loop starts.
pub(super) fn setup_all_events(tab_page: &gui::TabPage, edit: &gui::Edit) {
    tab_layout::paint_tab_page_background(tab_page);
    setup_resize_event(tab_page, edit);
}

/// Register the `WM_SIZE` handler that repositions all controls on the page.
fn setup_resize_event(tab_page: &gui::TabPage, edit: &gui::Edit) {
    let cloned_edit = edit.clone();

    tab_page.on().wm_size(move |size_info| {
        let page_layout = WindowVisualStylesPageLayout::calculate(
            size_info.client_area.cx,
            size_info.client_area.cy,
        );

        tab_layout::reposition_and_resize_control(
            cloned_edit.hwnd(),
            page_layout.edit_position,
            page_layout.edit_size,
        )?;

        Ok(())
    });
}
