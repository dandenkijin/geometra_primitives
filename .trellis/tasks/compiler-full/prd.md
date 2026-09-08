# Compiler Full — PRD
## Goal
Build a full compiler pipeline that takes .pga syntax, parses via PgaAst (AST intermediate), emits exact WGSL shader strings, supports multi-backend slots.
## In Scope
- Parser (.pga tokenizer + AST builder)
- Emission layer (WGSL shader strings with exact geometric contracts)
- Multi-backend architecture (parser-independent emission)
- Integration with R_3_0_1 SIMD library
## Out of Scope
- Changing geometric algebra math or SIMD back-end
- GUI / IDE integration
- Performance benchmarking (future phase)
## Acceptance Criteria
- .pga file parses to PgaAst without errors
- Emitted WGSL strings match geometric contracts (wedge, vee, sandwich, primitives)
- Multi-backend slot preserved (parser -> emission independent)
- Branchless execution maintained through pipeline
