use crate::config::{AppConfig, AppLanguage};
use crate::{error, ui};

pub fn initialize_application() {
    setup_initial_locale();
    ui::window::hook::install_ui_customization_hook();
    error::panic::install_panic_hook();
    setup_config_locale();
}

/// Set the locale to the system default before config is loaded.
/// This ensures early UI elements (e.g. config error dialogs) are localized.
fn setup_initial_locale() {
    let system_locale =
        sys_locale::get_locale().unwrap_or_else(|| AppLanguage::EnUs.as_locale_str().to_string());
    rust_i18n::set_locale(&system_locale);
}

/// Override the locale with the user's saved preference from config.
fn setup_config_locale() {
    let app_config = AppConfig::load();
    rust_i18n::set_locale(&app_config.language.as_locale_str());
}
