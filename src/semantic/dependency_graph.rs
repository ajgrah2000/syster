use crate::core::events::EventEmitter;
use crate::semantic::events::DependencyEvent;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

/// Tracks import dependencies between files for smart invalidation
#[derive(Debug)]
pub struct DependencyGraph {
    // Map from file -> files it depends on (imports)
    dependencies: HashMap<PathBuf, HashSet<PathBuf>>,
    // Map from file -> files that depend on it (reverse index)
    dependents: HashMap<PathBuf, HashSet<PathBuf>>,
    // Event emitter for dependency changes
    events: EventEmitter<DependencyEvent, DependencyGraph>,
}

impl Default for DependencyGraph {
    fn default() -> Self {
        Self::new()
    }
}

impl DependencyGraph {
    pub fn new() -> Self {
        Self {
            dependencies: HashMap::new(),
            dependents: HashMap::new(),
            events: EventEmitter::new(),
        }
    }

    /// Subscribes a listener to dependency graph events
    pub fn subscribe<F>(&mut self, listener: F)
    where
        F: Fn(&DependencyEvent, &mut DependencyGraph) + Send + Sync + 'static,
    {
        self.events.subscribe(listener);
    }

    /// Adds a dependency: `from` imports `to`
    pub fn add_dependency(&mut self, from: &Path, to: &Path) {
        self.dependencies
            .entry(from.to_path_buf())
            .or_default()
            .insert(to.to_path_buf());

        self.dependents
            .entry(to.to_path_buf())
            .or_default()
            .insert(from.to_path_buf());

        // Emit event
        let events = std::mem::take(&mut self.events);
        self.events = events.emit(
            DependencyEvent::DependencyAdded {
                from: from.to_path_buf(),
                to: to.to_path_buf(),
            },
            self,
        );
    }

    /// Returns all files that `file` directly depends on
    pub fn get_dependencies(&self, file: &PathBuf) -> Vec<PathBuf> {
        self.dependencies
            .get(file)
            .map(|set| set.iter().cloned().collect())
            .unwrap_or_default()
    }

    /// Returns all files that directly depend on `file`
    pub fn get_dependents(&self, file: &PathBuf) -> Vec<PathBuf> {
        self.dependents
            .get(file)
            .map(|set| set.iter().cloned().collect())
            .unwrap_or_default()
    }

    /// Returns all files transitively affected if `file` changes
    pub fn get_all_affected(&self, file: &PathBuf) -> Vec<PathBuf> {
        let mut affected = HashSet::new();
        let mut to_visit = vec![file.clone()];
        let mut visited = HashSet::new();

        while let Some(current) = to_visit.pop() {
            if !visited.insert(current.clone()) {
                continue; // Already visited, skip to avoid infinite loops
            }

            // Get all files that depend on current
            if let Some(deps) = self.dependents.get(&current) {
                for dep in deps {
                    if dep != file {
                        // Don't include the original file
                        affected.insert(dep.clone());
                    }
                    if !visited.contains(dep) {
                        to_visit.push(dep.clone());
                    }
                }
            }
        }

        affected.into_iter().collect()
    }

    /// Checks if `file` is part of a circular dependency
    pub fn has_circular_dependency(&self, file: &PathBuf) -> bool {
        let mut visited = HashSet::new();
        let mut rec_stack = HashSet::new();
        self.has_cycle_dfs(file, &mut visited, &mut rec_stack)
    }

    fn has_cycle_dfs(
        &self,
        file: &PathBuf,
        visited: &mut HashSet<PathBuf>,
        rec_stack: &mut HashSet<PathBuf>,
    ) -> bool {
        if rec_stack.contains(file) {
            return true; // Found a cycle
        }

        if visited.contains(file) {
            return false; // Already checked this path
        }

        visited.insert(file.clone());
        rec_stack.insert(file.clone());

        // Check all dependencies
        if let Some(deps) = self.dependencies.get(file) {
            for dep in deps {
                if self.has_cycle_dfs(dep, visited, rec_stack) {
                    return true;
                }
            }
        }

        rec_stack.remove(file);
        false
    }

    /// Removes all dependencies for a file (e.g., when file is deleted)
    pub fn remove_file(&mut self, file: &PathBuf) {
        // Remove file's dependencies
        if let Some(deps) = self.dependencies.remove(file) {
            for dep in deps {
                if let Some(dep_set) = self.dependents.get_mut(&dep) {
                    dep_set.remove(file);
                }
            }
        }

        // Remove file from dependents
        if let Some(deps) = self.dependents.remove(file) {
            for dep in deps {
                if let Some(dep_set) = self.dependencies.get_mut(&dep) {
                    dep_set.remove(file);
                }
            }
        }

        // Emit event
        let events = std::mem::take(&mut self.events);
        self.events = events.emit(DependencyEvent::FileRemoved { path: file.clone() }, self);
    }

    /// Returns the total number of tracked dependencies
    pub fn dependencies_count(&self) -> usize {
        self.dependencies.values().map(|set| set.len()).sum()
    }
}

#[cfg(test)]
mod tests;
