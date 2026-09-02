//! DPI-aware layout constants and calculators for the window visual styles page.
//!
//! All raw pixel constants are defined at 96 DPI (100% scaling).
//! [`gui::dpi_x`] and [`gui::dpi_y`] scale them to the actual display DPI
//! at runtime, so the layout looks correct at any scaling factor.

use winsafe::{POINT, SIZE, gui};

const PAGE_MARGIN: i32 = 10;
const EDIT_HEIGHT: i32 = 26;

/// Pre-calculated positions and sizes for every control on the window visual styles page.
///
/// Computed once per WM_SIZE event from the current tab page client dimensions.
/// All values are DPI-scaled pixels relative to the tab page's client origin.
pub(super) struct WindowVisualStylesPageLayout {
    pub edit_position: POINT,
    pub edit_size: SIZE,
}

impl WindowVisualStylesPageLayout {
    /// Calculate the full page layout from the tab page's current client size.
    pub fn calculate(tab_page_client_width: i32, _tab_page_client_height: i32) -> Self {
        let page_margin = gui::dpi_x(PAGE_MARGIN);
        let edit_height = gui::dpi_y(EDIT_HEIGHT);

        Self {
            edit_position: POINT {
                x: page_margin,
                y: page_margin,
            },
            edit_size: SIZE {
                cx: tab_page_client_width - page_margin * 2,
                cy: edit_height,
            },
        }
    }
}
