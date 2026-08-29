use winsafe::{HBRUSH, HWND, HwndPlace, POINT, RECT, SIZE, co, gui, msg, prelude::*};

pub(super) fn setup_tab_page_background_events(tab_page: &gui::TabPage) {
    let cloned_tab_page_for_background = tab_page.clone();
    tab_page.on().wm_erase_bkgnd(move |erase_bkgnd_params| {
        let tab_page_content_client_rect = cloned_tab_page_for_background.hwnd().GetClientRect()?;
        let background_brush = HBRUSH::GetSysColorBrush(co::COLOR::BTNFACE)?;
        erase_bkgnd_params
            .hdc
            .FillRect(tab_page_content_client_rect, &background_brush)?;
        Ok(1)
    });
}

pub(super) fn reposition_control(
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

pub(super) fn bring_control_to_top(control_hwnd: &HWND) -> winsafe::AnyResult<()> {
    control_hwnd.SetWindowPos(
        HwndPlace::Place(co::HWND_PLACE::TOP),
        POINT::default(),
        SIZE::default(),
        co::SWP::NOMOVE | co::SWP::NOSIZE,
    )?;
    Ok(())
}

pub(super) fn calculate_tab_page_rect(tab_control_hwnd: &HWND) -> winsafe::AnyResult<RECT> {
    let tab_control_parent_hwnd = tab_control_hwnd.GetParent()?;

    let mut tab_page_screen_to_client_rect =
        tab_control_parent_hwnd.ScreenToClientRc(tab_control_hwnd.GetWindowRect()?)?;

    unsafe {
        tab_control_hwnd.SendMessage(msg::TcmAdjustRect {
            display_rect: false,
            rect: &mut tab_page_screen_to_client_rect,
        });
    }

    Ok(tab_page_screen_to_client_rect)
}
