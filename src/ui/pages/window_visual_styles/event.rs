//! Window visual styles page event registration.
//!
//! [`setup_all_events`] is the single entry point called from
//! [`WindowVisualStylesPage::new`]. It wires up every event handler for
//! this page in the correct order.

use winsafe::prelude::{GuiEventsWindow, GuiWindow, Handle};
use winsafe::{GetSysColor, HBRUSH, co, gui};

use super::layout::WindowVisualStylesPageLayout;
use crate::ui::tab::layout as tab_layout;

/// Wire up all event handlers for the window visual styles page.
///
/// Must be called once during [`WindowVisualStylesPage::new`], after all
/// controls are constructed but before the message loop starts.
pub(super) fn setup_all_events(
    tab_page: &gui::TabPage,
    outer_edit: &gui::Edit,
    inner_edit: &gui::Edit,
) {
    tab_layout::paint_tab_page_background(tab_page);
    setup_outer_edit_background(tab_page, outer_edit);
    setup_resize_event(tab_page, outer_edit, inner_edit);
}

/// Register the `WM_CTLCOLORSTATIC` handler for the disabled outer edit control.
fn setup_outer_edit_background(tab_page: &gui::TabPage, outer_edit: &gui::Edit) {
    let cloned_outer_edit = outer_edit.clone();
    tab_page.on().wm_ctl_color_static(move |color_info| {
        if color_info.hwnd == *cloned_outer_edit.hwnd() {
            color_info.hdc.SetBkMode(co::BKMODE::OPAQUE)?;
            color_info.hdc.SetBkColor(GetSysColor(co::COLOR::WINDOW))?;
            color_info
                .hdc
                .SetTextColor(GetSysColor(co::COLOR::WINDOWTEXT))?;
            return Ok(HBRUSH::from_sys_color(co::COLOR::WINDOW));
        }
        Ok(HBRUSH::NULL)
    });
}

/// Register the `WM_SIZE` handler that repositions all controls on the page.
fn setup_resize_event(tab_page: &gui::TabPage, outer_edit: &gui::Edit, inner_edit: &gui::Edit) {
    let cloned_outer_edit = outer_edit.clone();
    let cloned_inner_edit = inner_edit.clone();

    tab_page.on().wm_size(move |size_info| {
        let window_visual_styles_page_layout = WindowVisualStylesPageLayout::calculate(
            size_info.client_area.cx,
            size_info.client_area.cy,
        );

        tab_layout::reposition_and_resize_control(
            cloned_outer_edit.hwnd(),
            window_visual_styles_page_layout.outer_edit_position,
            window_visual_styles_page_layout.outer_edit_size,
        )?;

        tab_layout::reposition_and_resize_control(
            cloned_inner_edit.hwnd(),
            window_visual_styles_page_layout.inner_edit_position,
            window_visual_styles_page_layout.inner_edit_size,
        )?;

        // Bring inner edit to top so it's above the outer edit
        tab_layout::bring_control_to_top(cloned_inner_edit.hwnd())?;

        Ok(())
    });
}
