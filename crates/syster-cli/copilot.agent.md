# Copilot Agent: syster-cli

## Purpose
`syster-cli` is the **command-line interface** for Syster, providing tools for parsing, analyzing, and working with SysML v2 and KerML files from the terminal. It's the user-facing interface for batch operations and developer workflows.

## Scope
- **CLI Commands**: Parse, analyze, validate, format, etc.
- **Terminal UI**: Progress bars, colorized output, error reporting
- **Batch Operations**: Process multiple files, generate reports
- **Developer Tools**: Debug commands, introspection, testing helpers

## Responsibilities

### ✅ This Agent Owns

1. **Command-Line Interface** (`src/`)
   - Argument parsing (clap)
   - Command dispatch
   - Subcommand implementations

2. **Commands** (planned/future)
   - `syster parse` - Parse files and report errors
   - `syster validate` - Semantic validation
   - `syster format` - Code formatting
   - `syster stats` - Project statistics
   - `syster debug` - Debug introspection

3. **Terminal Output** (`src/`)
   - Progress reporting
   - Error formatting (user-friendly)
   - Colorized output
   - Tabular reports

4. **Batch Processing** (`src/`)
   - Multi-file operations
   - Parallel processing (optional)
   - Report generation

### ❌ This Agent Does NOT Own

- **Parser Logic**: Grammar, AST construction (belongs to syster-base)
- **Semantic Analysis**: Symbol resolution, relationship graphs (syster-base)
- **LSP Protocol**: Tower-LSP, editor integration (belongs to syster-lsp)
- **Core Types**: AST, Symbol, Workspace (syster-base)

## Public Surface (Contract)

### Exported Types
```rust
// Minimal exports (mostly a binary crate)
// Public for testing purposes
#[cfg(test)]
pub use commands::*;
```

### Binary Target
```toml
[[bin]]
name = "syster"
path = "src/main.rs"
```

### Command-Line Interface (Stable)
```bash
# Planned stable interface
syster parse <file>           # Parse and report errors
syster validate <file>        # Full semantic validation
syster stats <directory>      # Project statistics
syster debug symbols <file>   # Show symbol table
```

### Stability Guarantees
- **CLI arguments**: Stable (follow semver)
- **Output format**: May evolve (use --json for stable)
- **Exit codes**: Stable (0=success, 1=error, 2=usage error)
- **Internal code**: Free to refactor

## Dependency Rules

### Allowed Dependencies (External)
- ✅ `clap` (CLI argument parsing)
- ✅ `colored` or `owo-colors` (terminal colors)
- ✅ `indicatif` (progress bars)
- ✅ `syster-base` (core library - **required**)
- ✅ `syster-lsp` (optional, for LSP testing tools)

### Forbidden Dependencies
- ❌ **NO** `tower-lsp` (LSP should stay in syster-lsp)
- ❌ **NO** direct `pest` usage (use syster-base parser)
- ❌ **NO** web frameworks (CLI is terminal-based)
- ❌ **NO** GUI frameworks

### Dependency Direction
```
syster-base
    ↑
syster-lsp (optional)
    ↑
syster-cli  (depends on both)
```

## Change Sensitivity

### High Sensitivity (Breaking Changes)
- **CLI arguments** (affects user scripts)
- **Exit codes** (affects automation)
- **Stable output formats** (JSON, structured output)

### Medium Sensitivity (Require Coordination)
- **Error messages** (users may parse these)
- **Default behavior** (affects UX)
- **Command names** (aliases can help migration)

### Low Sensitivity (Free to Change)
- **Internal code structure**
- **Progress bar styling**
- **Debug output formatting**
- **Performance optimizations**

## Architectural Constraints

### 1. Thin Command Layer
`syster-cli` should be a **thin interface** to syster-base:
```rust
// ✅ Good: Delegates to syster-base
fn parse_command(path: &Path) -> Result<()> {
    let file = FileLoader::load_and_parse(path)?;  // ← base
    print_parse_result(&file);  // ← format for terminal
    Ok(())
}

// ❌ Bad: Reimplements parsing
fn parse_command(path: &Path) -> Result<()> {
    // Custom parsing logic
    // Should use syster-base!
}
```

### 2. Synchronous by Default
CLI operations are inherently sequential:
- User runs one command at a time
- Output is streamed to terminal
- No need for async complexity (unless doing parallel file processing)

### 3. User-Friendly Errors
Convert technical errors to user-friendly messages:
```rust
// ✅ Good: User-friendly error
match parse_file(path) {
    Err(e) => {
        eprintln!("Failed to parse {}: {}", path.display(), e);
        eprintln!("Hint: Check for syntax errors near line X");
    }
}

// ❌ Bad: Raw debug output
Err(e) => panic!("{:?}", e);  // Users don't want this
```

### 4. Machine-Readable Output (Optional)
For automation, provide structured output:
```bash
# Human-readable (default)
syster parse file.sysml
Error: Missing semicolon at line 10

# Machine-readable (opt-in)
syster parse file.sysml --json
{"status": "error", "line": 10, "message": "Missing semicolon"}
```

## Common Patterns

