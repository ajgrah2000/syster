use crate::core::constants::SUPPORTED_EXTENSIONS;
use crate::language::sysml::syntax::SysMLFile;
use crate::semantic::Workspace;
use from_pest::FromPest;
use pest::Parser;
use rayon::prelude::*;
use std::fs;
use std::path::PathBuf;

/// Loads the standard library from /sysml.lib/ at startup
pub struct StdLibLoader {
    stdlib_path: PathBuf,
}

impl StdLibLoader {
    pub fn new() -> Self {
        Self {
            stdlib_path: PathBuf::from("sysml.library"),
        }
    }

    pub fn with_path(path: PathBuf) -> Self {
        Self { stdlib_path: path }
    }

    /// Loads the SysML standard library into the workspace.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The stdlib directory cannot be read
    /// - File collection fails
    ///
    /// Note: Individual file parse failures are logged but do not cause the load to fail.
    pub fn load(&self, workspace: &mut Workspace) -> Result<(), String> {
        if !self.stdlib_path.exists() || !self.stdlib_path.is_dir() {
            return Ok(());
        }

        // Collect all file paths first
        let file_paths = self.collect_file_paths(&self.stdlib_path)?;

        // Parse files in parallel
        let results: Vec<_> = file_paths
            .par_iter()
            .map(|path| (path, self.parse_file(path)))
            .collect();

        // Add successfully parsed files and track failures
        let mut failed_files = Vec::new();
        for (path, result) in results {
            match result {
                Ok((path, file)) => {
                    workspace.add_file(path, file);
                }
                Err(e) => {
                    failed_files.push((path.clone(), e));
                }
            }
        }

        workspace.mark_stdlib_loaded();

        // Log failures but don't error (stdlib may have incomplete files during development)
        #[cfg(test)]
        if !failed_files.is_empty() {
            eprintln!(
                "Warning: {} files failed to parse during stdlib load:",
                failed_files.len()
            );
            for (path, err) in &failed_files {
                eprintln!("  - {}: {}", path.display(), err);
            }
        }

        Ok(())
    }

    fn collect_file_paths(&self, dir: &PathBuf) -> Result<Vec<PathBuf>, String> {
        let mut paths = Vec::new();
        Self::collect_recursive(dir, &mut paths)?;
        Ok(paths)
    }

    fn collect_recursive(dir: &PathBuf, paths: &mut Vec<PathBuf>) -> Result<(), String> {
        let entries = fs::read_dir(dir)
            .map_err(|e| format!("Failed to read directory {}: {}", dir.display(), e))?;

        for entry in entries {
            let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
            let path = entry.path();

            if path.is_dir() {
                Self::collect_recursive(&path, paths)?;
            } else if path.is_file()
                && path
                    .extension()
                    .and_then(|e| e.to_str())
                    .is_some_and(|ext| SUPPORTED_EXTENSIONS.contains(&ext))
            {
                paths.push(path);
            }
        }

        Ok(())
    }

    fn parse_file(&self, path: &PathBuf) -> Result<(PathBuf, SysMLFile), String> {
        let content = fs::read_to_string(path)
            .map_err(|e| format!("Failed to read {}: {}", path.display(), e))?;

        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .ok_or_else(|| format!("Invalid file extension for {}", path.display()))?;

        match ext {
            "sysml" => {
                let mut pairs =
                    crate::parser::SysMLParser::parse(crate::parser::sysml::Rule::model, &content)
                        .map_err(|e| format!("Parse error in {}: {}", path.display(), e))?;

                let file = SysMLFile::from_pest(&mut pairs)
                    .map_err(|e| format!("AST error in {}: {:?}", path.display(), e))?;

                Ok((path.clone(), file))
            }
            "kerml" => {
                // TODO: Add KerML parser support - skip for now
                Ok((
                    path.clone(),
                    SysMLFile {
                        namespace: None,
                        elements: vec![],
                    },
                ))
            }
            _ => Err(format!("Unsupported file extension: {}", ext)),
        }
    }
}

impl Default for StdLibLoader {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests;
