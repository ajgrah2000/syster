# Sub-Agent: semantic

**Language-agnostic semantic analysis - symbols, graphs, validation.**

## Purpose
Analyzes parsed ASTs to build symbol tables, relationship graphs, and perform validation. **Must remain language-agnostic** except in adapters/ and processors/.

## Responsibilities

### ✅ Owns
- **Symbol Tables**: Cross-file symbol resolution
- **Relationship Graphs**: Definition-usage, dependency tracking
- **Validation**: Semantic rules (via adapters)
- **Workspace**: Multi-file coordination
- **Types**: SemanticRole, Symbol, Location, Diagnostic
- **Adapters**: Language-specific → semantic conversion (ONLY place that imports syntax)
- **Processors**: Language-specific extraction (imports, references)

### ❌ Does NOT Own
- Parsing (parser/)
- AST definitions (syntax/)
- File loading (project/)
- LSP/CLI concerns

## Module Structure

```
semantic/
├── adapters/          ✅ CAN import from syntax/
│   ├── sysml/         (SysML-specific adapter)
│   └── syntax_factory (creates appropriate adapter)
├── processors/        ✅ CAN import from syntax/
│   └── (reference collectors, import extractors)
├── analyzer.rs        ❌ MUST be language-agnostic
├── graphs.rs          ❌ MUST be language-agnostic
├── resolver.rs        ❌ MUST be language-agnostic
├── symbol_table.rs    ❌ MUST be language-agnostic
├── types.rs           ❌ MUST be language-agnostic
└── workspace.rs       ❌ MUST be language-agnostic
```

## Dependency Rules

**Allowed**: `core`, access to `syntax` **ONLY from adapters/ and processors/**  
**Forbidden**: `syntax` imports in analyzer/graphs/resolver/symbol_table/workspace/types (except via SemanticRole)

**Critical Boundary**: Only `adapters/` and `processors/` convert syntax → semantic. All other code uses `SemanticRole`.

## Architectural Patterns

**Adapter Pattern (REQUIRED)**:
```rust
// adapters/sysml_adapter.rs
impl SysmlAdapter {
    // Converts SysML AST → language-agnostic Symbol
    fn convert_definition(&self, def: &sysml::Definition) -> Symbol {
        Symbol {
            role: SemanticRole::Definition,  // ← language-agnostic
            name: def.name.clone(),
            // ...
        }
    }
}

// analyzer.rs (language-agnostic)
fn analyze_symbol(&self, symbol: &Symbol) {
    match symbol.role {  // ← works for ANY language
        SemanticRole::Definition => { /* ... */ }
        SemanticRole::Usage => { /* ... */ }
    }
}
```

**Factory Pattern**:
```rust
// adapters/validator_factory.rs
pub fn create_validator(file: &SyntaxFile) -> Box<dyn Validator> {
    match file {
        SyntaxFile::SysML(_) => Box::new(SysmlValidator::new()),
        SyntaxFile::KerML(_) => Box::new(KermlValidator::new()),
    }
}
```

**DO**:
- Use SemanticRole everywhere except adapters/processors
- Keep analysis generic (works for any language)
- Emit events for state changes
- Use traits for validation (language-agnostic interface)

**DON'T**:
- Import syntax types outside adapters/processors
- Add SysML/KerML-specific logic to analyzer/graphs/resolver
- Hardcode language-specific checks
- Bypass adapter layer

## Change Sensitivity

**Breaking**:
- SemanticRole enum changes (affects all adapters)
- Symbol structure changes
- WorkspaceEvent changes
- Public graph/table APIs

**Safe**:
- New adapter implementations
- New validators
- Internal algorithm improvements
- New relationship types

## Review Checklist
- [ ] No syntax imports outside adapters/processors
- [ ] Analyzer/graphs/resolver language-agnostic
- [ ] New language support = new adapter only
- [ ] Validation via adapter interface
- [ ] Events emitted for state changes
- [ ] Tests cover cross-language scenarios

## Coordination

**With syntax/**: Only via adapters (one-way: syntax → semantic)  
**With project/**: Provides Workspace that project/ uses

---

*Sub-Agent v1.0 - Semantic Layer*
