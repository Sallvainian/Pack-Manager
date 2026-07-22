//! In-app update: Pack-Manager updating *itself* (DECISIONS D25).
//!
//! This is deliberately separate from `ops.rs`/`queue.rs`, which only ever
//! describe package-manager operations — an app self-update is not an
//! `Operation`, holds no manager lock, and never appears in History.
//!
//! Flow: `check` → (found) download automatically, streaming progress → park in
//! `ReadyToInstall` → the user clicks Restart → `install` → `AppHandle::restart`.
//! Installing is never automatic; that click is the gate (SPEC §1 "No
//! auto-upgrades without user action").
//!
//! The `UpdateSource`/`PendingRelease` traits are the nondeterminism seam, in
//! the same spirit as `CommandRunner`/`FakeRunner` (SPEC §7): the real
//! implementation wraps `tauri_plugin_updater`, tests use [`FakeUpdateSource`]
//! and drive the whole state machine offline.

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, RwLock};

use async_trait::async_trait;

use crate::events::{AppEvent, EventSink};
use crate::ipc::{AppUpdateState, AppUpdateStatus, UpdateCheckTrigger};

/// How often the background task re-checks while the app stays open.
pub const AUTO_CHECK_INTERVAL: std::time::Duration = std::time::Duration::from_secs(6 * 60 * 60);

/// Download progress callback: `(chunk_len, content_length)`.
pub type ChunkFn = Box<dyn FnMut(usize, Option<u64>) + Send>;

/// Outcome of [`UpdateSource::check`]: `Ok(None)` = already latest.
pub type CheckResult = Result<Option<Arc<dyn PendingRelease>>, String>;

/// A downloaded, signature-verified release paired with its archive bytes.
type Downloaded = (Arc<dyn PendingRelease>, Vec<u8>);

/// The updater endpoint. One implementation talks to `tauri_plugin_updater`;
/// [`FakeUpdateSource`] is the test double.
#[async_trait]
pub trait UpdateSource: Send + Sync {
    /// `Ok(None)` means "already on the latest version".
    async fn check(&self) -> CheckResult;
}

/// A release newer than the running build, not yet downloaded.
#[async_trait]
pub trait PendingRelease: Send + Sync {
    fn version(&self) -> String;
    fn notes(&self) -> Option<String>;
    /// Downloads and signature-verifies the archive.
    async fn download(&self, on_chunk: ChunkFn) -> Result<Vec<u8>, String>;
    /// Replaces the installed bundle. Caller restarts afterwards.
    fn install(&self, bytes: &[u8]) -> Result<(), String>;
}

/// Owns the update state machine and broadcasts every transition.
pub struct AppUpdater {
    current_version: String,
    sink: Arc<dyn EventSink>,
    /// `Arc` because the per-chunk progress callback handed to `download` must
    /// be `'static`, so it cannot borrow from `self`.
    status: Arc<RwLock<AppUpdateStatus>>,
    /// Set once a download completes; consumed by [`AppUpdater::install`].
    downloaded: Mutex<Option<Downloaded>>,
    /// Guards against a menu click racing the 6h timer into two checks.
    busy: AtomicBool,
}

impl AppUpdater {
    pub fn new(current_version: impl Into<String>, sink: Arc<dyn EventSink>) -> Self {
        let current_version = current_version.into();
        Self {
            status: Arc::new(RwLock::new(AppUpdateStatus {
                current_version: current_version.clone(),
                state: AppUpdateState::Idle,
                last_trigger: None,
            })),
            current_version,
            sink,
            downloaded: Mutex::new(None),
            busy: AtomicBool::new(false),
        }
    }

    pub fn status(&self) -> AppUpdateStatus {
        self.status
            .read()
            .expect("app update status poisoned")
            .clone()
    }

    fn set(&self, state: AppUpdateState, trigger: UpdateCheckTrigger) {
        let status = {
            let mut guard = self.status.write().expect("app update status poisoned");
            guard.state = state;
            guard.last_trigger = Some(trigger);
            guard.clone()
        };
        self.sink.emit(AppEvent::AppUpdateStatus(status));
    }

