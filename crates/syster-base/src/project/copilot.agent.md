# Sub-Agent: project

**Multi-file project management - orchestrates loading, parsing, analysis.**

## Purpose
Coordinates file loading, standard library management, and workspace initialization. The "application layer" that ties together parsing and semantic analysis.

## Responsibilities

### ✅ Owns
- **File Loading**: Discover, read, cache SysML/KerML files
- **Stdlib Management**: Load and integrate standard library
- **Workspace Loading**: Initialize multi-file projects
- **Diagnostic Publishing**: Aggregate and report errors
- **Import Resolution**: File-level dependency tracking

### ❌ Does NOT Own
- Parsing logic (parser/)
- AST types (syntax/)
- Semantic analysis algorithms (semantic/)
- LSP protocol (syster-lsp)
- CLI commands (syster-cli)

## Dependency Rules

**Allowed**: `core`, `parser`, `syntax`, `semantic`, file I/O crates  
**Forbidden**: `syster-lsp`, `syster-cli`, `tower-lsp`, `clap`

**Flow**: Load files → Parse → Populate workspace → Emit events

## Architectural Patterns

**Orchestration Pattern**:
```rust
// project/workspace_loader.rs
pub fn load_workspace(root: PathBuf) -> Result<Workspace> {
    let mut workspace = Workspace::new(root);
    
    // 1. Discover files
    let files = file_loader::discover_files(&root)?;
    
    // 2. Parse files (delegates to parser/)
    for file in files {
        let ast = parser::parse_file(&file)?;
        
        // 3. Populate semantic model (delegates to semantic/)
        workspace.add_file(file, ast)?;
    }
    
    // 4. Load stdlib (if needed)
    stdlib_loader::load(&mut workspace)?;
    
    Ok(workspace)
}
```

**DO**:
- Delegate parsing to parser/
- Delegate analysis to semantic/
- Manage file I/O and caching
- Emit diagnostics via events
- Handle cross-file dependencies

**DON'T**:
- Implement parsing logic
- Implement semantic analysis
- Add LSP-specific code
- Add CLI-specific code
- Bypass parser/semantic layers

## Change Sensitivity

**Breaking**:
- Workspace initialization API changes
- File loading strategy changes (affects performance)
- Event emission changes

**Safe**:
- File discovery optimizations
- Caching improvements
- Better diagnostic aggregation
- Stdlib loading strategy

## Review Checklist
- [ ] Delegates to parser/ for parsing
- [ ] Delegates to semantic/ for analysis
- [ ] No parsing logic implemented here
- [ ] No semantic analysis implemented here
- [ ] File I/O errors handled gracefully
- [ ] Events emitted for major operations

## Coordination

**With parser/**: Uses parsers to convert files → ASTs  
**With semantic/**: Populates Workspace with parsed files

---

*Sub-Agent v1.0 - Project Layer*
