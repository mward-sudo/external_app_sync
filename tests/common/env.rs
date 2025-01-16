use std::env;
use std::path::PathBuf;

/// Get the test external drive path from environment or use a default
pub fn get_test_external_drive_path() -> PathBuf {
    env::var("TEST_EXTERNAL_DRIVE_PATH")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            let temp_dir = env::temp_dir();
            temp_dir.join("external_app_sync_test_external")
        })
}

/// Get the test applications directory path from environment or use a default
pub fn get_test_apps_dir() -> PathBuf {
    env::var("TEST_APPS_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            let temp_dir = env::temp_dir();
            temp_dir.join("external_app_sync_test_apps")
        })
}

/// Initialize test environment
pub fn init_test_env() {
    if env::var("TEST_EXTERNAL_DRIVE_PATH").is_err() {
        let path = get_test_external_drive_path();
        std::fs::create_dir_all(&path).ok();
        env::set_var("TEST_EXTERNAL_DRIVE_PATH", path);
    }

    if env::var("TEST_APPS_DIR").is_err() {
        let path = get_test_apps_dir();
        std::fs::create_dir_all(&path).ok();
        env::set_var("TEST_APPS_DIR", path);
    }
}
