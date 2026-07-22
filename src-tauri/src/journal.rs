//! Crash-safe journal `operations.jsonl` (SPEC §5.7, F8) — implemented by U5:
//! append/flush/compact/interrupted-scan; recorded pgids are NEVER signaled on
//! startup.
