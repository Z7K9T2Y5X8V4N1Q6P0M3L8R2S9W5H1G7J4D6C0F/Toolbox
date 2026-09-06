//! System process information retrieval and sorting.
//!
//! Owns the [`sysinfo::System`] instance and provides utilities to fetch and
//! format the current list of running processes for display in the UI.

use sysinfo::{ProcessRefreshKind, ProcessesToUpdate, System};

/// A snapshot of a single running system process.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProcessItem {
    pub process_id: u32,
    pub process_name: String,
}

/// Manages system process querying with persistent state for incremental updates.
pub struct ProcessManager {
    system_monitor: System,
}

impl ProcessManager {
    /// Create a new process manager with an initialized system monitor.
    pub fn new() -> Self {
        Self {
            system_monitor: System::new(),
        }
    }

    /// Refresh the current process list and return the sorted snapshots.
    ///
    /// Sorted primarily by process name alphabetically (case-insensitive A-Z),
    /// and secondarily by process ID ascending.
    pub fn fetch_sorted_processes(&mut self) -> Vec<ProcessItem> {
        self.system_monitor.refresh_processes_specifics(
            ProcessesToUpdate::All,
            true,
            ProcessRefreshKind::nothing(),
        );

        let mut process_list: Vec<ProcessItem> = self
            .system_monitor
            .processes()
            .iter()
            .map(|(pid, process)| {
                let process_id = pid.as_u32();
                let process_name = process.name().to_string_lossy().to_string();
                ProcessItem {
                    process_id,
                    process_name,
                }
            })
            .collect();

        process_list.sort_by(|first_process, second_process| {
            let name_comparison = first_process
                .process_name
                .to_lowercase()
                .cmp(&second_process.process_name.to_lowercase());

            if name_comparison == std::cmp::Ordering::Equal {
                first_process.process_id.cmp(&second_process.process_id)
            } else {
                name_comparison
            }
        });

        process_list
    }
}
