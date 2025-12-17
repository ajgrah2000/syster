use std::collections::HashMap;
use std::path::PathBuf;
use syster::core::ParseError;
use syster::semantic::{Workspace, resolver::Resolver};
use syster::syntax::SyntaxFile;

/// LspServer manages the workspace state for the LSP server
pub struct LspServer {
    pub(super) workspace: Workspace<SyntaxFile>,
    /// Track parse errors for each file (keyed by file path)
    pub(super) parse_errors: HashMap<PathBuf, Vec<ParseError>>,
    /// Track document text for hover and other features (keyed by file path)
    pub(super) document_texts: HashMap<PathBuf, String>,
}

impl Default for LspServer {
    fn default() -> Self {
        Self::new()
    }
}

impl LspServer {
    pub fn new() -> Self {
        // Initialize workspace without loading stdlib
        // Stdlib loading is expensive and not needed for most LSP operations
        // Files can load stdlib symbols through explicit imports
        let workspace = Workspace::<SyntaxFile>::new();

        Self {
            workspace,
            parse_errors: HashMap::new(),
            document_texts: HashMap::new(),
        }
    }

    pub fn workspace(&self) -> &Workspace<SyntaxFile> {
        &self.workspace
    }

    #[allow(dead_code)] // Used in integration tests
    pub fn workspace_mut(&mut self) -> &mut Workspace<SyntaxFile> {
        &mut self.workspace
    }

    pub fn resolver(&self) -> Resolver<'_> {
        Resolver::new(self.workspace.symbol_table())
    }

    #[allow(dead_code)]
    pub fn document_texts_mut(&mut self) -> &mut HashMap<PathBuf, String> {
        &mut self.document_texts
    }
}
