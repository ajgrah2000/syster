# Sub-Agent: parser

**PEG parsing layer - converts text to ASTs using pest.**

## Purpose
Transforms raw text into parse trees. Handles both SysML v2 and KerML using separate pest grammars.

## Responsibilities

### ✅ Owns
- **Grammar Definitions**: kerml.pest, sysml.pest (PEG rules)
- **Parser Implementation**: kerml.rs, sysml.rs (pest → AST)
- **Keywords**: Language-specific keyword lists
- **Parse Errors**: Syntax error reporting with positions

### ❌ Does NOT Own
- AST type definitions (syntax/ owns those)
- Semantic analysis (semantic/)
- Symbol resolution (semantic/)
- File loading (project/)

## Dependency Rules

**Allowed**: `core`, `syntax` (AST types only), `pest`, `pest_derive`  
**Forbidden**: `semantic`, `project` (wrong direction)

**Flow**: Text → Parser → AST (in syntax/)

## Architectural Patterns

**Factory Pattern Required**:
```rust
// DO: Use factory to route by language
match language {
    Language::SysML => sysml::parse(text),
    Language::KerML => kerml::parse(text),
}

// DON'T: Let callers know about sysml/kerml internals
```

**DO**:
- Keep parsers pure (text in, AST out)
- Return ParseResult<AST> consistently
- Map pest errors to ParseError (from core)
- Share common parsing logic between sysml/kerml

**DON'T**:
- Perform semantic analysis during parsing
- Resolve symbols or types
- Load files or manage workspace
- Expose pest types in public API (wrap them)

## Change Sensitivity

**Breaking**:
- Grammar changes affecting valid syntax
- AST structure changes (coordinate with syntax/)
- ParseResult type changes

**Safe**:
- Grammar optimizations (same parse tree)
- Better error messages
- Parse performance improvements
- Internal refactoring

## Review Checklist
- [ ] No semantic analysis logic
- [ ] No symbol resolution
- [ ] Pest types not in public API
- [ ] Both sysml/kerml updated for cross-cutting changes
- [ ] Grammar changes documented
- [ ] Error messages helpful

## Coordination

**With syntax/**: AST types must match parser output  
**With core/**: Uses ParseError, Span, OperationResult

---

*Sub-Agent v1.0 - Parsing Layer*
