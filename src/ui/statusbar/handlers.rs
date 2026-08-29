use rust_i18n::t;
use winsafe::{gui, prelude::*};

use crate::ui::tab::pages::settings::CheckboxId;

pub fn register_hover_events(scrollable_panel: &gui::WindowControl, status_bar: &gui::StatusBar) {
    let cloned_status_bar_for_set_cursor = status_bar.clone();
    scrollable_panel.on().wm_set_cursor(move |set_cursor_info| {
        let control_id = set_cursor_info.hwnd.GetDlgCtrlID().unwrap_or(0);
        if let Some(checkbox_id) = CheckboxId::from_control_id(control_id) {
            let description_text = t!(checkbox_id.description_i18n_key());
            cloned_status_bar_for_set_cursor
                .parts()
                .set_texts(&[Some(description_text.as_ref())]);
        } else {
            cloned_status_bar_for_set_cursor
                .parts()
                .set_texts(&[Some("")]);
        }

        Ok(false)
    });
}
