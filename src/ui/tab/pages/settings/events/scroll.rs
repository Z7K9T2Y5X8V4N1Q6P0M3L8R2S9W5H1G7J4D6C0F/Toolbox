use winsafe::{POINT, SCROLLINFO, SIZE, SystemParametersInfo, co, gui, msg, prelude::*};

use crate::ui;

pub(super) fn setup_scroll_event(
    scrollable_panel: &gui::WindowControl,
    content_panel: &gui::WindowControl,
) {
    let cloned_content_panel = content_panel.clone();
    let cloned_scrollable_panel = scrollable_panel.clone();

    scrollable_panel
        .on()
        .wm_v_scroll(move |vertical_scroll_info| {
            let mut current_scroll_info = SCROLLINFO::default();
            current_scroll_info.fMask = co::SIF::ALL;
            cloned_scrollable_panel
                .hwnd()
                .GetScrollInfo(co::SBB::VERT, &mut current_scroll_info)?;

            let old_scroll_position = current_scroll_info.nPos;
            let scrollable_maximum =
                (current_scroll_info.nMax - current_scroll_info.nPage as i32).max(0);

            let scroll_line_height = gui::dpi_y(20);
            let new_scroll_position = match vertical_scroll_info.request {
                co::SB_REQ::LINEUP => (old_scroll_position - scroll_line_height).max(0),
                co::SB_REQ::LINEDOWN => {
                    (old_scroll_position + scroll_line_height).min(scrollable_maximum)
                }
                co::SB_REQ::PAGEUP => {
                    (old_scroll_position - current_scroll_info.nPage as i32).max(0)
                }
                co::SB_REQ::PAGEDOWN => {
                    (old_scroll_position + current_scroll_info.nPage as i32).min(scrollable_maximum)
                }
                co::SB_REQ::THUMBTRACK | co::SB_REQ::THUMBPOSITION => {
                    (vertical_scroll_info.scroll_box_pos as i32)
                        .min(scrollable_maximum)
                        .max(0)
                }
                co::SB_REQ::TOP => 0,
                co::SB_REQ::BOTTOM => scrollable_maximum,
                _ => old_scroll_position,
            };

            if new_scroll_position != old_scroll_position {
                apply_scroll_position(
                    &cloned_scrollable_panel,
                    &cloned_content_panel,
                    new_scroll_position,
                )?;
            }

            Ok(())
        });
}

fn get_wheel_scroll_lines() -> Option<u32> {
    const WHEEL_PAGESCROLL: u32 = u32::MAX;

    let mut wheel_scroll_lines: u32 = 0;
    let fetch_scroll_lines_result = unsafe {
        SystemParametersInfo(
            co::SPI::GETWHEELSCROLLLINES,
            0,
            &mut wheel_scroll_lines,
            co::SPIF::NoValue,
        )
    };
    match fetch_scroll_lines_result {
        Ok(_) if wheel_scroll_lines == WHEEL_PAGESCROLL => None,
        Ok(_) => Some(wheel_scroll_lines),
        Err(_) => Some(3),
    }
}

pub(super) fn setup_mousewheel_event(
    scrollable_panel: &gui::WindowControl,
    content_panel: &gui::WindowControl,
) {
    let cloned_scrollable_panel = scrollable_panel.clone();
    content_panel.on().wm_mouse_wheel(move |mouse_wheel_info| {
        // WORKAROUND: WinSafe#201
        // WmMouseWheel.wheel_distance is mistakenly 0 in some versions.
        // We prioritize the official field and fall back to keys.raw() to get the actual delta.
        // Once updated to a fixed version, this workaround can be removed.
        // See: https://github.com/rodrigocfd/winsafe/issues/201
        let official_wheel_distance = mouse_wheel_info.wheel_distance;
        let hacks_wheel_distance = mouse_wheel_info.keys.raw() as i16;
        let wheel_distance = if official_wheel_distance != 0 {
            official_wheel_distance
        } else {
            hacks_wheel_distance
        };

        let is_scrolling_up = wheel_distance > 0;
        let wheel_notches = (wheel_distance.abs() / 120).max(1) as u32;

        let (request, total_wheel_notches) = match get_wheel_scroll_lines() {
            Some(lines_per_wheel_notch) => {
                let request = if is_scrolling_up {
                    co::SB_REQ::LINEUP
                } else {
                    co::SB_REQ::LINEDOWN
                };
                (request, wheel_notches * lines_per_wheel_notch)
            }
            None => {
                let request = if is_scrolling_up {
                    co::SB_REQ::PAGEUP
                } else {
                    co::SB_REQ::PAGEDOWN
                };
                (request, wheel_notches)
            }
        };

        for _ in 0..total_wheel_notches {
            unsafe {
                cloned_scrollable_panel.hwnd().SendMessage(msg::WmVScroll {
                    request,
                    scroll_box_pos: 0,
                    hcontrol: None,
                });
            }
        }
        Ok(())
    });
}

pub(super) fn apply_scroll_position(
    scrollable_panel: &gui::WindowControl,
    content_panel: &gui::WindowControl,
    new_scroll_position: i32,
) -> winsafe::AnyResult<()> {
    scrollable_panel
        .hwnd()
        .SetScrollPos(co::SBB::VERT, new_scroll_position, true)?;

    let content_panel_client_rect = content_panel.hwnd().GetClientRect()?;
    let content_panel_height = content_panel_client_rect.bottom;

    let scrollable_panel_client_rect = scrollable_panel.hwnd().GetClientRect()?;

    ui::tab::utils::reposition_control(
        content_panel.hwnd(),
        POINT {
            x: 0,
            y: -new_scroll_position,
        },
        SIZE {
            cx: scrollable_panel_client_rect.right,
            cy: content_panel_height,
        },
    )?;

    Ok(())
}
