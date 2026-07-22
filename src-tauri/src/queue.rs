//! Lock-set operation scheduler (SPEC §5.7) — implemented by U5: single task,
//! atomic lock acquisition, FIFO + skip-ahead + 120s aging guard, Semaphore(4),
//! coalescing, routed dual-locks, npm/uv→Mise guards, plan builder.
