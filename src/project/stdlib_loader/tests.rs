use super::StdLibLoader;
use crate::core::constants::SUPPORTED_EXTENSIONS;
use crate::semantic::Workspace;
use std::path::PathBuf;

#[test]
fn test_stdlib_loader_creation() {
    let loader = StdLibLoader::new();
    assert_eq!(loader.stdlib_path, PathBuf::from("sysml.library"));

    let custom_loader = StdLibLoader::with_path(PathBuf::from("/custom/path"));
    assert_eq!(custom_loader.stdlib_path, PathBuf::from("/custom/path"));
}

#[test]
fn test_load_missing_directory() {
    let loader = StdLibLoader::with_path(PathBuf::from("/nonexistent/path"));
    let mut workspace = Workspace::new();

    let result = loader.load(&mut workspace);
    assert!(
        result.is_ok(),
        "Loading missing directory should succeed gracefully"
    );
    assert!(
        !workspace.has_stdlib(),
        "Stdlib should not be marked as loaded"
    );
}

#[test]
fn test_load_actual_stdlib() {
    let loader = StdLibLoader::new();
    assert!(
        loader.stdlib_path.exists(),
        "sysml.library/ must exist for this test"
    );

    let mut workspace = Workspace::new();
    let result = loader.load(&mut workspace);

    assert!(result.is_ok(), "Loading stdlib should succeed");
    assert!(workspace.has_stdlib(), "Stdlib should be marked as loaded");
}

#[test]
fn test_collect_file_paths() {
    let loader = StdLibLoader::new();
    assert!(
        loader.stdlib_path.exists(),
        "sysml.library/ must exist for this test"
    );

    let paths = loader.collect_file_paths(&loader.stdlib_path);
    assert!(paths.is_ok(), "Should collect paths successfully");

    let paths = paths.unwrap();
    // Should find at least some .sysml files
    let sysml_files: Vec<_> = paths
        .iter()
        .filter(|p| p.extension().and_then(|e| e.to_str()) == Some("sysml"))
        .collect();

    assert!(
        !sysml_files.is_empty(),
        "Should find at least one .sysml file in stdlib"
    );

    // Verify we're finding the exact number of files (stdlib has 58 .sysml files)
    assert_eq!(
        sysml_files.len(),
        58,
        "Expected exactly 58 .sysml files in stdlib, found {}",
        sysml_files.len()
    );
}

#[test]
fn test_supported_extensions_only() {
    let loader = StdLibLoader::new();
    assert!(
        loader.stdlib_path.exists(),
        "sysml.library/ must exist for this test"
    );

    let paths = loader.collect_file_paths(&loader.stdlib_path);
    assert!(paths.is_ok());

    // All collected paths should have supported extensions
    for path in paths.unwrap() {
        let ext = path.extension().and_then(|e| e.to_str());
        assert!(
            ext.is_some_and(|e| SUPPORTED_EXTENSIONS.contains(&e)),
            "Path {:?} has unsupported extension",
            path
        );
    }
}

#[test]
fn test_parallel_loading() {
    let loader = StdLibLoader::new();
    assert!(
        loader.stdlib_path.exists(),
        "sysml.library/ must exist for this test"
    );

    let mut workspace = Workspace::new();
    // Load once
    let result1 = loader.load(&mut workspace);
    assert!(result1.is_ok());

    // Should be able to load multiple times (idempotent)
    let mut workspace2 = Workspace::new();
    let result2 = loader.load(&mut workspace2);
    assert!(result2.is_ok());
}

#[test]
fn test_files_added_to_workspace() {
    let loader = StdLibLoader::new();
    assert!(
        loader.stdlib_path.exists(),
        "sysml.library/ must exist for this test"
    );

    let mut workspace = Workspace::new();
    let result = loader.load(&mut workspace);
    assert!(result.is_ok());

    // Verify files were added to workspace
    let file_count = workspace.file_paths().count();
    assert_eq!(
        file_count, 58,
        "Expected exactly 58 successfully parsed files, found {}",
        file_count
    );

    // Verify stdlib flag is set
    assert!(workspace.has_stdlib());
}

#[test]
fn test_kerml_files_handled() {
    let loader = StdLibLoader::new();
    assert!(
        loader.stdlib_path.exists(),
        "sysml.library/ must exist for this test"
    );

    let paths = loader.collect_file_paths(&loader.stdlib_path).unwrap();

    let kerml_count = paths
        .iter()
        .filter(|p| p.extension().and_then(|e| e.to_str()) == Some("kerml"))
        .count();

    // There are ~36 .kerml files in stdlib
    assert!(
        kerml_count == 36,
        "Expected 36 .kerml files, found {}",
        kerml_count
    );

    // Note: KerML parsing not yet implemented, but files should be collected
}

#[test]
fn test_parse_failures_logged() {
    let loader = StdLibLoader::new();
    assert!(
        loader.stdlib_path.exists(),
        "sysml.library/ must exist for this test"
    );

    let mut workspace = Workspace::new();

    // This will print warnings about parse failures to stderr (visible with --nocapture)
    let result = loader.load(&mut workspace);
    assert!(
        result.is_ok(),
        "Load should succeed even with parse failures"
    );

    // Some files should parse successfully
    assert!(
        workspace.file_paths().count() > 0,
        "At least some files should parse successfully"
    );
}
