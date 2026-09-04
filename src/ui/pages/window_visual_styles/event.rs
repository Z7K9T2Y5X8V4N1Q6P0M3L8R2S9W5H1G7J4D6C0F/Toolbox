//! Window visual styles page event registration.
//!
//! [`setup_all_events`] is the single entry point called from [`WindowVisualStylesPage::new`].
//! It wires up every event handler for the window visual styles page in the correct order.

use rust_i18n::t;
use winsafe::{WString, gui, msg, prelude::*};

use super::layout::WindowVisualStylesPageLayout;
use crate::ui::tab::layout as tab_layout;

/// Wire up all event handlers for the window visual styles page.
///
/// Must be called once during [`WindowVisualStylesPage::new`], after all controls
/// are constructed but before the message loop starts.
pub(super) fn setup_all_events(
    tab_page: &gui::TabPage,
    combobox: &gui::ComboBox,
    listview: &gui::ListView,
) {
    tab_layout::paint_tab_page_background(tab_page);
    setup_resize_event(tab_page, combobox, listview);
    setup_combobox_cue_banner_event(tab_page, combobox);
}

// ---------------------------------------------------------------------------
// Combobox CueBanner
// ---------------------------------------------------------------------------

/// Register the `WM_CREATE` handler on the tab page to set the cue banner (placeholder) for the combobox.
fn setup_combobox_cue_banner_event(tab_page: &gui::TabPage, combobox: &gui::ComboBox) {
    let cloned_combobox = combobox.clone();
    tab_page.on().wm_create(move |_| {
        unsafe {
            cloned_combobox
                .hwnd()
                .SendMessage(msg::CbSetCueBanner {
                    text: WString::from_str(t!("COMBOBOX_CUE_BANNER")),
                })
                .ok()
        };
        Ok(0)
    });
}

// ---------------------------------------------------------------------------
// Resize
// ---------------------------------------------------------------------------

/// Register the `WM_SIZE` handler that repositions all controls on the page.
///
/// Both controls are repositioned and resized dynamically whenever the window size changes.
/// The combobox stays at the top with fixed height, and the listview fills all remaining
/// vertical space below it, maintaining consistent margins on all sides.
fn setup_resize_event(tab_page: &gui::TabPage, combobox: &gui::ComboBox, listview: &gui::ListView) {
    let cloned_combobox = combobox.clone();
    let cloned_listview = listview.clone();

    tab_page.on().wm_size(move |size_info| {
        let window_visual_styles_page_layout = WindowVisualStylesPageLayout::calculate(
            size_info.client_area.cx,
            size_info.client_area.cy,
        );

        tab_layout::reposition_and_resize_control(
            cloned_combobox.hwnd(),
            window_visual_styles_page_layout.combobox_position,
            window_visual_styles_page_layout.combobox_size,
        )?;

        tab_layout::reposition_and_resize_control(
            cloned_listview.hwnd(),
            window_visual_styles_page_layout.listview_position,
            window_visual_styles_page_layout.listview_size,
        )?;

        Ok(())
    });
}
