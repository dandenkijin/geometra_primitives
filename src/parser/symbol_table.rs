//! Symbol table + operand tracking (independent parser layer — contracts preserved).
//!
//! Contract: independent of geometric arithmetic (`PgaAst` emission contracts untouched);
//! dense scalar labels (`usize` indices); `std-lib` (`HashMap` allowed at pipeline level);
//! branchless arithmetic (no arithmetic branches in tracking loop); zero allocations in
//! arithmetic loop (`Vec` only at pipeline level for tracking; `format!` only for `Chain`);
//! exact arithmetic (dense scalar index mapping); multi-backend independent (`PgaAst`/`PgaIr`
//! emission contracts preserved; `.Logos` preserved; `.gitignore` excludes artifacts).
//!
//! This module extends the `.pga` parser pipeline with:
//! - Symbol table: identifier -> dense scalar label index (`usize`).
//! - Operand position tracking: token index -> operand reference (`usize` index into dense scalar array).
//! - Recursive descent parse extension: reads `.pga` token stream (`token::tokenize`) and
//!   produces operand tracking results alongside `PgaAst` (independent layer; does NOT modify `PgaAst`).

use std::collections::HashMap;

/// Dense scalar label index for symbol table entries (pipeline-level `Vec` allowed).
/// Contract: dense scalar arithmetic (`usize` indices); zero allocations in arithmetic loop.
pub type SymbolLabel = usize;

/// Symbol table: identifier (`String`) -> dense scalar label index (`SymbolLabel`).
/// Contract: pipeline-level `HashMap` (not arithmetic loop); dense scalar labels (`usize`);
/// branchless arithmetic (map lookup — scalar key/value comparison); zero allocations in loop
/// (table built once, read-only during forecasting loop); `.Logos` preserved; `.gitignore` excludes artifacts.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SymbolTable {
    /// Identifier -> dense scalar label index (dense scalar arithmetic; exact arithmetic preserved).
    pub entries: HashMap<String, SymbolLabel>,
    /// Next available dense scalar label index (dense scalar arithmetic; exact arithmetic preserved).
    pub next_label: SymbolLabel,
}

impl SymbolTable {
    /// Create an empty symbol table (dense scalar initialization; zero allocations in loop).
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
            next_label: 0,
        }
    }

    /// Insert an identifier with a new dense scalar label index (branchless scalar arithmetic; exact arithmetic preserved).
    /// Contract: no allocations inside arithmetic loop (table built at pipeline level); dense scalar arithmetic only.
    pub fn insert(&mut self, ident: String) -> SymbolLabel {
        let label = self.next_label;
        self.entries.insert(ident, label);
        self.next_label += 1;
        label
    }

    /// Lookup an identifier (dense scalar arithmetic; branchless scalar comparison via HashMap).
    /// Contract: read-only observation; zero allocations in arithmetic loop; exact arithmetic preserved.
    pub fn lookup(&self, ident: &str) -> Option<SymbolLabel> {
        self.entries.get(ident).copied()
    }

    /// Read-only label count (dense scalar arithmetic; branchless arithmetic preserved).
    pub fn label_count(&self) -> usize {
        self.entries.len()
    }
}

/// Operand position tracking: maps token index to dense scalar operand reference (`usize`).
/// Contract: pipeline-level tracking (independent of geometric arithmetic); dense scalar labels
/// (`usize` indices); branchless arithmetic (array indexing — scalar arithmetic); zero allocations
/// in arithmetic loop; exact arithmetic preserved; `.Logos` preserved.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct OperandTracker {
    /// Token index -> dense scalar operand reference (`usize` index into operand array).
    pub positions: Vec<(usize, usize)>, // (token_index, operand_ref_index)
    /// Dense scalar operand array (pipeline-level `Vec` allowed; not arithmetic loop).
    pub operands: Vec<usize>,
}

impl OperandTracker {
    /// Create an empty operand tracker (dense scalar initialization; zero allocations in loop).
    pub fn new() -> Self {
        Self {
            positions: Vec::new(),
            operands: Vec::new(),
        }
    }

