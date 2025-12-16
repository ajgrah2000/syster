use super::LspServer;
use std::path::Path;
use syster::semantic::symbol_table::Symbol;
use tower_lsp::lsp_types::{DocumentSymbol, SymbolKind};

impl LspServer {
    /// Get all symbols in a document for the outline view
    pub fn get_document_symbols(&self, file_path: &Path) -> Vec<DocumentSymbol> {
        let mut symbols = Vec::new();

        // Get all symbols from the symbol table for this file
        for (_, symbol) in self.workspace.symbol_table().all_symbols() {
            // Only include symbols defined in this file
            if symbol.source_file() != Some(file_path.to_str().unwrap_or("")) {
                continue;
            }

            if let Some(span) = symbol.span() {
                let range = super::helpers::span_to_lsp_range(&span);
                let selection_range = range; // For now, use the same range

                let symbol_kind = match symbol {
                    Symbol::Package { .. } => SymbolKind::NAMESPACE,
                    Symbol::Classifier { .. } | Symbol::Definition { .. } => SymbolKind::CLASS,
                    Symbol::Feature { .. } | Symbol::Usage { .. } => SymbolKind::PROPERTY,
                    Symbol::Alias { .. } => SymbolKind::VARIABLE,
                };

                let doc_symbol = DocumentSymbol {
                    name: symbol.name().to_string(),
                    detail: Some(symbol.qualified_name().to_string()),
                    kind: symbol_kind,
                    range,
                    selection_range,
                    children: None, // TODO: Add hierarchical children
                    tags: None,
                    #[allow(deprecated)]
                    deprecated: None,
                };

                symbols.push(doc_symbol);
            }
        }

        symbols
    }
}
