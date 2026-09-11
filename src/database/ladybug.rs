//! LadybugDB — in-memory graph store (SIMD-aligned flat index arrays).
//!
//! Contract (from design.md): flat parallel index arrays (`u32x4` / `u32x8` SIMD-aligned
//! IDs) tracking syntax trees (`PgaAst` nodes) and inter-module dependency graphs.
//! No pointer-heavy nodes; raw IDs load directly into vector registers for branchless
//! variant checks.
//!
//! Zero allocations in lookup loops; dense scalar arithmetic only.

use std::simd::{f32x4, u32x4};

/// Flat parallel index array: SIMD-aligned module IDs (dense, branchless load).
/// Stores raw `Module_ID` values for dependency graph tracking.
/// Memory layout: contiguous `u32` array mapped to `u32x4` blocks (4 IDs per SIMD reg).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ModuleIdBlock {
    /// 4 module IDs (SIMD-aligned `u32x4`).
    pub ids: u32x4,
}

/// Dependency graph edge (dense scalar representation): source module → target module.
/// No heap pointers; raw indices only.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DependencyEdge {
    /// Source module index (dense scalar).
    pub src: u32,
    /// Target module index (dense scalar).
    pub tgt: u32,
}

/// LadybugDB graph store: flat index arrays for cascading-boundary tracking.
/// No pointer-heavy nodes; all data dense and contiguous.
/// Read-only observation contract: this module reads `PgaAst` / `PgaIr`
/// state IDs but does NOT modify geometric arithmetic contracts.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct LadybugGraph {
    /// Flat module ID blocks (`u32x4` aligned for SIMD loads).
    pub module_blocks: Vec<ModuleIdBlock>,
    /// Flat dependency edges (dense scalar pairs; `Vec` only in pipeline, not in arithmetic loop).
    pub edges: Vec<DependencyEdge>,
}

impl LadybugGraph {
    /// Create an empty graph (dense array initialization; zero allocations in arithmetic loop).
    pub fn new() -> Self {
        Self {
            module_blocks: Vec::new(),
            edges: Vec::new(),
        }
    }

    /// Add a dense `ModuleIdBlock` (SIMD-aligned `u32x4`) — branchless insertion.
    /// Contract: no allocations inside arithmetic / forecasting loops (only pipeline-level `Vec` growth).
    pub fn push_module_block(&mut self, block: ModuleIdBlock) {
        self.module_blocks.push(block);
    }

    /// Add a dense dependency edge (`src` → `tgt`) — branchless scalar insertion.
    pub fn push_edge(&mut self, src: u32, tgt: u32) {
        self.edges.push(DependencyEdge { src, tgt });
    }

    /// Read-only dependency count for cascading-boundary forecasting.
    /// Returns dense scalar (`usize`) — branchless arithmetic, exact count.
    /// Contract: zero allocations; dense scalar arithmetic only.
    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }

    /// Read-only module block count — dense scalar arithmetic.
    pub fn module_block_count(&self) -> usize {
        self.module_blocks.len()
    }
}
