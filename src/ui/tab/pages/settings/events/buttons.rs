use winsafe::{co, gui, msg, prelude::*};

use crate::ui::tab::pages::settings::builder::checkboxes::CheckboxId;

pub(super) fn setup_button_select_all_toggle_event(
    button_select_all_toggle: &gui::Button,
    checkboxes: &[(CheckboxId, gui::Button)],
) {
    let checkboxes_buttons: Vec<gui::Button> = checkboxes
        .iter()
        .map(|(_, checkbox)| checkbox.clone())
        .collect();

    button_select_all_toggle.on().bn_clicked(move || {
        let all_currently_checked = checkboxes_buttons.iter().all(|checkbox_button| {
            let check_state = unsafe { checkbox_button.hwnd().SendMessage(msg::BmGetCheck {}) };
            check_state == co::BST::CHECKED
        });

        let target_check_state = if all_currently_checked {
            co::BST::UNCHECKED
        } else {
            co::BST::CHECKED
        };

        for checkbox_button in &checkboxes_buttons {
            unsafe {
                checkbox_button.hwnd().SendMessage(msg::BmSetCheck {
                    state: target_check_state,
                });
            }
        }

        Ok(())
    });
}

pub(super) fn setup_button_apply_event(button_apply: &gui::Button) {
    button_apply.on().bn_clicked(move || Ok(()));
}
