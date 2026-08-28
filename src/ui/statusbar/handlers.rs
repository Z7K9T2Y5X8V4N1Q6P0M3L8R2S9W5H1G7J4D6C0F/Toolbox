use rust_i18n::t;
use winsafe::{gui, prelude::*};

use crate::ui::tab::pages::settings::CheckboxId;

pub fn register_hover_events(content_panel: &gui::WindowControl, status_bar: &gui::StatusBar) {
    let cloned_status_bar = status_bar.clone();
    let cloned_content_panel = content_panel.clone();

    content_panel.on().wm_set_cursor(move |set_cursor_info| {
        let cursor_hwnd = set_cursor_info.hwnd;
        if cursor_hwnd == *cloned_content_panel.hwnd() {
            cloned_status_bar.parts().set_texts(&[Some("")]);
            return Ok(false);
        }

        let control_id = cursor_hwnd.GetDlgCtrlID().unwrap_or(0) as u16;
        if let Some(checkbox_id) = CheckboxId::from_control_id(control_id) {
            let description_text = t!(checkbox_id.description_i18n_key());
            cloned_status_bar
                .parts()
                .set_texts(&[Some(description_text.as_ref())]);
        } else {
            cloned_status_bar.parts().set_texts(&[Some("")]);
        }

        Ok(false)
    });
}
