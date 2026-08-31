use rust_i18n::t;
use winsafe::prelude::{GuiEventsParent, GuiEventsWindow, GuiWindow};

use super::build::MainWindow;
use crate::ui;

pub fn register_all_events(main_window_instance: &MainWindow) -> winsafe::AnyResult<()> {
    ui::menu::register_menu_events(main_window_instance);
    register_window_create_event(main_window_instance);
    register_window_min_max_info_event(main_window_instance);
    register_window_size_event(main_window_instance);
    register_window_app_message_event(main_window_instance);
    Ok(())
}

fn register_window_create_event(main_window_instance: &MainWindow) {
    let cloned_main_window_instance = main_window_instance.clone();
    main_window_instance.main_window.on().wm_create(move |_| {
        let main_window_hwnd = cloned_main_window_instance.main_window.hwnd();
        let main_menu_bar = ui::menu::build_main_menu()?;
        main_window_hwnd.SetMenu(&main_menu_bar)?;
        ui::window::layout::center_and_resize_window(main_window_hwnd)?;
        Ok(0)
    });
}

fn register_window_min_max_info_event(main_window_instance: &MainWindow) {
    main_window_instance
        .main_window
        .on()
        .wm_get_min_max_info(|min_max| {
            ui::window::layout::apply_minimum_window_size(min_max.info);
            Ok(())
        });
}

fn register_window_size_event(main_window_instance: &MainWindow) {
    let cloned_main_window_instance = main_window_instance.clone();
    main_window_instance
        .main_window
        .on()
        .wm_size(move |size_info| {
            let status_bar_height = cloned_main_window_instance
                .status_bar
                .hwnd()
                .GetWindowRect()
                .map(|rect| rect.bottom - rect.top)
                .unwrap_or(0);

            let available_height_for_tab = size_info.client_area.cy - status_bar_height;
            cloned_main_window_instance
                .tab_container
                .resize(size_info.client_area.cx, available_height_for_tab)?;
            Ok(())
        });
}

/// Register a custom WM_APP message handler for deferred error dialogs.
///
/// Some operations (e.g. config save during a menu handler) cannot show a modal
/// dialog synchronously because it would block the current message processing.
/// Instead, they store the error message and post WM_APP. This handler picks it
/// up and shows the dialog outside the original call stack.
fn register_window_app_message_event(main_window_instance: &MainWindow) {
    let cloned_main_window_instance = main_window_instance.clone();
    main_window_instance
        .main_window
        .on()
        .wm(winsafe::co::WM::APP, move |_| {
            if let Some(error_message) = cloned_main_window_instance
                .pending_error_message
                .borrow_mut()
                .take()
            {
                cloned_main_window_instance.main_window.hwnd().MessageBox(
                    &error_message,
                    &t!("ERROR"),
                    winsafe::co::MB::OK | winsafe::co::MB::ICONWARNING,
                )?;
            }
            Ok(0)
        });
}
