//! Pure, fixture-grounded parsers (SPEC §5.5) — implemented by U3, who also
//! adds the adapter-level merge helpers here (inventory + outdated overlay →
//! snapshot rows; overlay-only rows appended; self row extracted).

pub mod brew;
pub mod mas;
pub mod mise;
pub mod npm;
pub mod rustup;
pub mod uv;
