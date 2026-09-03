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

/// Create a standard single-line edit control for user input
///
/// `WS_EX_CLIENTEDGE` provides the standard sunken border.
/// `ES_AUTOHSCROLL` enables horizontal scrolling when text exceeds the visible width.
pub(super) fn create_edit(parent_window: &gui::TabPage) -> gui::Edit {
    gui::Edit::new(
        parent_window,
        gui::EditOpts {
            window_ex_style: co::WS_EX::CLIENTEDGE,
            control_style: co::ES::AUTOHSCROLL,
            ..Default::default()
        },
    )
}
