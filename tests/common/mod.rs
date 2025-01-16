pub mod env;
pub use env::*;

use std::path::PathBuf;
use tempfile::TempDir;

/// Test helper to create a temporary directory for tests
pub fn setup_test_dir() -> (TempDir, PathBuf) {
    let temp_dir = tempfile::tempdir().expect("Failed to create temp directory");
    let temp_path = temp_dir.path().to_path_buf();
    (temp_dir, temp_path)
}

/// Helper function to create a test application directory structure
pub fn setup_test_app_structure(base_dir: &PathBuf, app_name: &str) -> PathBuf {
    let app_path = base_dir.join(format!("{}.app", app_name));
    std::fs::create_dir_all(&app_path).expect("Failed to create test app directory");
    app_path
}

/// Clean up test resources
pub fn cleanup_test_resources(temp_dir: TempDir) {
    temp_dir.close().expect("Failed to clean up temp directory");
}
