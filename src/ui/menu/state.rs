//! Menu command ID constants.
//!
//! Each constant is a unique `u16` sent via `WM_COMMAND` when the user
//! clicks the corresponding menu item. Ranges are grouped by submenu:
//! - `1000–1999` — Options submenu
//! - `2000–2999` — Language submenu

/// Trigger an Explorer process restart.
pub const IDM_OPTIONS_RESTART_EXPLORER: u16 = 1001;

/// Switch the application language to English (United States).
pub const IDM_LANG_EN_US: u16 = 2001;

/// Switch the application language to Simplified Chinese.
pub const IDM_LANG_ZH_CN: u16 = 2002;
