# SysML v2 Language Support for VS Code

Rich language support for SysML v2 (Systems Modeling Language) and KerML (Kernel Modeling Language) files.

## Features

- **Syntax Highlighting** - Basic highlighting for comments and strings
- **Semantic Highlighting** - Rich, context-aware token coloring via LSP
- **Diagnostics** - Real-time error and warning messages
- **Go to Definition** - Navigate to symbol definitions
- **Find References** - Find all references to a symbol
- **Hover Information** - View symbol information on hover
- **Code Completion** - Intelligent code suggestions
- **Document Symbols** - Outline view of file structure
- **Rename Symbol** - Rename symbols across files
- **Semantic Tokens** - Advanced syntax coloring based on semantic analysis

## Requirements

This extension requires the `syster-lsp` language server binary. The extension will automatically search for it in the following locations:

1. Path specified in settings: `syster.lsp.path`
2. Environment variable: `SYSTER_LSP_PATH`
3. Workspace build directory: `./target/release/syster-lsp`
4. Workspace debug directory: `./target/debug/syster-lsp`
5. System PATH

### Building the Language Server

If you're working from the source repository:

```bash
# From the repository root
cargo build --release --manifest-path crates/syster-lsp/Cargo.toml

# The binary will be at: target/release/syster-lsp
```

## Extension Settings

This extension contributes the following settings:

- `syster.lsp.path`: Path to the syster-lsp binary (leave empty for auto-detection)
- `syster.lsp.trace.server`: Trace LSP communication for debugging (`off`, `messages`, `verbose`)
- `syster.stdlib.enabled`: Load SysML standard library (default: `true`)

## Commands

- `SysML: Restart Language Server` - Restart the language server if it crashes or becomes unresponsive

## Usage

1. Open a `.sysml` or `.kerml` file
2. The extension will automatically activate and connect to the language server
3. All language features will be available immediately

## Troubleshooting

### Language server not found

If you see an error about the language server not being found:

1. Check that `syster-lsp` is built: `cargo build --release`
2. Verify the binary exists: `ls target/release/syster-lsp`
3. Or specify the path in settings: `"syster.lsp.path": "/path/to/syster-lsp"`

### Language server crashes

Use the restart command: `SysML: Restart Language Server`

### Enable detailed logging

Set `"syster.lsp.trace.server": "verbose"` in your settings, then check:
- View → Output → "SysML Language Server"

## Development

To work on this extension:

```bash
cd editors/vscode
npm install
npm run watch  # Start TypeScript compiler in watch mode

# Press F5 in VS Code to launch Extension Development Host
```

## Contributing

See the main repository: [jade-codes/syster](https://github.com/jade-codes/syster)

## License

MIT - See LICENSE.md in the repository root
