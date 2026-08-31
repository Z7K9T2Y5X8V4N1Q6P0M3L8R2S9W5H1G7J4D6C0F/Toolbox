use rust_i18n::t;
use winsafe::{POINT, co, gui, prelude::*};

use super::layout::CheckboxLayoutCalculator;
use super::state::CheckboxId;

pub(super) fn create_tab_page(parent_window: &(impl GuiParent + 'static)) -> gui::TabPage {
    gui::TabPage::new(
        parent_window,
        gui::TabPageOpts {
            class_style: co::CS::HREDRAW | co::CS::VREDRAW,
            ..Default::default()
        },
    )
}

pub(super) fn create_group_box(parent_window: &gui::TabPage) -> gui::Button {
    gui::Button::new(
        parent_window,
        gui::ButtonOpts {
            text: &t!("GROUP_BOX_SETTINGS_TITLE"),
            control_style: co::BS::GROUPBOX,
            ..Default::default()
        },
    )
}

/// Create the outer scrollable panel that hosts the content panel.
///
/// This panel has WS_VSCROLL to show a vertical scrollbar, and WS_EX_COMPOSITED
/// to reduce flicker during scroll operations.
pub(super) fn create_scrollable_panel(parent_window: &gui::TabPage) -> gui::WindowControl {
    gui::WindowControl::new(
        parent_window,
        gui::WindowControlOpts {
            style: co::WS::CHILD
                | co::WS::VISIBLE
                | co::WS::CLIPCHILDREN
                | co::WS::CLIPSIBLINGS
                | co::WS::VSCROLL,
            ex_style: co::WS_EX::COMPOSITED,
            class_style: co::CS::HREDRAW | co::CS::VREDRAW,
            class_bg_brush: gui::Brush::Color(co::COLOR::BTNFACE),
            ..Default::default()
        },
    )
}

/// Create the inner content panel that holds all checkboxes.
///
/// This panel is a child of the scrollable panel and can be taller than its
/// parent. Scrolling is achieved by shifting this panel's Y position.
pub(super) fn create_content_panel(parent_window: &gui::WindowControl) -> gui::WindowControl {
    gui::WindowControl::new(
        parent_window,
        gui::WindowControlOpts {
            style: co::WS::CHILD | co::WS::VISIBLE | co::WS::CLIPCHILDREN | co::WS::CLIPSIBLINGS,
            class_style: co::CS::HREDRAW | co::CS::VREDRAW,
            class_bg_brush: gui::Brush::Color(co::COLOR::BTNFACE),
            ..Default::default()
        },
    )
}

pub(super) fn create_checkboxes(
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

pub(super) fn create_button_select_all_toggle(parent_window: &gui::TabPage) -> gui::Button {
    gui::Button::new(
        parent_window,
        gui::ButtonOpts {
            text: &t!("BUTTON_SELECT_ALL_TOGGLE"),
            width: super::layout::BUTTON_WIDTH,
            height: super::layout::BUTTON_HEIGHT,
            ..Default::default()
        },
    )
}

pub(super) fn create_button_apply(parent_window: &gui::TabPage) -> gui::Button {
    gui::Button::new(
        parent_window,
        gui::ButtonOpts {
            text: &t!("BUTTON_APPLY"),
            width: super::layout::BUTTON_WIDTH,
            height: super::layout::BUTTON_HEIGHT,
            ..Default::default()
        },
    )
}
