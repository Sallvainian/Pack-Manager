//! Pack-Manager backend crate. Module layout per SPEC §5.1; U5 completes the
//! builder wiring (managed state, startup sequence §5.12, quit-guard kill
//! hook).

pub mod commands;
pub mod detect;
pub mod diagnostics;
pub mod error;
pub mod events;
pub mod ipc;
pub mod journal;
pub mod logging;
pub mod managers;
pub mod ops;
pub mod paths;
pub mod process;
pub mod queue;
pub mod registry;
pub mod settings;
pub mod state;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(state::AppState::default())
        .invoke_handler(tauri::generate_handler![
            commands::detect_managers,
            commands::get_state,
            commands::refresh_manager,
            commands::refresh_all,
            commands::build_upgrade_plan,
            commands::execute_plan,
            commands::self_update_manager,
            commands::run_health_fix,
            commands::cancel_operation,
            commands::get_operation,
            commands::list_operations,
            commands::get_settings,
            commands::set_settings,
            commands::reveal_operation_log,
            commands::reveal_logs_dir,
            commands::export_diagnostics,
            commands::log_frontend_event,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