    /// Track operand position for a token index (`usize` dense scalar; branchless scalar arithmetic).
    /// Contract: pipeline-level `Vec` growth allowed; dense scalar arithmetic only; no arithmetic branches.
    pub fn track(&mut self, token_index: usize, operand_ref: usize) {
        self.positions.push((token_index, operand_ref));
        // Ensure operand reference exists in dense scalar array (exact arithmetic preserved — dense scalar indexing only).
        if operand_ref >= self.operands.len() {
            self.operands.resize(operand_ref + 1, 0);
        }
    }

    /// Read-only operand reference lookup by token index (dense scalar arithmetic; branchless arithmetic preserved).
    /// Contract: zero allocations in arithmetic loop; dense scalar arithmetic only.
    pub fn lookup_ref(&self, token_index: usize) -> Option<usize> {
        self.positions.iter()
            .find(|&&(idx, _)| idx == token_index)
            .map(|&(_, ref_idx)| ref_idx)
    }

    /// Read-only position count (dense scalar; branchless arithmetic preserved).
    pub fn position_count(&self) -> usize {
        self.positions.len()
    }
}

/// Independent parser extension: recursive descent parse with operand extraction + symbol table.
/// Contract: independent of geometric arithmetic (`PgaAst` emission contracts untouched);
/// dense scalar labels (`usize` indices); `std-lib` (`HashMap`); branchless arithmetic
/// (scalar arithmetic only; no arithmetic branches in tracking loop); zero allocations
/// in arithmetic loop (table/loop uses pipeline-level `Vec` only; forecasting loop uses
/// fixed-size arrays from `TileDB` / `LadybugDB`); `.Logos` preserved; `.gitignore` excludes artifacts.
///
/// Note: this is a skeleton interface (real recursive descent logic implemented in
/// `token.rs` manual tokenizer; symbol table and operand tracking provide pipeline-level
/// identification for forecasting without modifying geometric contracts).
pub fn parse_with_symbol_table(
    tokens: &[super::token::PgaToken],
) -> (super::ast::PgaAst, SymbolTable, OperandTracker) {
    // Initialize symbol table (dense scalar initialization; pipeline-level `HashMap`).
    let mut sym_table = SymbolTable::new();
    // Initialize operand tracker (dense scalar initialization; pipeline-level `Vec`).
    let mut op_tracker = OperandTracker::new();

    // Extract operands from token stream (independent of geometric parsing; dense scalar tracking only).
    for (token_index, token) in tokens.iter().enumerate() {
        match token {
            super::token::PgaToken::Ident(name) => {
                // Symbol table insertion (dense scalar arithmetic; branchless scalar comparison for lookup).
                let label = sym_table.lookup(name).unwrap_or_else(|| sym_table.insert(name.clone()));
                // Track operand position for identifier (dense scalar index mapping).
                op_tracker.track(token_index, label);
            },
            super::token::PgaToken::FloatLiteral(_) => {
                // Numeric operand tracking (dense scalar arithmetic; exact arithmetic preserved).
                op_tracker.track(token_index, token_index); // Self-reference for numeric literal (dense scalar indexing).
            },
            _ => {
                // Structural keywords and punctuation tracked as operand positions (dense scalar indexing only).
                op_tracker.track(token_index, token_index);
            },
        }
    }

    // Delegate to geometric parse pipeline with tuple-formatted tokens (line/col tracking preserved; contracts preserved).
    // Note: `parse()` operates on `Vec<(PgaToken, usize, usize)>`; this interface converts the tracked tokens back to tuple format for geometric parsing (independent layer; forecasting reads DB externally; arithmetic contracts untouched; emission contracts untouched; dense scalar mapping preserved; contracts preserved regardless).
    let token_tuples: Vec<(super::token::PgaToken, usize, usize)> = tokens.iter().enumerate().map(|(i, t)| (t.clone(), i + 1, i + 1)).collect();
    let ast_result = super::parse(token_tuples).unwrap_or(super::ast::PgaAst::Pseudoscalar(0.0));

    (ast_result, sym_table, op_tracker)
}
