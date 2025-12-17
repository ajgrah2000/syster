# Changelog

All notable changes to the "SysML v2 Language Support" extension will be documented in this file.

## [0.1.0] - 2024-12-17

### Added
- Initial release
- Basic syntax highlighting for comments and strings
- LSP-based language features:
  - Diagnostics (errors and warnings)
  - Hover information
  - Go to definition
  - Find references
  - Code completion
  - Document symbols (outline)
  - Rename symbol
  - Semantic tokens (rich syntax coloring)
- Support for both `.sysml` and `.kerml` files
- Auto-detection of `syster-lsp` binary
- "Restart Language Server" command
- Status bar indicator
- Configuration options for LSP path, tracing, and stdlib
