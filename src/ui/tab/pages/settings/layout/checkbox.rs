use winsafe::{POINT, gui};

use super::constants::{
    CHECKBOX_BOTTOM_MARGIN, CHECKBOX_HEIGHT, CHECKBOX_LEFT_MARGIN, CHECKBOX_RIGHT_MARGIN,
    CHECKBOX_TOP_MARGIN, CHECKBOX_VERTICAL_GAP,
};

pub(in crate::ui::tab::pages::settings) struct CheckboxLayoutCalculator {
    checkbox_height: i32,
    checkbox_vertical_gap: i32,
    checkbox_left_margin: i32,
    checkbox_right_margin: i32,
    checkbox_top_margin: i32,
    checkbox_bottom_margin: i32,
}

impl CheckboxLayoutCalculator {
    pub fn new() -> Self {
        Self {
            checkbox_height: gui::dpi_y(CHECKBOX_HEIGHT),
            checkbox_vertical_gap: gui::dpi_y(CHECKBOX_VERTICAL_GAP),
            checkbox_left_margin: gui::dpi_x(CHECKBOX_LEFT_MARGIN),
            checkbox_right_margin: gui::dpi_x(CHECKBOX_RIGHT_MARGIN),
            checkbox_top_margin: gui::dpi_y(CHECKBOX_TOP_MARGIN),
            checkbox_bottom_margin: gui::dpi_y(CHECKBOX_BOTTOM_MARGIN),
        }
    }

    pub fn calculate_checkbox_position(&self, index: usize) -> POINT {
        POINT {
            x: self.checkbox_left_margin,
            y: self.checkbox_top_margin
                + (index as i32) * (self.checkbox_height + self.checkbox_vertical_gap),
        }
    }

    pub fn calculate_checkbox_width(&self, content_panel_width: i32) -> i32 {
        (content_panel_width - self.checkbox_left_margin - self.checkbox_right_margin).max(100)
    }

    pub fn checkbox_height(&self) -> i32 {
        self.checkbox_height
    }

    pub fn calculate_total_content_height(&self, checkbox_count: usize) -> i32 {
        if checkbox_count == 0 {
            return 0;
        }

        self.checkbox_top_margin
            + (checkbox_count as i32) * (self.checkbox_height + self.checkbox_vertical_gap)
            - self.checkbox_vertical_gap
            + self.checkbox_bottom_margin
    }
}
