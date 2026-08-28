use rust_i18n::t;
use winsafe::{co, gui, prelude::*};

pub(in crate::ui::tab::pages::settings) fn create_tab_page(
    parent_window: &(impl GuiParent + 'static),
) -> gui::TabPage {
    gui::TabPage::new(
        parent_window,
        gui::TabPageOpts {
            class_style: co::CS::HREDRAW | co::CS::VREDRAW,
            ..Default::default()
        },
    )
}

pub(in crate::ui::tab::pages::settings) fn create_group_box(
    parent_window: &gui::TabPage,
) -> gui::Button {
    gui::Button::new(
        parent_window,
        gui::ButtonOpts {
            text: &t!("GROUP_BOX_SETTINGS_TITLE"),
            control_style: co::BS::GROUPBOX,
            ..Default::default()
        },
    )
}

pub(in crate::ui::tab::pages::settings) fn create_scrollable_panel(
    parent_window: &gui::TabPage,
) -> gui::WindowControl {
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

pub(in crate::ui::tab::pages::settings) fn create_content_panel(
    parent_window: &gui::WindowControl,
) -> gui::WindowControl {
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
