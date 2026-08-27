use rust_i18n::t;
use winsafe::{POINT, co, gui};

use super::ids::CheckboxId;
use crate::ui::tab::pages::settings::layout::CheckboxLayoutCalculator;

pub(in crate::ui::tab::pages::settings) fn create_checkboxes(
    content_panel: &gui::WindowControl,
) -> Vec<(CheckboxId, gui::Button)> {
    let layout_calculator = CheckboxLayoutCalculator::new();

    CheckboxId::all()
        .iter()
        .enumerate()
        .map(|(index, &checkbox_id)| {
            let POINT { x, y } = layout_calculator.calculate_checkbox_position(index);
            let checkbox = gui::Button::new(
                content_panel,
                gui::ButtonOpts {
                    text: &t!(checkbox_id.i18n_key()),
                    position: (x, y),
                    width: 250,
                    height: layout_calculator.checkbox_height(),
                    ctrl_id: checkbox_id.window_control_id(),
                    control_style: co::BS::AUTOCHECKBOX,
                    ..Default::default()
                },
            );
            (checkbox_id, checkbox)
        })
        .collect()
}
