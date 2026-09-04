//! Window visual styles tab page.
//!
//! Presents a single-line edit control for user input and a ListView control
//! for displaying and managing window visual style configurations.
//!
//! # Layout
//! ```text
//! ┌─ tab page ───────────────────────────────┐
//! │  ┌─ edit ─────────────────────────────┐  │
//! │  │ [input field                     ] │  │
//! │  └────────────────────────────────────┘  │
//! │                                          │
//! │  ┌─ listview ─────────────────────────┐  │
//! │  │ Column 1             Column 2      │  │
//! │  ├────────────────────────────────────┤  │
//! │  │ Item 1               Data          │  │
//! │  │ Item 2               Data          │  │
//! │  │ ...                                │  │
//! │  └────────────────────────────────────┘  │
//! └──────────────────────────────────────────┘
//! ```
//!
//! # Module Structure
//! - [`build`]  — constructs all controls (tab page, edit, listview)
//! - [`event`]  — registers all event handlers (resize, cue banner, background paint)
//! - [`layout`] — DPI-aware layout constants, dynamic font metrics height calculation, and position calculators
//! - [`page`]   — [`WindowVisualStylesPage`] struct: public API for construction and text updates

mod build;
mod event;
mod layout;
mod page;

pub use page::WindowVisualStylesPage;
