//! DPI-aware layout constants and calculators for the window visual styles page.
//!
//! All raw pixel constants are defined at 96 DPI (100% scaling).
//! [`gui::dpi_x`] and [`gui::dpi_y`] scale them to the actual display DPI
//! at runtime, so the layout looks correct at any scaling factor.

use winsafe::{POINT, SIZE, gui};

// ---------------------------------------------------------------------------
// Raw pixel constants at 96 DPI
// ---------------------------------------------------------------------------

const PAGE_MARGIN: i32 = 10;
const COMBOBOX_HEIGHT: i32 = 25;
const CONTROL_VERTICAL_GAP: i32 = 10;

// ---------------------------------------------------------------------------
// Helper functions
// ---------------------------------------------------------------------------
/// Clamp a dimension to zero if negative.
///
/// Used throughout layout calculations to ensure widths and heights
/// never become negative when the window is resized very small.
#[inline]
fn non_negative(value: i32) -> i32 {
    value.max(0)
}

// ---------------------------------------------------------------------------
// WindowVisualStylesPageLayout
// ---------------------------------------------------------------------------

/// Pre-calculated positions and sizes for every control on the window visual styles page.
///
/// Computed once per WM_SIZE event from the current tab page client dimensions.
/// All values are DPI-scaled pixels relative to the tab page's client origin.
pub(super) struct WindowVisualStylesPageLayout {
    pub combobox_position: POINT,
    pub combobox_size: SIZE,
    pub listview_position: POINT,
    pub listview_size: SIZE,
}

impl WindowVisualStylesPageLayout {
    /// Calculate the full page layout from the tab page's current client size.
    pub fn calculate(tab_page_client_width: i32, tab_page_client_height: i32) -> Self {
        let page_margin = gui::dpi_x(PAGE_MARGIN);
        let combobox_height = gui::dpi_y(COMBOBOX_HEIGHT);
        let control_vertical_gap = gui::dpi_y(CONTROL_VERTICAL_GAP);

        let combobox_position = POINT {
            x: page_margin,
            y: page_margin,
        };
        let combobox_size = SIZE {
            cx: non_negative(tab_page_client_width - page_margin * 2),
            cy: combobox_height,
        };

        let listview_position = POINT {
            x: page_margin,
            y: page_margin + combobox_height + control_vertical_gap,
        };

        let listview_height = non_negative(
            tab_page_client_height
                - (page_margin + combobox_height + control_vertical_gap + page_margin),
        );

        let listview_size = SIZE {
            cx: non_negative(tab_page_client_width - page_margin * 2),
            cy: listview_height,
        };

        Self {
            combobox_position,
            combobox_size,
            listview_position,
            listview_size,
        }
    }
}
