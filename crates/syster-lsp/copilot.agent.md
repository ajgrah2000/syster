# Agent: syster-lsp

**LSP protocol implementation - thin async adapter over syster-base.**

## Responsibilities

### ✅ Owns
- **LSP Protocol**: tower-lsp backend, message handlers, notifications
- **Document Lifecycle**: textDocument/didOpen, didChange, didSave, didClose
- **IDE Features**: Hover, go-to-definition, references, completion, diagnostics
- **Position Mapping**: LSP Position/Range ↔ syster-base types
- **Async Coordination**: tokio runtime, async event handling

### ❌ Does NOT Own
- Parser logic (lives in syster-base)
- Semantic analysis algorithms (lives in syster-base)
- CLI features (lives in syster-cli)

## Public API

```rust
pub struct SysterLanguageServer {
    workspace: Arc<Mutex<Workspace>>,
}

#[tower_lsp::async_trait]
impl LanguageServer for SysterLanguageServer {
    async fn initialize(...) -> Result<InitializeResult>
    async fn hover(...) -> Result<Option<Hover>>
    async fn goto_definition(...) -> Result<Option<GotoDefinitionResponse>>
    // ... standard LSP handlers
}
```

## Dependencies

**Allowed**: `syster-base`, `tower-lsp`, `lsp-types`, `tokio`, `serde`, `serde_json`  
**Forbidden**: `clap`, `syster-cli`, direct pest imports (use base abstractions)

## Change Sensitivity

**Breaking** (coordinate with editor clients):
- InitializeResult capabilities changes
- Custom LSP extensions/notifications
- Configuration schema changes

**Safe**:
- New LSP feature support (adds handlers)
- Performance improvements (caching, debouncing)
- Diagnostic quality improvements
- Internal event handler changes

## Constraints

**DO**:
- Delegate to syster-base for all analysis
- Handle async/sync boundary cleanly (spawn_blocking if needed)
- Return graceful errors to client (don't panic)
- Map syster-base events to LSP notifications

**DON'T**:
- Duplicate parser/semantic logic
- Block tokio runtime with sync operations
- Add syster-base dependencies (wrong direction)
- Implement business logic (belongs in base)

## Patterns

**Async → sync delegation**:
```rust
async fn goto_definition(&self, params: GotoDefinitionParams) -> Result<...> {
    let workspace = self.workspace.lock().await;
    let position = convert_position(params.text_document_position);
    let symbol = workspace.get_symbol_table().resolve_at_position(position)?;
    Ok(convert_to_lsp_location(symbol.definition))
}
```

**Event subscription**:
```rust
workspace.subscribe(|event| match event {
    WorkspaceEvent::FilePopulated(path, _) => {
        client.publish_diagnostics(uri, diagnostics, None).await;
    }
    _ => {}
});
```

## Review Checklist
- [ ] No parser/semantic logic added
- [ ] Delegates to syster-base
- [ ] Handles async properly (no blocking)
- [ ] Returns LSP-compliant responses
- [ ] Maps base events to LSP notifications
- [ ] Error responses user-friendly

## Coordination

**Needs base when**: New query APIs, event types, or position utilities  
**Consult orchestrator if**: Base API breaking changes, new event handling patterns

---

*Agent Contract v1.0*
