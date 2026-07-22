//! ToolEnv & PATH resolution (SPEC §5.2) — implemented by U2: static list,
//! sentinel `-lc` login-shell probe (5s timeout, 64KiB cap), merge/dedupe,
//! child-env construction, `which_in` resolution helpers.