    /// Checks, and on a hit downloads immediately. Returns without doing
    /// anything when a check is already in flight — the second caller would
    /// only fight the first over `downloaded`.
    pub async fn check_and_download(&self, source: &dyn UpdateSource, trigger: UpdateCheckTrigger) {
        if self.busy.swap(true, Ordering::SeqCst) {
            tracing::debug!("app update check already in flight; ignoring");
            return;
        }
        self.run_check(source, trigger).await;
        self.busy.store(false, Ordering::SeqCst);
    }

    async fn run_check(&self, source: &dyn UpdateSource, trigger: UpdateCheckTrigger) {
        self.set(AppUpdateState::Checking, trigger);

        let release = match source.check().await {
            Ok(Some(release)) => release,
            Ok(None) => {
                tracing::info!(version = %self.current_version, "app is up to date");
                self.set(AppUpdateState::UpToDate, trigger);
                return;
            }
            Err(message) => {
                // Automatic checks fire on a timer and on every launch; a
                // laptop that is simply offline must not produce an error every
                // six hours, so this is WARN, and the frontend only toasts for
                // manual checks.
                tracing::warn!(%message, ?trigger, "app update check failed");
                self.set(AppUpdateState::Error { message }, trigger);
                return;
            }
        };

        let version = release.version();
        let notes = release.notes();
        tracing::info!(from = %self.current_version, to = %version, "app update found; downloading");
        self.set(
            AppUpdateState::Downloading {
                version: version.clone(),
                received: 0,
                total: None,
            },
            trigger,
        );

        // `on_chunk` is called on the download task; it emits an event per
        // chunk, which the frontend renders as a percentage.
        let progress = {
            let downloading_version = version.clone();
            let status = self.status.clone();
            let sink = self.sink.clone();
            let mut received: u64 = 0;
            move |chunk_len: usize, total: Option<u64>| {
                received += chunk_len as u64;
                let payload = {
                    let mut guard = status.write().expect("app update status poisoned");
                    guard.state = AppUpdateState::Downloading {
                        version: downloading_version.clone(),
                        received,
                        total,
                    };
                    guard.clone()
                };
                sink.emit(AppEvent::AppUpdateStatus(payload));
            }
        };

        match release.download(Box::new(progress)).await {
            Ok(bytes) => {
                tracing::info!(version = %version, bytes = bytes.len(), "app update downloaded");
                *self.downloaded.lock().expect("downloaded poisoned") =
                    Some((release.clone(), bytes));

                // Stop before the plugin's admin-password fallback: on macOS it
                // shells out to AppleScript `with administrator privileges`
                // when the bundle's directory is not writable, and this app
                // promises never to ask for a password.
                if let Err(reason) = install_target_writable() {
                    tracing::warn!(%reason, "app update downloaded but cannot be installed in place");
                    self.set(
                        AppUpdateState::ManualInstallRequired { version, reason },
                        trigger,
                    );
                    return;
                }
                self.set(AppUpdateState::ReadyToInstall { version, notes }, trigger);
            }
            Err(message) => {
                tracing::error!(%message, version = %version, "app update download failed");
                self.set(AppUpdateState::Error { message }, trigger);
            }
        }
    }

    /// Installs the downloaded archive over the running bundle. The caller
    /// restarts the app on `Ok(())` — this returns only on failure or once the
    /// bundle has been replaced.
    pub fn install(&self) -> Result<(), String> {
        let downloaded = self
            .downloaded
            .lock()
            .expect("downloaded poisoned")
            .clone()
            .ok_or_else(|| "no update has been downloaded".to_string())?;
        let (release, bytes) = downloaded;
        let version = release.version();
        tracing::info!(version = %version, "installing app update");
        match release.install(&bytes) {
            Ok(()) => Ok(()),
            Err(message) => {
                tracing::error!(%message, version = %version, "app update install failed");
                let trigger = self
                    .status()
                    .last_trigger
                    .unwrap_or(UpdateCheckTrigger::Manual);
                self.set(
                    AppUpdateState::Error {
                        message: message.clone(),
                    },
                    trigger,
                );
                Err(message)
            }
        }
    }
}

