//! Managed application state — completed by U5 (ToolEnv, registry, scheduler
//! handle, settings, journal). U1 provides a compiling placeholder so
//! `lib.rs` and `commands.rs` have a stable name to wire against.

/// Placeholder managed state. U5 replaces the contents (never the name).
#[derive(Debug, Default)]
pub struct AppState {}
