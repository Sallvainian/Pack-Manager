//! Pack-Manager backend crate. Module layout per SPEC §5.1; builder wiring,
//! startup sequence (SPEC §5.12), and the quit-guard kill hook by U5.

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

use settings::Settings;
use tauri::Manager as _;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // SPEC §5.12: logging first (settings are loaded before so the filter can
    // honor the persisted level), then prune old logs.
    let settings_path = Settings::default_path();
    let loaded_settings = Settings::load_from(&settings_path);
    let logging_handle = logging::init(&loaded_settings);
    logging::prune_at_startup();
    tracing::info!(
        version = env!("CARGO_PKG_VERSION"),
        os = std::env::consts::OS,
        arch = std::env::consts::ARCH,
        "Pack-Manager starting"
    );

    let mut setup_state = Some((loaded_settings, settings_path, logging_handle));
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(move |app| {
            let (settings, path, logging_handle) = setup_state.take().expect("setup runs once");
            let handle = app.handle().clone();
            let app_state = tauri::async_runtime::block_on(state::AppState::initialize(
                handle,
                settings,
                path,
                Some(logging_handle),
            ));
            app.manage(app_state.clone());
            // ToolEnv probe + detection run async; the window shows
            // immediately and the frontend renders skeletons until
            // `detection:updated` lands.
            tauri::async_runtime::spawn(async move { app_state.startup().await });
            Ok(())
        })
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
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|app_handle, event| {
            // Quit-guard kill hook: on exit, cancel every running op so child
            // process groups are SIGTERMed and never outlive the app. The
            // confirm dialog lives in the frontend (QuitGuardDialog, U8).
            if let tauri::RunEvent::Exit = event {
                if let Some(state) = app_handle.try_state::<state::AppState>() {
                    state.shutdown();
                }
            }
        });
}