/// True when the running bundle can be replaced without elevated privileges.
///
/// The plugin renames the `.app` out of the way and moves the new one in, so
/// what matters is write access to the bundle's *parent* directory (usually
/// `/Applications`), not the bundle itself.
fn install_target_writable() -> Result<(), String> {
    let exe = std::env::current_exe().map_err(|e| format!("cannot locate the running app: {e}"))?;
    let bundle = bundle_root(&exe);
    let parent = bundle
        .parent()
        .ok_or_else(|| "the running app has no parent directory".to_string())?;
    nix::unistd::access(parent, nix::unistd::AccessFlags::W_OK).map_err(|_| {
        format!(
            "{} is not writable — move Pack-Manager to /Applications (or your home folder) and try again",
            parent.display()
        )
    })
}

/// `…/Pack-Manager.app/Contents/MacOS/pack-manager` → `…/Pack-Manager.app`.
/// Falls back to the executable's own directory outside a bundle (`cargo run`).
fn bundle_root(exe: &Path) -> PathBuf {
    for ancestor in exe.ancestors() {
        if ancestor.extension().is_some_and(|e| e == "app") {
            return ancestor.to_path_buf();
        }
    }
    exe.parent().unwrap_or(exe).to_path_buf()
}

// ---------------------------------------------------------------------------
// Tauri-backed source (the only part that knows about the updater plugin),
// mirroring how `events.rs` keeps `EventSink` pure and `TauriSink` concrete.
// ---------------------------------------------------------------------------

pub struct TauriUpdateSource {
    handle: tauri::AppHandle,
}

impl TauriUpdateSource {
    pub fn new(handle: tauri::AppHandle) -> Self {
        Self { handle }
    }
}

#[async_trait]
impl UpdateSource for TauriUpdateSource {
    async fn check(&self) -> CheckResult {
        use tauri_plugin_updater::UpdaterExt as _;
        let updater = self.handle.updater().map_err(|e| e.to_string())?;
        match updater.check().await {
            Ok(Some(update)) => Ok(Some(Arc::new(TauriRelease { update }))),
            Ok(None) => Ok(None),
            Err(e) => Err(e.to_string()),
        }
    }
}

struct TauriRelease {
    update: tauri_plugin_updater::Update,
}

#[async_trait]
impl PendingRelease for TauriRelease {
    fn version(&self) -> String {
        self.update.version.clone()
    }

    fn notes(&self) -> Option<String> {
        self.update.body.clone()
    }

    async fn download(&self, mut on_chunk: ChunkFn) -> Result<Vec<u8>, String> {
        self.update
            .download(&mut *on_chunk, || {})
            .await
            .map_err(|e| e.to_string())
    }

    fn install(&self, bytes: &[u8]) -> Result<(), String> {
        self.update.install(bytes).map_err(|e| e.to_string())
    }
}

// ---------------------------------------------------------------------------
// Test double
// ---------------------------------------------------------------------------

#[cfg(any(test, feature = "test-util"))]
pub mod fake {
    use super::*;

    /// Scripted release: fixed version/notes, a canned chunk sequence, and
    /// switchable download/install failures.
    pub struct FakeRelease {
        pub version: String,
        pub notes: Option<String>,
        /// Chunk sizes handed to `on_chunk`, in order.
        pub chunks: Vec<usize>,
        pub total: Option<u64>,
        pub download_error: Option<String>,
        pub install_error: Option<String>,
        pub installed: Mutex<bool>,
    }

    impl FakeRelease {
        pub fn new(version: &str) -> Self {
            Self {
                version: version.to_string(),
                notes: None,
                chunks: vec![4, 6],
                total: Some(10),
                download_error: None,
                install_error: None,
                installed: Mutex::new(false),
            }
        }

        pub fn with_notes(mut self, notes: &str) -> Self {
            self.notes = Some(notes.to_string());
            self
        }

        pub fn failing_download(mut self, message: &str) -> Self {
            self.download_error = Some(message.to_string());
            self
        }

