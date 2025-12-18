# Future Work

## KerML Support Implementation (PRIORITY - Next Major Feature)

### Phase 1: Enable Basic KerML Parsing (Week 1) 🔥
- [ ] **parser/ Sub-Agent** - Task 1.1: Create `parse_kerml()` function in [`parser/kerml.rs`](../crates/syster-base/src/parser/kerml.rs)
  - Use existing [`KerMLParser`](../crates/syster-base/src/parser/kerml.rs) and pest grammar
  - Convert pest pairs to [`KerMLFile`](../crates/syster-base/src/syntax/kerml/ast.rs) AST using `FromPest` trait
  - Handle parse errors and return structured diagnostics
  - Pattern: Mirror [`parser/sysml.rs`](../crates/syster-base/src/parser/sysml.rs) implementation
- [ ] **parser/ Sub-Agent** - Task 1.2: Add unit tests for `parse_kerml()`
  - Test basic classifier parsing
  - Test package parsing
  - Test feature parsing
  - Test import statements
  - Verify existing 28 tests in [`kerml_tests.rs`](../crates/syster-base/tests/parser/kerml_tests.rs) still pass

- [ ] **syntax/ Sub-Agent** - Task 1.3: Verify `FromPest` implementations complete
  - Check all KerML AST types in `syntax/kerml/` have `FromPest` trait
  - Add missing implementations if needed
  - Test conversions work correctly
- [ ] **syntax/ Sub-Agent** - Task 1.4: Add `SyntaxFile` integration
  - Ensure `SyntaxFile::KerML(KerMLFile)` variant works
  - Implement `extract_imports()` stub (returns `vec![]` for now)
  - Verify pattern matching works in file loader

- [ ] **project/ Sub-Agent** - Task 1.5: Update [`parsing.rs`](../crates/syster-base/src/project/file_loader/parsing.rs) to call `parse_kerml()`
  - Wire up `KERML_EXT` case to actually parse instead of returning placeholder
  - Follow same pattern as `SYSML_EXT` case
- [ ] **project/ Sub-Agent** - Task 1.6: Add integration test for loading KerML files
  - Test loading single KerML file
  - Test loading stdlib KerML files (36 files)
  - Verify no parse errors on valid KerML

**Success Criteria**: All 36 stdlib KerML files parse without errors

---

### Phase 2: Symbol Population (Week 1-2) 🔥
- [ ] **semantic/adapters/ Sub-Agent** - Task 2.1: Create `semantic/adapters/kerml/mod.rs`
  - Export `KermlAdapter`, `KermlValidator` (placeholder for now)
  - Follow same structure as [`adapters/sysml/`](../crates/syster-base/src/semantic/adapters/sysml/)

- [ ] **semantic/adapters/ Sub-Agent** - Task 2.2: Create `semantic/adapters/kerml/visitors.rs` with `KermlAdapter`
  - Implement visitor pattern for KerML AST traversal
  - Populate symbols: classifiers, packages, features
  - Track relationships: specialization, subsetting, typing, redefinition
  - Handle nested scopes correctly
  - Pattern: Mirror [`adapters/sysml/visitors.rs`](../crates/syster-base/src/semantic/adapters/sysml/visitors.rs)

- [ ] **semantic/adapters/ Sub-Agent** - Task 2.3: Create `semantic/adapters/kerml/helpers.rs`
  - `classifier_kind_to_string()` - Map [`ClassifierKind`](../crates/syster-base/src/syntax/kerml/model/types.rs) enum to string
  - `feature_kind_to_string()` - Map feature types
  - Pattern: Similar to [`adapters/sysml/helpers.rs`](../crates/syster-base/src/semantic/adapters/sysml/helpers.rs)

- [ ] **semantic/adapters/ Sub-Agent** - Task 2.4: Update `semantic/adapters/syntax_factory.rs`
  - Add `SyntaxFile::KerML` case to `populate_from_syntax()`
  - Call `KermlAdapter::populate()` for KerML files

