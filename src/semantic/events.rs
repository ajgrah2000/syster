//! # Semantic Event System
//!
//! Defines all event types emitted by semantic analysis components.
//!
//! These events enable observability and decoupled logic for:
//! - LSP implementations (subscribe to changes)
//! - Testing (verify events fire without side effects)
//! - Extensibility (add listeners without modifying core code)
//!
//! ## Event Types
//!
//! - **WorkspaceEvent**: File additions, updates, removals
//! - **DependencyEvent**: Dependency graph changes
//!
//! Future additions:
//! - SymbolTableEvent: Symbol insertions, import additions
//! - RelationshipEvent: Relationship graph changes
//! - AnalyzerEvent: Semantic error detection

use crate::core::events::Event;
use std::path::PathBuf;

/// Events emitted by the workspace during file operations
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WorkspaceEvent {
    /// A file was added to the workspace
    FileAdded { path: PathBuf },

    /// A file's content was updated
    FileUpdated { path: PathBuf },

    /// A file was removed from the workspace
    FileRemoved { path: PathBuf },
}

impl Event for WorkspaceEvent {}

/// Events emitted by the dependency graph during updates
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DependencyEvent {
    /// A dependency was added between two files
    DependencyAdded {
        /// The file that imports another file
        from: PathBuf,
        /// The file being imported
        to: PathBuf,
    },

    /// A file and all its dependencies were removed
    FileRemoved { path: PathBuf },
}

impl Event for DependencyEvent {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_workspace_event_creation() {
        let path = PathBuf::from("test.sysml");

        let added = WorkspaceEvent::FileAdded { path: path.clone() };
        let updated = WorkspaceEvent::FileUpdated { path: path.clone() };
        let removed = WorkspaceEvent::FileRemoved { path: path.clone() };

        assert!(matches!(added, WorkspaceEvent::FileAdded { .. }));
        assert!(matches!(updated, WorkspaceEvent::FileUpdated { .. }));
        assert!(matches!(removed, WorkspaceEvent::FileRemoved { .. }));
    }

    #[test]
    fn test_workspace_event_equality() {
        let path = PathBuf::from("test.sysml");

        let event1 = WorkspaceEvent::FileUpdated { path: path.clone() };
        let event2 = WorkspaceEvent::FileUpdated { path: path.clone() };

        assert_eq!(event1, event2);
    }

    #[test]
    fn test_dependency_event_creation() {
        let from = PathBuf::from("app.sysml");
        let to = PathBuf::from("base.sysml");

        let added = DependencyEvent::DependencyAdded {
            from: from.clone(),
            to: to.clone(),
        };
        let removed = DependencyEvent::FileRemoved { path: from.clone() };

        assert!(matches!(added, DependencyEvent::DependencyAdded { .. }));
        assert!(matches!(removed, DependencyEvent::FileRemoved { .. }));
    }

    #[test]
    fn test_dependency_event_equality() {
        let from = PathBuf::from("app.sysml");
        let to = PathBuf::from("base.sysml");

        let event1 = DependencyEvent::DependencyAdded {
            from: from.clone(),
            to: to.clone(),
        };
        let event2 = DependencyEvent::DependencyAdded {
            from: from.clone(),
            to: to.clone(),
        };

        assert_eq!(event1, event2);
    }
}
