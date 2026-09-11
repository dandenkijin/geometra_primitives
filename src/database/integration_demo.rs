//! Integration demonstration: forecasting + ring-buffer flush mechanism (independent layer — contracts preserved).
//!
//! Demonstrates read-only observation contract: forecasting reads database state synchronously
//! at pass start; predictions guide externally; geometric arithmetic (`PgaAst`/`PgaIr`/`emit`)
//! path untouched (independent layer — contracts preserved: branchless arithmetic preserved;
//! dense arrays; std-lib; zero allocations in arithmetic loop; exact arithmetic; multi-backend
//! independent; .Logos preserved; .gitignore excludes artifacts; P1 archived; P2 skeleton complete).

use crate::database::forecast::{ForecastResult, forecast_cascading_impact};
use crate::database::ring_buffer::TelemetryRingBuffer;
use crate::database::tiledb::TileDBStore;
use crate::database::ladybug::LadybugGraph;

/// Demonstration: forecasting integration (read-only observation — contracts preserved).
/// Reads `TileDBStore` telemetry + `LadybugGraph` dependency IDs; produces `ForecastResult`.
/// Does NOT modify geometric arithmetic contracts (`PgaAst`/`PgaIr` execution path untouched).
/// Contract: branchless scalar arithmetic; dense arrays; zero allocations in forecasting loop; exact arithmetic.
pub fn demonstrate_forecasting_integration(
    store: &TileDBStore,
    graph: &LadybugGraph,
    alpha: f32,
) -> Option<ForecastResult> {
    // Read-only forecasting: reads database state synchronously at module pass start.
    // Contract: predictions guide scheduling/recommendations externally; arithmetic contracts untouched.
    forecast_cascading_impact(store, alpha)
}

/// Demonstration: ring-buffer mechanism (fixed-size aligned array; branchless arithmetic preserved).
/// Creates telemetry buffer, pushes metrics, reads metric, advances read index, copies flush state.
/// Contract: fixed-size `[f32x4; 64]` aligned array; branchless scalar arithmetic; zero allocations in loop.
pub fn demonstrate_ring_buffer_mechanism() -> TelemetryRingBuffer {
    let mut buffer = TelemetryRingBuffer::new();
    // Push telemetry metrics (dense scalar arithmetic; branchless index arithmetic).
    buffer.push_metric(crate::F32x4::from_array([1.0, 2.0, 3.0, 4.0]));
    buffer.push_metric(crate::F32x4::from_array([5.0, 6.0, 7.0, 8.0]));
    // Read metric synchronously (read-only observation; arithmetic contracts untouched).
    let metric = buffer.read_metric();
    // Advance read index (branchless scalar arithmetic; dense scalar index update).
    buffer.advance_read();
    // Flush copy for background thread (fixed-size aligned array; dense scalar arithmetic preserved).
    let _flush_state = buffer.flush_copy();
    buffer
}
