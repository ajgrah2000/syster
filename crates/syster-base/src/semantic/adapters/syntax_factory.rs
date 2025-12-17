//! Adapter Factory
//!
//! Creates the appropriate adapter for a given syntax file type.
//! This is the only place in the semantic layer that knows about specific adapter implementations.

use crate::semantic::graphs::RelationshipGraph;
use crate::semantic::symbol_table::SymbolTable;
use crate::semantic::types::SemanticError;
use crate::syntax::SyntaxFile;

use super::SysmlAdapter;

/// Populates a syntax file into the symbol table using the appropriate adapter
pub fn populate_syntax_file(
    syntax_file: &SyntaxFile,
    symbol_table: &mut SymbolTable,
    relationship_graph: &mut RelationshipGraph,
) -> Result<(), Vec<SemanticError>> {
    match syntax_file {
        SyntaxFile::SysML(sysml_file) => {
            let mut adapter = SysmlAdapter::with_relationships(symbol_table, relationship_graph);
            adapter.populate(sysml_file)
        }
        SyntaxFile::KerML(_kerml_file) => {
            // TODO: Implement KerML adapter when ready
            // let mut adapter = KermlAdapter::with_relationships(symbol_table, relationship_graph);
            // adapter.populate(kerml_file)
            // For now, skip KerML files silently instead of failing
            // This allows SysML files to be populated even when KerML files are present
            Ok(())
        }
    }
}
