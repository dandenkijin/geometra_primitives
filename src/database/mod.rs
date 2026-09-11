//! Database layer — dual-database predictive compilation pipeline.
//!
//! Contracts preserved from P1 (bootstrap / geometric contracts):
//! - `f32x4` dense SIMD (`core::simd`) — native `f32x4` arrays.
//! - Branchless arithmetic (`0` executable branches in forecasting / ring loop).
//! - Dense arrays (`[f32; 4]` / `f32x4` blocks); `Odd`/`Even` split preserved.
//! - `std-lib` only; zero heap allocations in arithmetic / forecasting loop
//!   (`Vec` only in pipeline; `String` only in error paths; `format!` only for `Chain`).
//! - Exact arithmetic (scalar FMA mapped directly to `simd` operations).
//! - Multi-backend independent (`database` module does not modify geometric contracts).
//! - Read-only observation of `PgaAst` / `PgaIr` (confirmed by user: cascading + read-only).
//! - `.Logos` unstable interaction preserved (reference only); manual `.pga` tokenizer accurate.

pub mod ladybug;
pub mod tiledb;
pub mod forecast;
pub mod ring_buffer;
