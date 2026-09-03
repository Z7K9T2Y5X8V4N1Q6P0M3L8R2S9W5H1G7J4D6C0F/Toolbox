//! Window visual styles page event registration.
//!
//! [`setup_all_events`] is the single entry point called from
//! [`WindowVisualStylesPage::new`]. It wires up every event handler for
//! this page in the correct order.

use winsafe::prelude::{GuiEventsWindow, GuiWindow};
use winsafe::{RECT, gui, msg};

use super::layout::WindowVisualStylesPageLayout;
use crate::ui::tab::layout as tab_layout;

/// Wire up all event handlers for the window visual styles page
///
/// Must be called once during [`WindowVisualStylesPage::new`], after all
/// controls are constructed but before the message loop starts.
pub(super) fn setup_all_events(tab_page: &gui::TabPage, edit: &gui::Edit) {
    tab_layout::paint_tab_page_background(tab_page);
    setup_resize_event(tab_page, edit);
}

/// Register the `WM_SIZE` handler that repositions the edit control
fn setup_resize_event(tab_page: &gui::TabPage, edit: &gui::Edit) {
    let cloned_edit = edit.clone();

    tab_page.on().wm_size(move |size_info| {
        let layout = WindowVisualStylesPageLayout::calculate(
            size_info.client_area.cx,
            size_info.client_area.cy,
            cloned_edit.hwnd(),
        );

        cloned_edit.hwnd().SetWindowPos(
            winsafe::HwndPlace::None,
            layout.edit_position,
            layout.edit_size,
            winsafe::co::SWP::NOZORDER | winsafe::co::SWP::NOCOPYBITS,
        )?;

        adjust_edit_format_rect(&cloned_edit);

        Ok(())
    });
}

/// Adjust the edit control's formatting rectangle to eliminate all internal padding
///
/// Uses `EM_SETRECT` to set the text formatting area to the full client area,
/// making the text sit flush against the top and bottom edges of the control.
fn adjust_edit_format_rect(edit: &gui::Edit) {
    let edit_client_rect = match edit.hwnd().GetClientRect() {
        Ok(edit_client_rect) => edit_client_rect,
        Err(_) => return,
    };

    let format_edit_rect = RECT {
        left: 0,
        top: 0,
        right: edit_client_rect.right,
        bottom: edit_client_rect.bottom,
    };

    unsafe {
        edit.hwnd().SendMessage(msg::EmSetRect {
            rect: Some(&format_edit_rect),
            is_absolute_coords: true,
        });
    }
}
