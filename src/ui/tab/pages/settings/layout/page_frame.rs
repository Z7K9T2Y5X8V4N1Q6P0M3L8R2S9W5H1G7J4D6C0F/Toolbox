use winsafe::{POINT, SIZE, gui};

use super::constants::{
    BUTTON_HEIGHT, BUTTON_HORIZONTAL_GAP, BUTTON_WIDTH, GROUP_BOX_INTERNAL_PADDING,
    GROUP_BOX_MARGIN, GROUP_BOX_TITLE_BAR_HEIGHT,
};

pub(in crate::ui::tab::pages::settings) struct SettingsPageLayout {
    pub group_box_position: POINT,
    pub group_box_size: SIZE,
    pub scrollable_panel_position: POINT,
    pub scrollable_panel_size: SIZE,
    pub button_apply_position: POINT,
    pub button_apply_size: SIZE,
    pub button_select_all_toggle_position: POINT,
    pub button_select_all_toggle_size: SIZE,
}

impl SettingsPageLayout {
    pub fn calculate(tab_page_client_width: i32, tab_page_client_height: i32) -> Self {
        let group_box_margin = gui::dpi_x(GROUP_BOX_MARGIN);
        let button_width = gui::dpi_x(BUTTON_WIDTH);
        let button_height = gui::dpi_y(BUTTON_HEIGHT);
        let button_horizontal_gap = gui::dpi_x(BUTTON_HORIZONTAL_GAP);
        let group_box_title_bar_height = gui::dpi_y(GROUP_BOX_TITLE_BAR_HEIGHT);
        let group_box_internal_padding = gui::dpi_x(GROUP_BOX_INTERNAL_PADDING);

        let group_box_width = tab_page_client_width - 2 * group_box_margin;
        let group_box_height =
            tab_page_client_height - (2 * group_box_margin) - button_height - group_box_margin;

        let scrollable_panel_position = POINT {
            x: group_box_margin + group_box_internal_padding,
            y: group_box_margin + group_box_title_bar_height,
        };
        let scrollable_panel_size = SIZE {
            cx: group_box_width - 2 * group_box_internal_padding,
            cy: group_box_height - group_box_title_bar_height - group_box_internal_padding,
        };

        let button_vertical_position = group_box_margin + group_box_height + group_box_margin;
        let button_apply_horizontal_position =
            tab_page_client_width - group_box_margin - button_width;
        let button_select_all_toggle_horizontal_position =
            button_apply_horizontal_position - button_horizontal_gap - button_width;

        Self {
            group_box_position: POINT {
                x: group_box_margin,
                y: group_box_margin,
            },
            group_box_size: SIZE {
                cx: group_box_width,
                cy: group_box_height,
            },
            scrollable_panel_position,
            scrollable_panel_size,
            button_apply_position: POINT {
                x: button_apply_horizontal_position,
                y: button_vertical_position,
            },
            button_apply_size: SIZE {
                cx: button_width,
                cy: button_height,
            },
            button_select_all_toggle_position: POINT {
                x: button_select_all_toggle_horizontal_position,
                y: button_vertical_position,
            },
            button_select_all_toggle_size: SIZE {
                cx: button_width,
                cy: button_height,
            },
        }
    }
}
