use std::cell::Cell;
use std::rc::Rc;

use rust_i18n::t;
use winsafe::{HWND, TRACKMOUSEEVENT, TrackMouseEvent, co, gui, prelude::*};

use crate::ui::pages::settings::CheckboxId;

pub fn register_hover_events(
    checkboxes: &[(CheckboxId, gui::Button)],
    status_bar: &gui::StatusBar,
) {
    for (checkbox_id, checkbox_button) in checkboxes {
        register_single_checkbox_hover_event(*checkbox_id, checkbox_button, status_bar);
    }
}

/// Show a description in the status bar when the mouse enters a checkbox,
/// and clear it when the mouse leaves.
///
/// Uses WM_MOUSEMOVE + TrackMouseEvent to detect enter/leave transitions,
/// since Win32 has no native "mouse enter" message for standard controls.
fn register_single_checkbox_hover_event(
    checkbox_id: CheckboxId,
    checkbox_button: &gui::Button,
    status_bar: &gui::StatusBar,
) {
    let is_mouse_inside = Rc::new(Cell::new(false));

    let cloned_status_bar_for_mouse_move = status_bar.clone();
    let cloned_checkbox_button_for_mouse_move = checkbox_button.clone();
    let cloned_is_mouse_inside_for_mouse_move = is_mouse_inside.clone();

    checkbox_button.on_subclass().wm_mouse_move(move |_| {
        if !cloned_is_mouse_inside_for_mouse_move.get() {
            cloned_is_mouse_inside_for_mouse_move.set(true);

            let description_text = t!(checkbox_id.description_i18n_key());
            cloned_status_bar_for_mouse_move
                .parts()
                .set_texts(&[Some(description_text.as_ref())]);

            // Request a WM_MOUSELEAVE notification so we know when to clear the status bar.
            let mut track_mouse_event_info = TRACKMOUSEEVENT::default();
            track_mouse_event_info.dwFlags = co::TME::LEAVE;
            track_mouse_event_info.hwndTrack =
                unsafe { HWND::from_ptr(cloned_checkbox_button_for_mouse_move.hwnd().ptr()) };
            TrackMouseEvent(&mut track_mouse_event_info)?;
        }

        Ok(())
    });

    let cloned_is_mouse_inside_for_mouse_leave = is_mouse_inside;
    let cloned_status_bar_for_mouse_leave = status_bar.clone();
    checkbox_button.on_subclass().wm_mouse_leave(move || {
        cloned_is_mouse_inside_for_mouse_leave.set(false);
        cloned_status_bar_for_mouse_leave
            .parts()
            .set_texts(&[Some("")]);
        Ok(())
    });
}
