//! Window visual styles page event registration.
//!
//! [`setup_all_events`] is the single entry point called from
//! [`WindowVisualStylesPage::new`]. It wires up every event handler for
//! this page in the correct order.

use winsafe::{gui, prelude::*};

use super::layout::WindowVisualStylesPageLayout;
use crate::ui::tab::layout as tab_layout;

/// Wire up all event handlers for the window visual styles page
///
/// Must be called once during [`WindowVisualStylesPage::new`], after all
/// controls are constructed but before the message loop starts.
pub(super) fn setup_all_events(tab_page: &gui::TabPage, combobox: &gui::ComboBox) {
    tab_layout::paint_tab_page_background(tab_page);
    setup_resize_event(tab_page, combobox);
}

/// Register the `WM_SIZE` handler that repositions the combobox control
fn setup_resize_event(tab_page: &gui::TabPage, combobox: &gui::ComboBox) {
    let cloned_combobox = combobox.clone();

    tab_page.on().wm_size(move |size_info| {
        let window_visual_styles_page_layout = WindowVisualStylesPageLayout::calculate(
            size_info.client_area.cx,
            size_info.client_area.cy,
        );

        cloned_combobox.hwnd().SetWindowPos(
            winsafe::HwndPlace::None,
            window_visual_styles_page_layout.combobox_position,
            window_visual_styles_page_layout.combobox_size,
            winsafe::co::SWP::NOZORDER | winsafe::co::SWP::NOCOPYBITS,
        )?;

        Ok(())
    });
}
