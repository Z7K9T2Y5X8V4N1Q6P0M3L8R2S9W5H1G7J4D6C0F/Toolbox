//! DPI-aware layout constants and calculators for the window visual styles page.
//!
//! All raw pixel constants are defined at 96 DPI (100% scaling).
//! [`gui::dpi_x`] and [`gui::dpi_y`] scale them to the actual display DPI
//! at runtime, so the layout looks correct at any scaling factor.

use rust_i18n::t;
use winsafe::{POINT, SIZE, gui, msg};

const PAGE_MARGIN: i32 = 10;

/// Pre-calculated positions and sizes for every control on the window visual styles page
///
/// Computed once per WM_SIZE event from the current tab page client dimensions.
/// All values are DPI-scaled pixels relative to the tab page's client origin.
pub(super) struct WindowVisualStylesPageLayout {
    pub edit_position: POINT,
    pub edit_size: SIZE,
}

impl WindowVisualStylesPageLayout {
    /// Calculate the full page layout from the tab page's current client size
    pub fn calculate(
        tab_page_client_width: i32,
        _tab_page_client_height: i32,
        edit_hwnd: &winsafe::HWND,
    ) -> Self {
        let page_margin = gui::dpi_x(PAGE_MARGIN);
        let edit_height = Self::calculate_edit_height(edit_hwnd);

        let edit_position = POINT {
            x: page_margin,
            y: page_margin,
        };
        let edit_size = SIZE {
            cx: tab_page_client_width - page_margin * 2,
            cy: edit_height,
        };

        Self {
            edit_position,
            edit_size,
        }
    }

    /// Calculate the appropriate edit control height based on its font metrics
    ///
    /// Returns the font height plus the border size, with no additional padding.
    /// The caller is responsible for using `EM_SETRECT` to eliminate the edit
    /// control's default internal margins.
    ///
    /// # Panics
    ///
    /// Panics if the device context cannot be obtained, the font cannot be selected,
    /// or text metrics cannot be retrieved. These failures indicate a critical
    /// system-level problem that cannot be recovered from.
    fn calculate_edit_height(edit_hwnd: &winsafe::HWND) -> i32 {
        let edit_device_context = edit_hwnd
            .GetDC()
            .expect(&t!("ERROR_GET_DEVICE_CONTEXT_FAILED"));

        let font_handle = unsafe { edit_hwnd.SendMessage(msg::WmGetFont {}) };
        if let Some(font_handle) = font_handle {
            edit_device_context
                .SelectObject(&font_handle)
                .expect(&t!("ERROR_SELECT_FONT_FAILED"));
        }
        let text_metrics = edit_device_context
            .GetTextMetrics()
            .expect(&t!("ERROR_GET_TEXT_METRICS_FAILED"));

        // tmHeight is already DPI-scaled by the system
        let font_height = text_metrics.tmHeight;

        // SM_CYEDGE returns the DPI-scaled border height for WS_EX_CLIENTEDGE
        let border_height = winsafe::GetSystemMetrics(winsafe::co::SM::CYEDGE) * 2;
        font_height + border_height
    }
}
