# Copilot Sub-Agent System

**Hierarchical agents enforce boundaries and architectural consistency.**

## Usage

**Single crate**: `@workspace Review /crates/[crate]/copilot.agent.md then [task]`  
**Cross-crate**: `@workspace Use orchestrator at /copilot/orchestrator.md for [change]`

## Structure

```
syster-base (no deps) → syster-lsp → syster-cli
```

**Rule**: No upward dependencies. Base stays framework-agnostic.

## Which Agent?

| Working on | Consult |
|------------|---------|
| Parser, AST, semantic analysis | `syster-base/copilot.agent.md` |
| LSP features, document sync | `syster-lsp/copilot.agent.md` + base if API needed |
| CLI commands, terminal output | `syster-cli/copilot.agent.md` + base if API needed |
| Breaking changes, cross-crate | `copilot/orchestrator.md` first |

## Common Workflows

**New LSP feature**: Check lsp agent → verify base has APIs → implement as thin wrapper  
**Base API change**: Consult orchestrator → check downstream impact → coordinate updates  
**New event**: Add to base WorkspaceEvent → check if lsp needs handler → update contracts

## Agent Contract Sections

- **Purpose**: What the crate does
- **Responsibilities**: ✅ Owns / ❌ Doesn't own
- **Public API**: Exports and stability
- **Dependencies**: Allowed / forbidden
- **Change Sensitivity**: Breaking vs safe
- **Constraints**: Patterns and anti-patterns
- **Checklist**: Pre-approval verification

## Enforced Boundaries

**Orchestrator flags**: LSP code in base • wrong dependency direction • uncoordinated breaking changes  
**Agents prevent**: Feature creep • tight coupling • implicit contracts • surprise breakage

## Examples

**✅ Approved**: Add SymbolTable.fuzzy_search() → internal to base, backward compatible  
**⚠️ Coordinated**: Add WorkspaceEvent.Renamed → affects base + lsp, update both  
**❌ Rejected**: Add tower-lsp to base → violates framework-agnostic rule, extract generic helper instead

## Maintenance

**Update agents when**: Public API changes • new responsibilities • dependency changes  
**Don't update for**: Internal refactoring • bug fixes • performance tweaks  
**Review**: Quarterly + on breaking changes + before new crates

## PR Checklist
- [ ] Respects agent boundaries
- [ ] Agents consulted and updated
- [ ] Breaking changes coordinated
- [ ] Dependencies match rules

## Troubleshooting

**Copilot violates rules**: Reference agent explicitly in prompt  
**Unsure which agent**: Start with orchestrator  
**Agent outdated**: Update contract with code changes

---

**Note**: Agents guide architecture, not block evolution. Update agents to reflect new patterns.

*v1.0 • 2025-12-17*
