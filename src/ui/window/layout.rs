//! Main window sizing and positioning utilities.
//!
//! Provides two functions used during window initialization:
//! - [`center_and_resize_window`] — sets the window to the default size and
//!   centers it on the primary monitor
//! - [`apply_minimum_window_size`] — enforces a minimum track size so the
//!   user cannot resize the window smaller than the UI requires

use winsafe::gui;

/// The default window width at 96 DPI (100% scaling).
const WINDOW_WIDTH: i32 = 310;

/// The default window height at 96 DPI (100% scaling).
const WINDOW_HEIGHT: i32 = 585;

/// Resize the main window to the default size and center it on the primary monitor.
///
/// Called once during `WM_CREATE` after the window handle is valid.
/// Both dimensions are scaled to the actual display DPI before applying.
pub fn center_and_resize_window(main_window_handle: &winsafe::HWND) -> winsafe::AnyResult<()> {
    let calculated_window_size = winsafe::SIZE {
        cx: gui::dpi_x(WINDOW_WIDTH),
        cy: gui::dpi_y(WINDOW_HEIGHT),
    };

    let system_screen_width = winsafe::GetSystemMetrics(winsafe::co::SM::CXSCREEN);
    let system_screen_height = winsafe::GetSystemMetrics(winsafe::co::SM::CYSCREEN);
    let centered_window_position = winsafe::POINT {
        x: (system_screen_width - calculated_window_size.cx) / 2,
        y: (system_screen_height - calculated_window_size.cy) / 2,
    };

    main_window_handle.SetWindowPos(
        winsafe::HwndPlace::None,
        centered_window_position,
        calculated_window_size,
        winsafe::co::SWP::NOZORDER,
    )?;

    Ok(())
}

/// Set the minimum track size in a `WM_GETMINMAXINFO` handler.
///
/// Prevents the user from resizing the window below the default dimensions,
/// which would cause controls to overlap or be clipped.
pub fn apply_minimum_window_size(min_max_info: &mut winsafe::MINMAXINFO) {
    min_max_info.ptMinTrackSize = winsafe::POINT {
        x: gui::dpi_x(WINDOW_WIDTH),
        y: gui::dpi_y(WINDOW_HEIGHT),
    };
}
