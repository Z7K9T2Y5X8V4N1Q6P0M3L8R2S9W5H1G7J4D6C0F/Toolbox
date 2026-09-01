//! Application entry point and main window orchestration.
//!
//! This module owns the [`MainWindow`] struct and is responsible for:
//! - Initializing the application environment (locale, hooks, panic handler)
//! - Creating the main Win32 window and all top-level UI components
//! - Registering window-level event handlers
//!
//! # Module Structure
//! - [`build`] — constructs [`MainWindow`] and runs the message loop
//! - [`event`] — registers all Win32 window message handlers
//! - [`init`]  — sets up locale, UI hook, and panic hook before the window is created

mod build;
mod event;
mod init;

pub use build::MainWindow;
