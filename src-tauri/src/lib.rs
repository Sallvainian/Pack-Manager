//! Pack-Manager backend crate. Module layout per SPEC §5.1; builder wiring,
//! startup sequence (SPEC §5.12), and the quit-guard kill hook by U5.

pub mod app_update;
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
use tauri::menu::{Menu, MenuItem, PredefinedMenuItem, Submenu};
use tauri::Manager as _;

/// Menu item id for the app menu's "Check for Updates…".
const MENU_CHECK_FOR_UPDATES: &str = "check_for_updates";

/// Set by `commands::install_app_update` immediately before `restart`, and read
/// back by the process that restart spawns — `Command::new` inherits the parent
/// environment, so this survives the hand-off. Its presence means "you are the
/// updated build, pull yourself to the front"; see the `RunEvent::Ready` arm.
///
/// Only ever set on the restart path, so a normal launch (already frontmost)
/// never calls `activateIgnoringOtherApps` and cannot steal focus.
pub const RELAUNCH_FOCUS_ENV: &str = "PM_FOCUS_AFTER_UPDATE";

/// Tauri's `Menu::default` has no room for a custom item, so the macOS menu is
/// rebuilt here. Everything except "Check for Updates…" mirrors that default —
/// notably the Edit submenu, without which ⌘X/⌘C/⌘V/⌘A stop working in the
/// package search field and every CopyableCommand.
fn build_menu(app: &tauri::AppHandle) -> tauri::Result<Menu<tauri::Wry>> {
    let pkg_info = app.package_info();
    let config = app.config();
    let about = tauri::menu::AboutMetadata {
        name: Some(pkg_info.name.clone()),
        version: Some(pkg_info.version.to_string()),
        copyright: config.bundle.copyright.clone(),
        authors: config.bundle.publisher.clone().map(|p| vec![p]),
        ..Default::default()
    };
    let check_for_updates = MenuItem::with_id(
        app,
        MENU_CHECK_FOR_UPDATES,
        "Check for Updates…",
        true,
        None::<&str>,
    )?;

    Menu::with_items(
        app,
        &[
            &Submenu::with_items(
                app,
                pkg_info.name.clone(),
                true,
                &[
                    &PredefinedMenuItem::about(app, None, Some(about))?,
                    &check_for_updates,
                    &PredefinedMenuItem::separator(app)?,
                    &PredefinedMenuItem::services(app, None)?,
                    &PredefinedMenuItem::separator(app)?,
                    &PredefinedMenuItem::hide(app, None)?,
                    &PredefinedMenuItem::hide_others(app, None)?,
                    &PredefinedMenuItem::separator(app)?,
                    &PredefinedMenuItem::quit(app, None)?,
                ],
            )?,
            &Submenu::with_items(
                app,
                "File",
                true,
                &[&PredefinedMenuItem::close_window(app, None)?],
            )?,
            &Submenu::with_items(
                app,
                "Edit",
                true,
                &[
                    &PredefinedMenuItem::undo(app, None)?,
                    &PredefinedMenuItem::redo(app, None)?,
                    &PredefinedMenuItem::separator(app)?,
                    &PredefinedMenuItem::cut(app, None)?,
                    &PredefinedMenuItem::copy(app, None)?,
                    &PredefinedMenuItem::paste(app, None)?,
                    &PredefinedMenuItem::select_all(app, None)?,
                ],
            )?,
            &Submenu::with_items(
                app,
                "View",
                true,
                &[&PredefinedMenuItem::fullscreen(app, None)?],
            )?,
            &Submenu::with_items(
                app,
                "Window",
                true,
                &[
                    &PredefinedMenuItem::minimize(app, None)?,
                    &PredefinedMenuItem::maximize(app, None)?,
                    &PredefinedMenuItem::separator(app)?,
                    &PredefinedMenuItem::close_window(app, None)?,
                ],
            )?,
        ],
    )
}

