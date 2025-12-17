# Sub-Agent: syntax

**AST definitions and language-specific types - immutable parse trees.**

## Purpose
Defines the Abstract Syntax Tree structures for SysML v2 and KerML. These are language-specific, immutable representations of parsed code.

## Responsibilities

### ✅ Owns
- **AST Types**: Struct definitions for all language constructs
- **SyntaxFile**: Common wrapper for sysml/kerml files
- **Language Variants**: Separate AST hierarchies for SysML vs KerML
- **Visitor Support**: Accept visitor patterns for traversal
- **Parser Trait**: Generic parser interface

### ❌ Does NOT Own
- Parsing logic (parser/ owns that)
- Semantic analysis (semantic/)
- Symbol tables (semantic/symbol_table)
- Validation rules (semantic/adapters)

## Dependency Rules

**Allowed**: `core` (Span, traits)  
**Forbidden**: `parser`, `semantic`, `project`

**Critical**: AST types are **data-only**. No behavior, no validation, no resolution.

## Architectural Patterns

**Adapter Pattern Required**:
```rust
// ASTs are language-specific (sysml/, kerml/)
// Semantic layer uses SemanticRole (language-agnostic)
// Adapters bridge this gap (in semantic/adapters/)

pub enum SyntaxFile {
    SysML(sysml::RootElement),
    KerML(kerml::RootElement),
}
```

**DO**:
- Keep AST types immutable (no setters)
- Use enums for variants (DefinitionKind, etc.)
- Implement visitor patterns for traversal
- Document node relationships clearly

**DON'T**:
- Add semantic analysis methods
- Store resolved symbols in AST
- Mix SysML and KerML types
- Add validation logic (belongs in semantic/adapters)

## Change Sensitivity

**Breaking**:
- AST structure changes (fields added/removed/renamed)
- Enum variant changes
- Visitor trait changes

**Safe**:
- New AST node types added
- Internal derives (Debug, Clone)
- Documentation improvements
- Helper methods (non-semantic)

## Review Checklist
- [ ] No semantic analysis logic
- [ ] AST types immutable
- [ ] SysML/KerML separated cleanly
- [ ] Visitor patterns supported
- [ ] No imports from parser/semantic/project
- [ ] Types documented with examples

## Coordination

**With parser/**: Parser produces these AST types  
**With semantic/adapters**: Adapters convert AST → Symbols

---

*Sub-Agent v1.0 - AST Layer*