        pub fn failing_install(mut self, message: &str) -> Self {
            self.install_error = Some(message.to_string());
            self
        }
    }

    #[async_trait]
    impl PendingRelease for FakeRelease {
        fn version(&self) -> String {
            self.version.clone()
        }

        fn notes(&self) -> Option<String> {
            self.notes.clone()
        }

        async fn download(&self, mut on_chunk: ChunkFn) -> Result<Vec<u8>, String> {
            if let Some(e) = &self.download_error {
                return Err(e.clone());
            }
            let mut bytes = Vec::new();
            for len in &self.chunks {
                on_chunk(*len, self.total);
                bytes.extend(std::iter::repeat_n(0u8, *len));
            }
            Ok(bytes)
        }

        fn install(&self, _bytes: &[u8]) -> Result<(), String> {
            if let Some(e) = &self.install_error {
                return Err(e.clone());
            }
            *self.installed.lock().expect("installed poisoned") = true;
            Ok(())
        }
    }

    /// Canned `check` outcome.
    pub struct FakeUpdateSource {
        outcome: Mutex<Option<CheckResult>>,
    }

    impl FakeUpdateSource {
        pub fn up_to_date() -> Self {
            Self {
                outcome: Mutex::new(Some(Ok(None))),
            }
        }

        pub fn found(release: Arc<dyn PendingRelease>) -> Self {
            Self {
                outcome: Mutex::new(Some(Ok(Some(release)))),
            }
        }

        pub fn failing(message: &str) -> Self {
            Self {
                outcome: Mutex::new(Some(Err(message.to_string()))),
            }
        }
    }

    #[async_trait]
    impl UpdateSource for FakeUpdateSource {
        async fn check(&self) -> CheckResult {
            self.outcome
                .lock()
                .expect("outcome poisoned")
                .clone()
                .unwrap_or(Ok(None))
        }
    }