/// Launch check + a 6h heartbeat, both gated on `autoCheckForUpdates`. The
/// setting is re-read every tick so toggling it in Settings takes effect
/// without a restart.
fn spawn_auto_update_checks(app: tauri::AppHandle, state: state::AppState) {
    // A debug build points at the same endpoint as a release but can never
    // install over itself (`cargo run` has no .app bundle), so it would only
    // produce noise and pointless network traffic.
    if cfg!(debug_assertions) {
        tracing::debug!("debug build: automatic update checks disabled");
        return;
    }
    tauri::async_runtime::spawn(async move {
        let mut ticker = tokio::time::interval(app_update::AUTO_CHECK_INTERVAL);
        loop {
            ticker.tick().await;
            let enabled = state
                .settings
                .read()
                .expect("settings poisoned")
                .auto_check_for_updates;
            if !enabled {
                continue;
            }
            let source = app_update::TauriUpdateSource::new(app.clone());
            state
                .app_update
                .check_and_download(&source, ipc::UpdateCheckTrigger::Automatic)
                .await;
        }
    });
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // SPEC §5.12: logging first (settings are loaded before so the filter can
    // honor the persisted level), then prune old logs. Settings load runs
    // before the subscriber exists, so a corrupt file is re-logged after init
    // — a silent revert of every preference must be loud in the logs.
    let settings_path = Settings::default_path();
    let (loaded_settings, settings_corrupt) = Settings::load_from_reporting(&settings_path);
    let logging_handle = logging::init(&loaded_settings);
    logging::prune_at_startup();
    if let Some(detail) = settings_corrupt {
        tracing::warn!(
            path = %settings_path.display(),
            %detail,
            "settings.json was corrupt; defaults are in effect"
        );
    }
    tracing::info!(
        version = env!("CARGO_PKG_VERSION"),
        os = std::env::consts::OS,
        arch = std::env::consts::ARCH,
        "Pack-Manager starting"
    );

    let mut setup_state = Some((loaded_settings, settings_path, logging_handle));
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
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

            let handle = app.handle().clone();
            app.set_menu(build_menu(&handle)?)?;
            app.on_menu_event(move |app, event| {
                if event.id() != MENU_CHECK_FOR_UPDATES {
                    return;
                }
                let Some(state) = app.try_state::<state::AppState>() else {
                    return;
                };
                let updater = state.app_update.clone();
                let app = app.clone();
                tauri::async_runtime::spawn(async move {
                    let source = app_update::TauriUpdateSource::new(app);
                    updater
                        .check_and_download(&source, ipc::UpdateCheckTrigger::Manual)
                        .await;
                });
            });
            spawn_auto_update_checks(handle, app_state.clone());

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
            commands::get_app_update_state,
            commands::check_for_app_update,
            commands::install_app_update,
        ])
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|app_handle, event| match event {
            // Pull an updated build to the front. `AppHandle::restart` bare-
            // spawns the new binary (tauri `process.rs`) rather than going
            // through LaunchServices, so the replacement inherits no
            // activation token and macOS opens it *behind* every other window.
            // `Ready` is the earliest point that works: it fires inside the
            // event loop, once NSApp has finished launching, so tao's
            // `activateIgnoringOtherApps` actually takes. Called from `setup`
            // it is silently dropped.
            tauri::RunEvent::Ready => {
                if std::env::var_os(RELAUNCH_FOCUS_ENV).is_some() {
                    // One-shot: a later self-restart must opt in again, and
                    // nothing downstream should inherit it.
                    std::env::remove_var(RELAUNCH_FOCUS_ENV);
                    if let Some(window) = app_handle.get_webview_window("main") {
                        if let Err(error) = window.set_focus() {
                            // Losing focus is a papercut, never a reason to
                            // fail the launch we just performed.
                            tracing::warn!(%error, "could not focus the updated build");
                        }
                    }
                }
            }
            // Quit-guard kill hook: on exit, cancel every running op so child
            // process groups are SIGTERMed and never outlive the app. The
            // confirm dialog lives in the frontend (QuitGuardDialog, U8).
            tauri::RunEvent::Exit => {
                if let Some(state) = app_handle.try_state::<state::AppState>() {
                    state.shutdown();
                }
            }
            _ => {}
        });
}
