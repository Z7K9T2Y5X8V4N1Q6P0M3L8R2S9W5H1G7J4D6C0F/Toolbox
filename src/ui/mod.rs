//! All user interface components.
//!
//! # Module Structure
//! - [`menu`]      — main menu bar construction and language/options event handlers
//! - [`statusbar`] — status bar construction and checkbox hover description events
//! - [`tab`]       — tab control container, layout, and resize logic
//! - [`pages`]     — individual tab page implementations
//! - [`window`]    — main window layout utilities and UI customization hook
pub mod menu;
pub mod pages;
pub mod statusbar;
pub mod tab;
pub mod window;