    impl Clone for FakeUpdateSource {
        fn clone(&self) -> Self {
            Self {
                outcome: Mutex::new(self.outcome.lock().expect("outcome poisoned").clone()),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::fake::{FakeRelease, FakeUpdateSource};
    use super::*;
    use crate::events::VecSink;

    fn states(sink: &VecSink) -> Vec<AppUpdateState> {
        sink.events()
            .into_iter()
            .filter_map(|e| match e {
                AppEvent::AppUpdateStatus(s) => Some(s.state),
                _ => None,
            })
            .collect()
    }

    fn updater() -> (Arc<AppUpdater>, Arc<VecSink>) {
        let sink = Arc::new(VecSink::new());
        let updater = Arc::new(AppUpdater::new("0.1.1", sink.clone()));
        (updater, sink)
    }

    #[test]
    fn starts_idle_with_the_running_version() {
        let (updater, sink) = updater();
        let status = updater.status();
        assert_eq!(status.current_version, "0.1.1");
        assert_eq!(status.state, AppUpdateState::Idle);
        assert_eq!(status.last_trigger, None);
        assert!(sink.events().is_empty(), "constructing emits nothing");
    }

    #[tokio::test]
    async fn up_to_date_check_emits_checking_then_up_to_date() {
        let (updater, sink) = updater();
        let source = FakeUpdateSource::up_to_date();
        updater
            .check_and_download(&source, UpdateCheckTrigger::Manual)
            .await;
        assert_eq!(
            states(&sink),
            vec![AppUpdateState::Checking, AppUpdateState::UpToDate]
        );
        assert_eq!(
            updater.status().last_trigger,
            Some(UpdateCheckTrigger::Manual)
        );
    }

    #[tokio::test]
    async fn failed_check_lands_in_error_with_the_message() {
        let (updater, sink) = updater();
        let source = FakeUpdateSource::failing("network unreachable");
        updater
            .check_and_download(&source, UpdateCheckTrigger::Automatic)
            .await;
        assert_eq!(
            states(&sink).last(),
            Some(&AppUpdateState::Error {
                message: "network unreachable".into()
            })
        );
        assert_eq!(
            updater.status().last_trigger,
            Some(UpdateCheckTrigger::Automatic),
            "the frontend needs the trigger to decide whether to toast"
        );
    }

    #[tokio::test]
    async fn found_update_downloads_automatically_and_reports_progress() {
        let (updater, sink) = updater();
        let release = Arc::new(FakeRelease::new("0.2.0").with_notes("Adds auto-update"));
        let source = FakeUpdateSource::found(release);
        updater
            .check_and_download(&source, UpdateCheckTrigger::Automatic)
            .await;

        let seen = states(&sink);
        assert_eq!(seen[0], AppUpdateState::Checking);
        assert_eq!(
            seen[1],
            AppUpdateState::Downloading {
                version: "0.2.0".into(),
                received: 0,
                total: None
            }
        );
        // One progress event per chunk, cumulative.
        assert_eq!(
            seen[2],
            AppUpdateState::Downloading {
                version: "0.2.0".into(),
                received: 4,
                total: Some(10)
            }
        );
        assert_eq!(
            seen[3],
            AppUpdateState::Downloading {
                version: "0.2.0".into(),
                received: 10,
                total: Some(10)
            }
        );
        // The terminal state depends on whether the test binary's directory is
        // writable; both outcomes are legitimate and neither installs anything.
        match seen.last().expect("a terminal state") {
            AppUpdateState::ReadyToInstall { version, notes } => {
                assert_eq!(version, "0.2.0");
                assert_eq!(notes.as_deref(), Some("Adds auto-update"));
            }
            AppUpdateState::ManualInstallRequired { version, .. } => {
                assert_eq!(version, "0.2.0");
            }
            other => panic!("unexpected terminal state: {other:?}"),
        }
    }

    #[tokio::test]
    async fn download_failure_lands_in_error_and_leaves_nothing_to_install() {
        let (updater, sink) = updater();
        let release = Arc::new(FakeRelease::new("0.2.0").failing_download("connection reset"));
        let source = FakeUpdateSource::found(release);
        updater
            .check_and_download(&source, UpdateCheckTrigger::Manual)
            .await;
        assert_eq!(
            states(&sink).last(),
            Some(&AppUpdateState::Error {
                message: "connection reset".into()
            })
        );
        assert_eq!(
            updater.install(),
            Err("no update has been downloaded".into())
        );
    }

    #[tokio::test]
    async fn install_without_a_download_is_refused() {
        let (updater, _sink) = updater();
        assert_eq!(
            updater.install(),
            Err("no update has been downloaded".into())
        );
    }

    #[tokio::test]
    async fn install_applies_the_downloaded_bytes() {
        let (updater, _sink) = updater();
        let release = Arc::new(FakeRelease::new("0.2.0"));
        let source = FakeUpdateSource::found(release.clone());
        updater
            .check_and_download(&source, UpdateCheckTrigger::Manual)
            .await;
        assert_eq!(updater.install(), Ok(()));
        assert!(*release.installed.lock().expect("installed poisoned"));
    }

    #[tokio::test]
    async fn install_failure_surfaces_as_error_state() {
        let (updater, sink) = updater();
        let release = Arc::new(FakeRelease::new("0.2.0").failing_install("permission denied"));
        let source = FakeUpdateSource::found(release);
        updater
            .check_and_download(&source, UpdateCheckTrigger::Manual)
            .await;
        assert_eq!(updater.install(), Err("permission denied".into()));
        assert_eq!(
            states(&sink).last(),
            Some(&AppUpdateState::Error {
                message: "permission denied".into()
            })
        );
    }

    #[test]
    fn bundle_root_finds_the_dot_app_ancestor() {
        assert_eq!(
            bundle_root(Path::new(
                "/Applications/Pack-Manager.app/Contents/MacOS/pack-manager"
            )),
            PathBuf::from("/Applications/Pack-Manager.app")
        );
    }

    #[test]
    fn bundle_root_outside_a_bundle_is_the_executables_directory() {
        assert_eq!(
            bundle_root(Path::new("/tmp/target/debug/pack-manager")),
            PathBuf::from("/tmp/target/debug")
        );
    }
}
