//! Supported application languages.

use serde::{Deserialize, Serialize};

/// A supported display language for the application.
///
/// Serialized as a BCP 47 locale string (e.g. `"zh-CN"`) in the config file.
/// The default is [`AppLanguage::EnUs`].
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum AppLanguage {
    #[serde(rename = "zh-CN")]
    ZhCn,
    #[serde(rename = "en-US")]
    EnUs,
}

impl Default for AppLanguage {
    fn default() -> Self {
        Self::EnUs
    }
}

impl AppLanguage {
    /// Returns the BCP 47 locale string for this language.
    ///
    /// The returned value is suitable for passing directly to
    /// [`rust_i18n::set_locale`].
    pub fn as_locale_str(&self) -> &'static str {
        match self {
            AppLanguage::ZhCn => "zh-CN",
            AppLanguage::EnUs => "en-US",
        }
    }
}
