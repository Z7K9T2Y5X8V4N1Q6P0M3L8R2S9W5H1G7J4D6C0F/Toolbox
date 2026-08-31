use std::{cell::RefCell, rc::Rc};

use rust_i18n::t;
use winsafe::gui;

use crate::ui;

#[derive(Clone)]
pub struct MainWindow {
    pub(crate) main_window: gui::WindowMain,
    pub(crate) pending_error_message: Rc<RefCell<Option<String>>>,
    pub(crate) tab_container: ui::tab::container::TabContainer,
    pub(crate) status_bar: gui::StatusBar,
}

impl MainWindow {
    pub fn create_and_run() -> winsafe::AnyResult<i32> {
        super::init::initialize_application();

        let window_title = t!("TOOLBOX_TITLE");
        let main_window = gui::WindowMain::new(gui::WindowMainOpts {
            title: &window_title,
            style: winsafe::co::WS::OVERLAPPEDWINDOW,
            ..Default::default()
        });

        let status_bar = ui::statusbar::create_status_bar(&main_window);
        let tab_container = ui::tab::container::TabContainer::new(&main_window, status_bar.clone());

        let main_window_instance = Self {
            main_window,
            pending_error_message: Rc::new(RefCell::new(None)),
            tab_container,
            status_bar,
        };

        super::event::register_all_events(&main_window_instance)?;
        main_window_instance.main_window.run_main(None)
    }
}
