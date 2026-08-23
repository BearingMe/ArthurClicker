# Testing

## Stack
- Unit & Integration Testing: Rust built-in test runner (`cargo test`)
- End-to-End Testing: `tests/e2e_test.rs` integration suite

## Test Pyramid
- **Unit Tests**: Configuration serialization/deserialization, default fallbacks, time interval math in `src/config.rs`.
- **E2E / Integration Tests (`tests/e2e_test.rs`)**:
  - `test_e2e_config_lifecycle`: End-to-end config atomic save-to-temp, rename, file verification, and deserialization.
  - `test_e2e_clicker_lifecycle_and_bounded_execution`: Worker engine spawning, bounded click repeat count completion, and auto-halt.
  - `test_e2e_clicker_instant_stop`: Start indefinite auto-clicking and verify sub-150ms responsive stop cancellation.
  - `test_e2e_hotkey_parsing_and_registration_pipeline`: Hotkey string-to-code mapping, modifiers combination parsing, and service registration.
- **Manual Verification**: Interactive UI rendering, accessibility permission elevation on macOS (`AXIsProcessTrusted`), and multi-monitor coordinate picking.

## Unit & Integration Testing Conventions
- Place unit tests in an inline `tests` module at the bottom of the relevant file (`#[cfg(test)] mod tests { ... }`).
- Place end-to-end multi-module integration tests in the `tests/` directory.
- Assert with `assert_eq!`, `assert!`, and test default configuration fallbacks.

## What NOT to test in Automated Unit Tests
- Real desktop window focus stealing in headless CI environments.
- OS-level system prompt accessibility dialogs.
