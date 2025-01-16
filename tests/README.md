# Testing Documentation

## Overview
This directory contains the test suite for External App Sync. The tests are organized into:
- Unit tests (within source files)
- Integration tests (in `tests/` directory)
- Common test utilities (`tests/common/`)

## Running Tests

### All Tests
```bash
cargo test
```

### Specific Test
```bash
cargo test test_name
```

### With Output
```bash
cargo test -- --nocapture
```

## Test Environment Setup
The test suite uses the following environment variables:
- `TEST_EXTERNAL_DRIVE_PATH`: Path to simulate external drive location (optional)
- `TEST_APPS_DIR`: Path to simulate Applications directory (optional)

## Writing Tests
1. Unit tests should be placed in the same file as the code they're testing
2. Integration tests go in the `tests/` directory
3. Use the common test utilities from `tests/common/mod.rs`
4. Use `serial_test` attribute for tests that can't run in parallel
5. Use `pretty_assertions` for better failure messages

## Test Utilities
- `setup_test_dir()`: Creates temporary test directories
- `setup_test_app_structure()`: Creates test application structure
- `cleanup_test_resources()`: Cleans up test resources
