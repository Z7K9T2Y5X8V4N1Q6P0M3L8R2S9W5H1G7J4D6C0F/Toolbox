//! Window visual styles tab page.
//!
//! Presents a combobox for user input and a ListView control for displaying
//! and managing window visual style configurations.
//!
//! # Layout
//! ```text
//! ┌─ tab page ───────────────────────────────┐
//! │  ┌─ combobox ─────────────────────────┐  │
//! │  │ [input field              ▼]       │  │
//! │  └────────────────────────────────────┘  │
//! │                                           │
//! │  ┌─ listview ─────────────────────────┐  │
//! │  │ Column1    Column2    Column3      │  │
//! │  ├────────────────────────────────────┤  │
//! │  │ Item 1     Data       Data         │  │
//! │  │ Item 2     Data       Data         │  │
//! │  │ ...                                │  │
//! │  └────────────────────────────────────┘  │
//! └──────────────────────────────────────────┘
//! ```
//!
//! # Module Structure
//! - [`build`]  — constructs all controls (tab page, combobox, listview)
//! - [`event`]  — registers all event handlers (resize, background paint)
//! - [`layout`] — DPI-aware layout constants and position calculators
//! - [`page`]   — [`WindowVisualStylesPage`] struct: public API for construction and text updates

mod build;
mod event;
mod layout;
mod page;

pub use page::WindowVisualStylesPage;
