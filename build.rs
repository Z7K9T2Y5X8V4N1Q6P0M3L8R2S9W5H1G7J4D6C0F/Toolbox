//! Build script for Windows application manifest embedding.
//!
//! Configures and embeds an application manifest targeting Windows platforms,
//! enabling UTF-8 code page support, long path awareness, segment heap, and
//! specifying minimum supported OS versions.

use embed_manifest::manifest::{ActiveCodePage, ExecutionLevel, HeapType, Setting, SupportedOS};
use embed_manifest::{embed_manifest, new_manifest};

fn main() {
    if std::env::var_os("CARGO_CFG_WINDOWS").is_some() {
        embed_windows_application_manifest();
    }

    println!("cargo:rerun-if-changed=build.rs");
}

/// Generates and embeds the application manifest into the Windows binary.
fn embed_windows_application_manifest() {
    let manifest = new_manifest(env!("CARGO_PKG_NAME"))
        .supported_os(SupportedOS::Windows10..)
        .active_code_page(ActiveCodePage::Utf8)
        .requested_execution_level(ExecutionLevel::AsInvoker)
        .long_path_aware(Setting::Enabled)
        .heap_type(HeapType::SegmentHeap);

    embed_manifest(manifest).expect("unable to embed manifest file");
}
