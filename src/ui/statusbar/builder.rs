use winsafe::{gui, prelude::*};

pub fn create_status_bar(parent_window: &(impl GuiParent + 'static)) -> gui::StatusBar {
    gui::StatusBar::new(parent_window, &[gui::SbPart::Proportional(1)])
}
