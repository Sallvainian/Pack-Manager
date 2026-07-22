//! U2 implements `FakeRunner` here (SPEC §5.6): canned outputs keyed by
//! (program basename, args) via `.on(...).fixture(...)`, scripted line streams
//! via `.on_streaming(...).emits(...).gate(...)`, `tokio::sync::Notify` gates
//! for deterministic ordering, call recording, and panic-on-unmatched.
//!
//! Gated by `cfg(any(test, feature = "test-util"))` once implemented.
