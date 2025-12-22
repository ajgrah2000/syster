use crate::semantic::types::SemanticError;
use crate::syntax::kerml::KerMLFile;

use crate::semantic::adapters::KermlAdapter;

impl<'a> KermlAdapter<'a> {
    pub fn populate(&mut self, file: &KerMLFile) -> Result<(), Vec<SemanticError>> {
        // Set root namespace if declared
        if let Some(namespace) = &file.namespace {
            self.visit_namespace(namespace);
        }

        // Process all top-level elements
        for element in &file.elements {
            self.visit_element(element);
        }

        // Exit root namespace if it was entered
        if file.namespace.is_some() {
            self.exit_namespace();
        }

        // Return errors if any
        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(std::mem::take(&mut self.errors))
        }
    }
}