- [ ] **semantic/adapters/ Sub-Agent** - Task 2.5: Add unit tests for KerML adapter
  - Test classifier population
  - Test package population
  - Test feature population
  - Test relationship tracking (specialization, subsetting, typing)

- [ ] **semantic/workspace/ Sub-Agent** - Task 2.6: Verify [`workspace/populator.rs`](../crates/syster-base/src/semantic/workspace/populator.rs) works with KerML
  - Test that `populate_file()` delegates to KerML adapter
  - Verify cross-file symbol resolution works
  - Test dependency tracking for KerML imports

- [ ] **semantic/workspace/ Sub-Agent** - Task 2.7: Add integration tests in `tests/semantic/kerml_workspace_tests.rs`
  - Test loading KerML stdlib (36 files, symbols populated)
  - Test cross-file relationships between KerML files
  - Test KerML → SysML references (foundation layer support)

**Success Criteria**: All 36 stdlib KerML files populate symbols correctly

---

### Phase 3: Import Resolution (Week 2)
- [ ] **semantic/processors/ Sub-Agent** - Task 3.1: Create `semantic/processors/kerml_imports.rs`
  - Extract import paths from KerML AST
  - Handle namespace imports (`::*`)
  - Handle recursive imports (`::**`)
  - Handle membership imports

- [ ] **semantic/processors/ Sub-Agent** - Task 3.2: Update [`syntax/kerml/ast.rs`](../crates/syster-base/src/syntax/kerml/ast.rs)
  - Add `extract_imports()` method to `KerMLFile`
  - Call processor from AST method

- [ ] **semantic/processors/ Sub-Agent** - Task 3.3: Update [`syntax/file.rs`](../crates/syster-base/src/syntax/file.rs)
  - Wire up `SyntaxFile::KerML` case in `extract_imports()`
  - Replace `vec![]` stub with actual call

- [ ] **semantic/processors/ Sub-Agent** - Task 3.4: Add tests for import extraction
  - Test membership imports
  - Test namespace imports (`::*`)
  - Test recursive imports (`::**`)
  - Test cross-file import resolution

**Success Criteria**: KerML imports resolve cross-file correctly

---

### Phase 4: Relationship Graph Integration (Week 2-3)
- [ ] **semantic/graphs/ Sub-Agent** - Task 4.1: Verify relationship graph handles KerML relationships
  - Specialization (`:>`)
  - Subsetting (`:>`)
  - Redefinition (`:>>`)
  - Feature typing (`:`)
  - Conjugation (`~`)

- [ ] **semantic/graphs/ Sub-Agent** - Task 4.2: Add relationship queries for KerML
  - `get_specializations(classifier: &str) -> Vec<String>`
  - `get_features(classifier: &str) -> Vec<String>`
  - `get_typed_by(feature: &str) -> Option<String>`

- [ ] **semantic/graphs/ Sub-Agent** - Task 4.3: Add tests in `tests/semantic/kerml_graph_tests.rs`
  - Test specialization chains
  - Test feature typing
  - Test subsetting relationships
  - Test transitive relationship queries

**Success Criteria**: Can query relationships between KerML elements

---

### Phase 5: LSP Support (Week 3)
- [ ] **syster-lsp Agent** - Task 5.1: Update [`server/document.rs`](../crates/syster-lsp/src/server/document.rs)
  - Remove "KerML not supported" error
  - Add `parse_kerml_file()` case
  - Update workspace with parsed KerML files

- [ ] **syster-lsp Agent** - Task 5.2: Add LSP feature tests for KerML
  - Test hover on KerML classifiers
  - Test go-to-definition across KerML files
  - Test find-references in KerML
  - Test diagnostics for invalid KerML

