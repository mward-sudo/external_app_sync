mod common;

use serial_test::serial;

#[tokio::test]
#[serial]
async fn test_basic_setup() {
    // Initialize test environment
    common::init_test_env();
    
    let (temp_dir, temp_path) = common::setup_test_dir();
    
    // Create a test app structure
    let app_name = "TestApp";
    let app_path = common::setup_test_app_structure(&temp_path, app_name);
    
    assert!(app_path.exists(), "Test app directory should exist");
    assert!(app_path.is_dir(), "Test app path should be a directory");
    
    common::cleanup_test_resources(temp_dir);
}