### Adding a New Command
```rust
// 1. Define in CLI args (main.rs or cli.rs)
#[derive(Parser)]
enum Commands {
    Parse { file: PathBuf },
    NewCommand { /* args */ },  // ← Add here
}

// 2. Implement handler (commands/new_command.rs)
pub fn run(args: NewCommandArgs) -> Result<()> {
    // Delegate to syster-base
    let workspace = Workspace::new();
    // ... use base APIs
    
    // Format output for terminal
    println!("Result: {:?}", result);
    Ok(())
}

// 3. Wire up in main dispatch
match cli.command {
    Commands::Parse { file } => commands::parse::run(file),
    Commands::NewCommand { .. } => commands::new_command::run(..),
}
```

### Progress Reporting
```rust
use indicatif::{ProgressBar, ProgressStyle};

let pb = ProgressBar::new(file_count as u64);
pb.set_style(ProgressStyle::default_bar()
    .template("[{bar:40}] {pos}/{len} {msg}"));

for file in files {
    pb.set_message(file.display().to_string());
    process_file(file)?;
    pb.inc(1);
}
pb.finish_with_message("Complete!");
```

### Error Formatting
```rust
use colored::Colorize;

fn print_error(error: &Error, path: &Path) {
    eprintln!(
        "{} Failed to process {}",
        "error:".red().bold(),
        path.display()
    );
    eprintln!("  {}", error);
}
```

## Testing Requirements

### Must Test
- ✅ Command parsing (clap integration)
- ✅ Each command's core logic
- ✅ Error handling and user messages
- ✅ Exit codes (success/failure)

### Test Organization
- Unit tests: `src/tests/` (moved from crate root)
- Integration tests: `tests/cli_tests.rs`
- Test fixtures: `tests/fixtures/`

### Coverage Goals
- Command handlers: 80%+ coverage
- Error formatting: Manual verification (UX)
- Integration tests: All main workflows

## Integration with syster-base

### Required APIs
The CLI uses these syster-base APIs:
- `FileLoader::load_and_parse()` - Load files
- `StdLibLoader::load()` - Load standard library
- `Workspace::new()` - Create workspace
- `Workspace::add_file()`, `populate_all()` - Analyze
- `SymbolTable`, `RelationshipGraph` - Introspection

### Required syster-lsp APIs (Optional)
If providing LSP testing commands:
- `LspServer::new()` - Create test server
- Testing utilities from syster-lsp

### Assumptions About Dependencies
- syster-base is **synchronous** (CLI doesn't need async)
- Parsing may be **slow** (show progress for multiple files)
- Errors are **structured** (can be formatted nicely)

## Performance Expectations

### Responsiveness
- Single file operations: <1 second
- Large project analysis: Show progress bar
- Batch operations: Parallelize if >10 files

### Resource Usage
- Memory: Proportional to files processed
- CPU: Single-threaded unless explicitly parallel
- Disk: Read-only (unless implementing format/fix)

## Known Limitations

### Current State
1. **Minimal implementation**
   - Only basic commands exist
   - Most features planned for future
   - CLI is a work in progress

2. **No formatting yet**
   - `syster format` not implemented
   - Would require formatter in syster-base first

3. **No fixing mode**
   - Can't auto-fix errors yet
   - Would need fix logic in syster-base

## Review Checklist

Before approving changes to syster-cli:
- [ ] No parser/semantic logic duplicated from syster-base
- [ ] User-friendly error messages (not raw debug)
- [ ] CLI arguments follow conventions (--flag, not -f)
- [ ] Help text is clear and accurate
- [ ] Exit codes are correct (0/1/2)
- [ ] Tests cover main command workflows
- [ ] No forbidden dependencies added
- [ ] Performance acceptable for typical use

## Future Commands (Planned)

### Parse Command
```bash
syster parse <file>
# Output: Errors, warnings, success message
# Exit: 0 if parse succeeds, 1 if errors
```

### Validate Command
```bash
syster validate <file>
# Output: Semantic errors, type errors
# Exit: 0 if valid, 1 if invalid
```

### Stats Command
```bash
syster stats <directory>
# Output: File count, symbol count, complexity metrics
# Exit: 0 always (informational)
```

### Debug Command
```bash
syster debug symbols <file>
# Output: Symbol table dump
syster debug ast <file>
# Output: AST tree
syster debug events <file>
# Output: Event trace
```

### Format Command (Future)
```bash
syster format <file>
# Output: Formatted code (or overwrite with --write)
# Exit: 0 if successful, 1 if error
```

## Coordination with Other Crates

### When syster-base Changes Affect CLI

**API changes**:
```
If: syster-base changes FileLoader API
Then: Update parse command
Impact: Medium (may need release coordination)
```

**Error types**:
```
If: syster-base changes error structure
Then: Update error formatting
Impact: Low (internal only)
```

**New features**:
```
If: syster-base adds new analysis
Then: Consider adding CLI command to expose it
Impact: Opportunity (opt-in)
```

### When syster-lsp Changes Affect CLI

**LSP testing**:
```
If: Providing LSP test commands
Then: May need to coordinate with syster-lsp changes
Impact: Low (testing only)
```

## Agent Metadata
- **Crate**: syster-cli
- **Responsibility Level**: Low (user interface, not core logic)
- **Breaking Change Impact**: Medium (affects user workflows)
- **Created**: 2025-12-17
- **Last Updated**: 2025-12-17
- **Schema Version**: 1.0
