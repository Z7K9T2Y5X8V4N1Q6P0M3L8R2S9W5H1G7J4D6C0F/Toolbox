//! Application configuration: language preference and persistence.
//!
//! # Module Structure
//! - [`language`]    — [`AppLanguage`] enum representing supported locales
//! - [`persistence`] — [`AppConfig`] struct with load/save logic

mod language;
mod persistence;

pub use language::AppLanguage;
pub use persistence::AppConfig;
