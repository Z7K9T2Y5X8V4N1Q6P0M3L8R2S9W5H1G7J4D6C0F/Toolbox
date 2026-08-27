use winsafe::{HwndPlace, POINT, SCROLLINFO, SIZE, co, gui, prelude::*};

use crate::ui;
use crate::ui::tab::pages::settings::builder::checkboxes::CheckboxId;
use crate::ui::tab::pages::settings::layout::{CheckboxLayoutCalculator, SettingsPageLayout};

pub(super) fn setup_resize_event(
    tab_page: &gui::TabPage,
    group_box: &gui::Button,
    scrollable_panel: &gui::WindowControl,
    content_panel: &gui::WindowControl,
    checkboxes: &[(CheckboxId, gui::Button)],
    button_select_all_toggle: &gui::Button,
    button_apply: &gui::Button,
) {
    let cloned_group_box = group_box.clone();
    let cloned_scrollable_panel = scrollable_panel.clone();
    let cloned_content_panel = content_panel.clone();
    let cloned_checkboxes: Vec<gui::Button> = checkboxes
        .iter()
        .map(|(_, checkbox)| checkbox.clone())
        .collect();
    let cloned_button_select_all_toggle = button_select_all_toggle.clone();
    let cloned_button_apply = button_apply.clone();

    tab_page.on().wm_size(move |size_info| {
        ui::tab::utils::bring_control_to_top(cloned_scrollable_panel.hwnd())?;

        let settings_page_layout =
            SettingsPageLayout::calculate(size_info.client_area.cx, size_info.client_area.cy);

        ui::tab::utils::reposition_control(
            cloned_group_box.hwnd(),
            settings_page_layout.group_box_position,
            settings_page_layout.group_box_size,
        )?;

        ui::tab::utils::reposition_control(
            cloned_scrollable_panel.hwnd(),
            settings_page_layout.scrollable_panel_position,
            settings_page_layout.scrollable_panel_size,
        )?;

        let layout_calculator = CheckboxLayoutCalculator::new();
        let content_panel_width = settings_page_layout.scrollable_panel_size.cx;
        let content_panel_total_height =
            layout_calculator.calculate_total_content_height(cloned_checkboxes.len());
        let scrollable_panel_visible_height = settings_page_layout.scrollable_panel_size.cy;

        ui::tab::utils::reposition_control(
            cloned_content_panel.hwnd(),
            POINT { x: 0, y: 0 },
            SIZE {
                cx: content_panel_width,
                cy: content_panel_total_height,
            },
        )?;

        let checkbox_width = layout_calculator.calculate_checkbox_width(content_panel_width);
        let checkbox_height = layout_calculator.checkbox_height();
        for (index, checkbox_button) in cloned_checkboxes.iter().enumerate() {
            let checkbox_position = layout_calculator.calculate_checkbox_position(index);
            checkbox_button.hwnd().SetWindowPos(
                HwndPlace::None,
                checkbox_position,
                SIZE {
                    cx: checkbox_width,
                    cy: checkbox_height,
                },
                co::SWP::NOZORDER | co::SWP::NOCOPYBITS,
            )?;
        }

        update_scrollbar_range(
            &cloned_scrollable_panel,
            scrollable_panel_visible_height,
            content_panel_total_height,
        );

        ui::tab::utils::reposition_control(
            cloned_button_apply.hwnd(),
            settings_page_layout.button_apply_position,
            settings_page_layout.button_apply_size,
        )?;

        ui::tab::utils::reposition_control(
            cloned_button_select_all_toggle.hwnd(),
            settings_page_layout.button_select_all_toggle_position,
            settings_page_layout.button_select_all_toggle_size,
        )?;

        Ok(())
    });
}

fn update_scrollbar_range(
    scrollable_panel: &gui::WindowControl,
    scrollable_panel_visible_height: i32,
    content_panel_total_height: i32,
) {
    if content_panel_total_height <= scrollable_panel_visible_height {
        scrollable_panel
            .hwnd()
            .ShowScrollBar(co::SBB::VERT, false)
            .ok();
    } else {
        scrollable_panel
            .hwnd()
            .ShowScrollBar(co::SBB::VERT, true)
            .ok();

        let mut scroll_info = SCROLLINFO::default();
        scroll_info.fMask = co::SIF::RANGE | co::SIF::PAGE | co::SIF::POS;
        scroll_info.nMin = 0;
        scroll_info.nMax = content_panel_total_height - 1;
        scroll_info.nPage = scrollable_panel_visible_height as u32;
        scroll_info.nPos = 0;

        scrollable_panel
            .hwnd()
            .SetScrollInfo(co::SBB::VERT, &scroll_info, true);
    }
}
