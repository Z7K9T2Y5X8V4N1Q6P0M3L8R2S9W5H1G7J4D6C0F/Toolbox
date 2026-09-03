//! Window visual styles page control construction.
//!
//! Each function creates exactly one control. All controls are created during
//! [`WindowVisualStylesPage::new`] before the message loop starts — Win32 requires
//! controls to be created on the same thread as their parent window.

use winsafe::{co, gui, prelude::*};

/// Create the [`gui::TabPage`] that hosts all window visual styles controls
pub(super) fn create_tab_page(parent_window: &(impl GuiParent + 'static)) -> gui::TabPage {
    gui::TabPage::new(
        parent_window,
        gui::TabPageOpts {
            class_style: co::CS::HREDRAW | co::CS::VREDRAW,
            ..Default::default()
        },
    )
}

/// Create an editable combobox for user input with a dropdown list
///
/// `CBS_DROPDOWN` creates a combobox where the edit field is always visible
/// and the user can type directly or select from the dropdown list.
pub(super) fn create_combobox(parent_window: &gui::TabPage) -> gui::ComboBox {
    gui::ComboBox::new(
        parent_window,
        gui::ComboBoxOpts {
            control_style: co::CBS::DROPDOWN | co::CBS::AUTOHSCROLL,
            ..Default::default()
        },
    )
}
