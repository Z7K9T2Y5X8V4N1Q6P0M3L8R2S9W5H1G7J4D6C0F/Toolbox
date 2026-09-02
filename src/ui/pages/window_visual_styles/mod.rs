//! Window visual styles tab page.
//!
//! Currently exposes a single-line edit control for user input.
//! Future versions will add additional controls for customizing Win32 window
//! visual style properties.
//!
//! # Module Structure
//! - [`build`]  — constructs all controls (tab page, edit control)
//! - [`event`]  — registers all event handlers (resize, background paint)
//! - [`layout`] — DPI-aware layout constants and position calculators
//! - [`page`]   — [`WindowVisualStylesPage`] struct: public API

mod build;
mod event;
mod layout;
mod page;

pub use page::WindowVisualStylesPage;
