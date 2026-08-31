use std::{ffi::c_void, ptr};
use windows::{
    Win32::{
        Foundation::{BOOL, LPARAM, LRESULT, TRUE, WPARAM},
        Graphics::Dwm::{DWMWA_TRANSITIONS_FORCEDISABLED, DwmSetWindowAttribute},
        System::Threading::GetCurrentThreadId,
        UI::{
            Controls::{SET_THEME_APP_PROPERTIES_FLAGS, SetThemeAppProperties, SetWindowTheme},
            WindowsAndMessaging::{
                CWPSTRUCT, CallNextHookEx, SetWindowsHookExW, WH_CALLWNDPROC, WM_CREATE,
            },
        },
    },
    core::w,
};

/// A WH_CALLWNDPROC hook that intercepts WM_CREATE for every window on this thread.
///
/// It applies three visual customizations:
/// 1. Disables DWM window transition animations (fade-in on show, etc.)
/// 2. Removes the visual theme from every control (classic flat look)
/// 3. Disables theme app properties so no themed painting occurs
///
/// This runs before the window procedure sees WM_CREATE, ensuring all
/// controls are unstyled from the moment they are created.
extern "system" fn ui_customization_hook_procedure(
    hook_code: i32,
    word_parameter: WPARAM,
    long_parameter: LPARAM,
) -> LRESULT {
    if hook_code >= 0 {
        let call_window_procedure_struct = unsafe { &*(long_parameter.0 as *const CWPSTRUCT) };
        if call_window_procedure_struct.message == WM_CREATE {
            let _ = unsafe {
                DwmSetWindowAttribute(
                    call_window_procedure_struct.hwnd,
                    DWMWA_TRANSITIONS_FORCEDISABLED,
                    ptr::from_ref::<_>(&TRUE).cast::<_>(),
                    size_of::<BOOL>() as u32,
                )
            };

            let _ = unsafe { SetWindowTheme(call_window_procedure_struct.hwnd, w!(""), w!("")) };
            unsafe { SetThemeAppProperties(SET_THEME_APP_PROPERTIES_FLAGS(0)) };
        }
    }

    unsafe { CallNextHookEx(None, hook_code, word_parameter, long_parameter) }
}

/// Install the UI customization hook on the current thread.
///
/// Must be called before any windows are created so the hook
/// can intercept their WM_CREATE messages.
pub fn install_ui_customization_hook() {
    unsafe {
        SetWindowsHookExW(
            WH_CALLWNDPROC,
            Some(ui_customization_hook_procedure),
            None,
            GetCurrentThreadId(),
        )
        .expect("INSTALL_UI_CUSTOMIZATION_HOOK_FAILED");
    };
}
