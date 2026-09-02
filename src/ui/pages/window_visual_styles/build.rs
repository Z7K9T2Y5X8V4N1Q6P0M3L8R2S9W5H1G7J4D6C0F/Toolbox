//! Window visual styles page control construction.
//!
//! Each function creates exactly one control. All controls are created during
//! [`WindowVisualStylesPage::new`] before the message loop starts — Win32 requires
//! controls to be created on the same thread as their parent window.

use winsafe::{co, gui, prelude::*};

/// Create the [`gui::TabPage`] that hosts all window visual styles controls.
pub(super) fn create_tab_page(parent_window: &(impl GuiParent + 'static)) -> gui::TabPage {
    gui::TabPage::new(
        parent_window,
        gui::TabPageOpts {
            class_style: co::CS::HREDRAW | co::CS::VREDRAW,
            ..Default::default()
        },
    )
}

/// Create a read-only decorative edit control that provides the outer border.
///
/// `WS_EX_CLIENTEDGE` gives it the standard sunken border appearance.
/// `ES_READONLY` and `WS_DISABLED` make it non-interactive and prevent the cursor.
pub(super) fn create_outer_edit(parent_window: &gui::TabPage) -> gui::Edit {
    gui::Edit::new(
        parent_window,
        gui::EditOpts {
            window_ex_style: co::WS_EX::CLIENTEDGE,
            window_style: co::WS::CHILD | co::WS::VISIBLE | co::WS::DISABLED,
            control_style: co::ES::READONLY,
            ..Default::default()
        },
    )
}

/// Create the real inner edit control for actual text input.
///
/// No border, with auto horizontal scroll and no hide selection.
/// This control is positioned inside the outer edit with padding.
pub(super) fn create_inner_edit(parent_window: &gui::TabPage) -> gui::Edit {
    gui::Edit::new(
        parent_window,
        gui::EditOpts {
            control_style: co::ES::AUTOHSCROLL | co::ES::NOHIDESEL,
            ..Default::default()
        },
    )
}