- [ ] **syster-lsp Agent** - Task 5.3: Update VS Code extension
  - Add `.kerml` to file associations in `package.json`
  - Test syntax highlighting for KerML (if needed)
  - Verify all LSP features work with `.kerml` files

- [ ] **syster-cli Agent** - Task 5.4: Update `stats` command to include KerML
  - Report KerML file count
  - Report KerML symbol count
  - Show KerML vs SysML breakdown

- [ ] **syster-cli Agent** - Task 5.5: Update `parse` command to handle KerML
  - Test parsing individual KerML files
  - Show KerML-specific diagnostics

**Success Criteria**: VS Code extension works with `.kerml` files, all LSP features enabled

---

### Phase 6: Documentation & Cleanup (Week 3)
- [ ] **syster-base Agent** - Task 6.1: Update [`TODO.md`](../TODO.md)
  - Remove "KerML parser note" (it now works!)
  - Add any remaining KerML work items

- [ ] **syster-base Agent** - Task 6.2: Update [`ARCHITECTURE.md`](../ARCHITECTURE.md)
  - Document KerML support
  - Explain KerML vs SysML adapters
  - Show example of adding new language support

- [ ] **syster-base Agent** - Task 6.3: Update [`docs/CONTRIBUTING.md`](CONTRIBUTING.md)
  - Add section on KerML testing
  - Document stdlib KerML files

- [ ] **syster-base Agent** - Task 6.4: Add examples
  - Create `examples/kerml/basic.kerml` with common patterns
  - Create `examples/kerml/relationships.kerml` showing specialization, etc.

**Success Criteria**: Complete documentation for KerML support

---

## LSP Feature Implementation (Priority Order)

### In Progress / Next Tasks

### Architecture Notes
- **Reusable patterns:**
  - `extract_word_at_cursor()` - Used in: go-to-def, find-refs, semantic-tokens
  - `find_symbol_at_position()` - Used in: hover, go-to-def, find-refs
  - Symbol lookup fallback (qualified → simple → all_symbols) - Used in: go-to-def, find-refs
  - Main files ~10-20 lines with focused submodules handling specific concerns.

## Event System
- [ ] Event batching for bulk operations
- [ ] Event replay/history for debugging
- [ ] Async event handlers (tokio/async-std)
- [ ] Priority-based listener ordering

## LSP Features
- [ ] Incremental symbol resolution (fine-grained updates)
- [ ] Workspace-wide event aggregation
- [ ] Snapshot/restore state for crash recovery

## Performance
- [ ] Parallel file population with Rayon
- [ ] Specialized symbol index (trie/inverted index)

## Testing & Quality
- [ ] Property-based testing with proptest
- [ ] Benchmark suite with criterion
- [ ] 100% public API documentation coverage
- [ ] **Test Organization & Separation of Concerns**
  - Review test files for proper organization (unit vs integration vs end-to-end)
  - Separate test helpers from test code (extract common test utilities)
  - Move integration tests to tests/ directory where appropriate
  - Ensure tests follow same modularization pattern as main code
  - Create test fixtures/builders for complex test data setup
  - Review workspace/tests.rs (934 lines) - consider splitting by feature area
  - Extract common test patterns (e.g., unwrap_sysml helper, parse_sysml helper)
  
## Architecture & Code Cleanup
### Next Module Refactoring Tasks
- [ ] **lsp/ folder** (lsp-server crate) - Review files for refactoring opportunities
- Check file sizes and identify files >100 lines
- Apply same modularization pattern as semantic/
- [ ] Metrics/observability layer for EventEmitter

### Code Cleanup
- [ ] Replace hardcoded strings in `language/sysml/populator.rs` with SYSML_KIND_* constants
- [ ] Create relationship type constants (RELATIONSHIP_SATISFY, RELATIONSHIP_PERFORM, etc.)
- [ ] Extract `is_abstract` and `is_variation` from definition_prefix in AST
- [ ] Add annotation properties to KerML types