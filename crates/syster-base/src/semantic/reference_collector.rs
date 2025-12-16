//! # Reference Collector
//!
//! Collects all references to symbols by analyzing relationship graphs.
//! Populates the `references` field in Symbol instances for LSP "Find References".
//!
//! ## How it works
//!
//! 1. Iterate through all symbols in the symbol table
//! 2. For each symbol with relationships (typed_by, specializes, etc.):
//!    - Get the target symbol name from the relationship graph
//!    - Add the symbol's span to the target's `references` list
//! 3. Result: Each symbol knows all locations where it's referenced

use crate::core::SymbolReference;
use crate::language::sysml::populator::{
    REL_REDEFINITION, REL_REFERENCE_SUBSETTING, REL_SPECIALIZATION, REL_SUBSETTING, REL_TYPING,
};
use crate::semantic::graph::RelationshipGraph;
use crate::semantic::symbol_table::SymbolTable;
use std::collections::HashMap;

pub struct ReferenceCollector<'a> {
    symbol_table: &'a mut SymbolTable,
    relationship_graph: &'a RelationshipGraph,
}

impl<'a> ReferenceCollector<'a> {
    pub fn new(
        symbol_table: &'a mut SymbolTable,
        relationship_graph: &'a RelationshipGraph,
    ) -> Self {
        Self {
            symbol_table,
            relationship_graph,
        }
    }

    /// Collect all references and populate the references field in symbols
    pub fn collect(&mut self) {
        let references_to_add = self.collect_references();

        for (target_name, refs) in references_to_add {
            // Try to resolve the target name (might be unqualified like "Wheel")
            // Collect keys first to avoid borrow conflicts
            let all_symbols: Vec<(String, String)> = self
                .symbol_table
                .all_symbols()
                .into_iter()
                .map(|(k, sym)| (k.clone(), sym.name().to_string()))
                .collect();

            let symbol_key = all_symbols
                .iter()
                .find(|(k, name)| k == &target_name || name == &target_name)
                .map(|(k, _)| k.clone());

            if let Some(key) = symbol_key {
                if let Some(symbol) = self.symbol_table.lookup_global_mut(&key) {
                    for reference in refs {
                        symbol.add_reference(reference);
                    }
                }
            }
        }
    }

    /// Collect references by examining relationship graphs
    fn collect_references(&self) -> HashMap<String, Vec<SymbolReference>> {
        let mut references: HashMap<String, Vec<SymbolReference>> = HashMap::new();

        for (key, symbol) in self.symbol_table.all_symbols() {
            let Some(span) = symbol.span() else {
                continue;
            };

            let Some(file) = symbol.source_file() else {
                continue;
            };

            // Collect all relationship targets for this symbol
            let targets = self.get_all_targets(symbol.qualified_name());

            for target in targets {
                references.entry(target).or_default().push(SymbolReference {
                    file: file.to_string(),
                    span: span.clone(),
                });
            }
        }

        eprintln!(
            "ReferenceCollector: Collected {} unique targets",
            references.len()
        );
        references
    }

    /// Get all relationship targets for a symbol
    fn get_all_targets(&self, qualified_name: &str) -> Vec<String> {
        let mut targets = Vec::new();

        // Typing relationship (: or "typed by")
        if let Some(target) = self
            .relationship_graph
            .get_one_to_one(REL_TYPING, qualified_name)
        {
            targets.push(target.clone());
        }

        // One-to-many relationships
        for rel_type in [
            REL_SPECIALIZATION,
            REL_REDEFINITION,
            REL_SUBSETTING,
            REL_REFERENCE_SUBSETTING,
        ] {
            if let Some(rel_targets) = self
                .relationship_graph
                .get_one_to_many(rel_type, qualified_name)
            {
                targets.extend(rel_targets.iter().cloned());
            }
        }

        targets
    }
}

#[cfg(test)]
mod tests;
