# Agent: syster-base

**Core SysML v2/KerML parsing, semantic analysis, and workspace management.**

## Module Structure

```
syster-base/src/
├── core/          [Foundation] Error handling, events, spans, traits
├── parser/        [Parsing] PEG grammars, pest → AST conversion
├── syntax/        [AST] Language-specific AST definitions
├── semantic/      [Analysis] Language-agnostic analysis (adapters bridge syntax)
└── project/       [Orchestration] File loading, workspace coordination
```

**Sub-Agents**: Each module has `copilot.agent.md` with detailed boundaries and patterns.

## Responsibilities

### ✅ Owns
- **Parser**: PEG grammars (.pest), AST construction, parse errors (see `parser/copilot.agent.md`)
- **Semantic**: Symbol tables, relationship graphs, dependency resolution (see `semantic/copilot.agent.md`)
- **Workspace**: Multi-file projects, stdlib loading, event-driven updates (see `project/copilot.agent.md`)
- **Type System**: AST types (syntax/), semantic types, visitors, validation
- **Events**: WorkspaceEvent, EventEmitter, OperationResult (see `core/copilot.agent.md`)

### ❌ Does NOT Own
- LSP protocol (tower-lsp, message handlers)
- CLI interface (clap, terminal output)
- Async runtime management (tokio orchestration)

## Public API

```rust
// Workspace management
pub struct Workspace { ... }
impl Workspace {
    pub fn new(root: PathBuf) -> Self
    pub fn add_file(&mut self, path: &Path) -> Result<()>
    pub fn get_symbol_table(&self) -> &SymbolTable
    pub fn subscribe<F>(&mut self, handler: F)
}

// Symbol resolution
pub struct SymbolTable { ... }
impl SymbolTable {
    pub fn resolve(&self, name: &str) -> Option<&Symbol>
    pub fn get_relationships(&self, id: SymbolId) -> Vec<Relationship>
}

// Events
pub enum WorkspaceEvent {
    FileAdded(PathBuf),
    FileUpdated(PathBuf),
    FilePopulated(PathBuf, Vec<RootElement>),
    Error(PathBuf, Vec<Diagnostic>),
}
```

## Dependencies

**Allowed**: `pest`, `rayon` (parallel file processing), `walkdir`, `thiserror`  
**Forbidden**: `tower-lsp`, `lsp-types`, `clap`, `tokio` (as runtime), async-specific crates

## Change Sensitivity

**Breaking** (coordinate via orchestrator):
- WorkspaceEvent enum changes (non-exhaustive, but handlers affected)
- Workspace/SymbolTable method signature changes
- Public type removals or renames

**Safe**:
- New workspace/symbol table methods
- Internal algorithm improvements
- New visitor implementations
- Bug fixes in resolution logic

## Architectural Constraints

**Dependency Flow** (enforced by sub-agents):
```
core (no deps)
  ↓
parser → syntax (AST types)
  ↓        ↓
  semantic (adapters bridge syntax → semantic)
  ↓
project (orchestrates all)
```

**Critical Rules**:
- `core/`: Zero dependencies on other modules
- `syntax/`: Data-only ASTs, no behavior
- `semantic/`: Only `adapters/` and `processors/` import from `syntax/`
- `parser/`: Factory pattern for sysml/kerml routing
- `project/`: Delegates parsing/analysis, never implements it

**DO**:
- Keep APIs synchronous (async adapters live in lsp/cli)
- Emit events for state changes
- Use adapter/factory patterns for multi-language support
- Consult sub-agents for module-specific changes

**DON'T**:
- Add LSP-specific types (Position, Range, Diagnostic → use internal types)
- Import from syster-lsp or syster-cli
- Leak pest types in public API (wrap in domain types)
- Import syntax in semantic/ (except adapters/processors)
- Make assumptions about async context

## Patterns

**Event emission**:
```rust
workspace.add_file(path)?;
self.emit(WorkspaceEvent::FileAdded(path.clone()));
```

**Error handling**:
```rust
pub fn analyze(&self, file: &Path) -> OperationResult<Analysis> {
    let symbols = self.resolve_imports(file)?;
    Ok(Analysis { symbols, ... })
}
```

## Review Checklist
- [ ] No LSP/CLI dependencies added
- [ ] Consulted relevant sub-agent (core/parser/syntax/semantic/project)
- [ ] Dependency flow respected (see Architectural Constraints)
- [ ] Adapter/factory pattern used for multi-language features
- [ ] Public API changes documented
- [ ] Events emitted for state changes
- [ ] Tests cover new symbol resolution
- [ ] No async/await in core logic
- [ ] pest types not in public signatures

## Coordination

**For module-specific work**: Consult sub-agents first:
- `src/core/copilot.agent.md` - Foundation types, events, errors
- `src/parser/copilot.agent.md` - Grammar, parsing, AST construction
- `src/syntax/copilot.agent.md` - AST definitions, language types
- `src/semantic/copilot.agent.md` - Analysis, adapters, symbol tables
- `src/project/copilot.agent.md` - File loading, workspace orchestration

**Affects lsp when**: Events added/changed, symbol table queries modified  
**Affects cli when**: Workspace initialization changes, diagnostic format changes  
**Consult orchestrator if**: Breaking changes, new event types, API redesign

---

*Agent Contract v1.0 • Sub-Agents: 5*
