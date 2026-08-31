use winsafe::{HWND, HwndPlace, POINT, RECT, SIZE, co, gui, msg, prelude::*};

const TAB_CONTROL_MARGIN: i32 = 10;

pub(crate) fn resize_tab_control(
    tab_control: &gui::Tab,
    window_client_width: i32,
    window_client_height: i32,
) -> winsafe::AnyResult<()> {
    let tab_control_margin = gui::dpi_x(TAB_CONTROL_MARGIN);
    let tab_control_size = SIZE {
        cx: window_client_width - (tab_control_margin * 2),
        cy: window_client_height - (tab_control_margin * 2),
    };

    tab_control.hwnd().SetWindowPos(
        HwndPlace::None,
        POINT {
            x: tab_control_margin,
            y: tab_control_margin,
        },
        tab_control_size,
        co::SWP::NOZORDER,
    )?;

    Ok(())
}

pub(crate) fn resize_current_tab_page(
    tab_control: &gui::Tab,
    tab_pages: &[gui::TabPage],
) -> winsafe::AnyResult<()> {
    let Some(selected_tab_control_item) = tab_control.items().selected() else {
        return Ok(());
    };

    let Some(target_tab_page) = tab_pages.get(selected_tab_control_item.index() as usize) else {
        return Ok(());
    };

    let tab_page_rect = calculate_tab_page_rect(tab_control.hwnd())?;

    let calculated_tab_page_content_size = SIZE {
        cx: tab_page_rect.right - tab_page_rect.left,
        cy: tab_page_rect.bottom - tab_page_rect.top,
    };

    target_tab_page.hwnd().SetWindowPos(
        HwndPlace::None,
        POINT {
            x: tab_page_rect.left,
            y: tab_page_rect.top,
        },
        calculated_tab_page_content_size,
        co::SWP::NOZORDER,
    )?;

    Ok(())
}

/// Calculate the client-area rectangle for a tab page inside the tab control.
///
/// The tab control header (the row of tab titles) occupies some space at the top.
/// TCM_ADJUSTRECT with display_rect=false converts the tab control's full rect
/// into the content area below the header, in the parent's client coordinates.
pub(crate) fn calculate_tab_page_rect(tab_control_hwnd: &HWND) -> winsafe::AnyResult<RECT> {
    let tab_control_parent_hwnd = tab_control_hwnd.GetParent()?;

    let mut tab_page_rect =
        tab_control_parent_hwnd.ScreenToClientRc(tab_control_hwnd.GetWindowRect()?)?;

    unsafe {
        tab_control_hwnd.SendMessage(msg::TcmAdjustRect {
            display_rect: false,
            rect: &mut tab_page_rect,
        });
    }

    Ok(tab_page_rect)
}

/// Reposition and resize a control in a single SetWindowPos call.
pub(crate) fn reposition_and_resize_control(
    control_hwnd: &HWND,
    position: POINT,
    size: SIZE,
) -> winsafe::AnyResult<()> {
    control_hwnd.SetWindowPos(
        HwndPlace::None,
        position,
        size,
        co::SWP::NOZORDER | co::SWP::NOCOPYBITS,
    )?;
    Ok(())
}

/// Bring a control to the top of the Z-order among its siblings.
pub(crate) fn bring_control_to_top(control_hwnd: &HWND) -> winsafe::AnyResult<()> {
    control_hwnd.SetWindowPos(
        HwndPlace::Place(co::HWND_PLACE::TOP),
        POINT::default(),
        SIZE::default(),
        co::SWP::NOMOVE | co::SWP::NOSIZE,
    )?;
    Ok(())
}

/// Paint the tab page background with the system button face color.
pub(crate) fn paint_tab_page_background(tab_page: &gui::TabPage) {
    let cloned_tab_page = tab_page.clone();
    tab_page.on().wm_erase_bkgnd(move |erase_bkgnd_params| {
        let tab_page_client_rect = cloned_tab_page.hwnd().GetClientRect()?;
        let background_brush = winsafe::HBRUSH::GetSysColorBrush(co::COLOR::BTNFACE)?;
        erase_bkgnd_params
            .hdc
            .FillRect(tab_page_client_rect, &background_brush)?;
        Ok(1)
    });
}
