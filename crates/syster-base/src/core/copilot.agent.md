# Sub-Agent: core

**Foundation types and utilities - zero dependencies on other syster-base modules.**

## Purpose
Provides primitive building blocks used across all other modules: error handling, events, results, spans, text utilities.

## Responsibilities

### ✅ Owns
- **Error Infrastructure**: ErrorCodes, ParseError, SemanticError base types
- **Event System**: Generic EventEmitter, OperationResult
- **Text Utilities**: Span, Position, Range, text manipulation
- **File I/O**: Generic file loading/validation (extension checks)
- **Traits**: Core interfaces (Visitor, EventEmitter, etc.)

### ❌ Does NOT Own
- Language-specific parsing (parser/)
- AST types (syntax/)
- Semantic analysis (semantic/)
- Workspace management (project/)

## Dependency Rules

**Allowed**: std library, basic crates (thiserror, serde)  
**Forbidden**: All other syster-base modules (parser, syntax, semantic, project)

**Critical**: This is the **foundation layer** - nothing here can depend on higher modules.

## Architectural Patterns

**DO**:
- Keep types generic and reusable
- Use trait-based abstraction
- Document all public types
- Make types serializable when useful

**DON'T**:
- Add language-specific logic (no SysML/KerML)
- Import from syntax/, semantic/, parser/, project/
- Create circular dependencies
- Leak implementation details in public API

## Change Sensitivity

**Breaking**:
- Public trait signature changes
- ErrorCode enum changes (affects error handling everywhere)
- Event types (affects subscribers)
- Span/Position representation changes

**Safe**:
- New error codes added
- New utility functions
- Internal optimizations
- Additional trait implementations

## Review Checklist
- [ ] No imports from parser/syntax/semantic/project
- [ ] Types are generic/reusable
- [ ] Error types have clear codes
- [ ] Public API documented
- [ ] No language-specific logic

---

*Sub-Agent v1.0 - Foundation Layer*
