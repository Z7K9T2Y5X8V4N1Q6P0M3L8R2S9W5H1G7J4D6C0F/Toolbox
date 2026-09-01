//! Tab control container, construction, and layout.
//!
//! This module manages the Win32 tab control widget and the pages it hosts.
//! It does not own any page content — that lives in [`crate::ui::pages`].
//!
//! # Module Structure
//! - [`build`]     — constructs the [`gui::Tab`] control and generates tab titles
//! - [`container`] — [`TabContainer`] struct that owns the tab control and all pages
//! - [`layout`]    — DPI-aware resize, positioning, and background painting utilities

mod build;
pub mod container;
pub(crate) mod layout;
