//! DPI-aware layout constants and calculators for the window visual styles page.
//!
//! All raw pixel constants are defined at 96 DPI (100% scaling).
//! [`gui::dpi_x`] and [`gui::dpi_y`] scale them to the actual display DPI
//! at runtime, so the layout looks correct at any scaling factor.

use winsafe::{POINT, SIZE, gui};

const PAGE_MARGIN: i32 = 10;
const OUTER_EDIT_HEIGHT: i32 = 28;
const INNER_EDIT_HORIZONTAL_PADDING: i32 = 4;
const INNER_EDIT_VERTICAL_PADDING: i32 = 4;

/// Pre-calculated positions and sizes for every control on the window visual styles page.
///
/// Computed once per WM_SIZE event from the current tab page client dimensions.
/// All values are DPI-scaled pixels relative to the tab page's client origin.
pub(super) struct WindowVisualStylesPageLayout {
    pub outer_edit_position: POINT,
    pub outer_edit_size: SIZE,
    pub inner_edit_position: POINT,
    pub inner_edit_size: SIZE,
}

impl WindowVisualStylesPageLayout {
    /// Calculate the full page layout from the tab page's current client size.
    pub fn calculate(tab_page_client_width: i32, _tab_page_client_height: i32) -> Self {
        let page_margin = gui::dpi_x(PAGE_MARGIN);
        let outer_edit_height = gui::dpi_y(OUTER_EDIT_HEIGHT);
        let inner_edit_horizontal_padding = gui::dpi_x(INNER_EDIT_HORIZONTAL_PADDING);
        let inner_edit_vertical_padding = gui::dpi_y(INNER_EDIT_VERTICAL_PADDING);

        let outer_edit_position = POINT {
            x: page_margin,
            y: page_margin,
        };
        let outer_edit_size = SIZE {
            cx: tab_page_client_width - page_margin * 2,
            cy: outer_edit_height,
        };

        // Inner edit is positioned inside outer edit with padding
        let inner_edit_position = POINT {
            x: outer_edit_position.x + inner_edit_horizontal_padding,
            y: outer_edit_position.y + inner_edit_vertical_padding,
        };
        let inner_edit_size = SIZE {
            cx: outer_edit_size.cx - inner_edit_horizontal_padding * 2,
            cy: outer_edit_size.cy - inner_edit_vertical_padding * 2,
        };

        Self {
            outer_edit_position,
            outer_edit_size,
            inner_edit_position,
            inner_edit_size,
        }
    }
}
