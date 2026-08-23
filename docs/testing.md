# Testing

## Stack
- Unit Testing: Rust built-in test runner (`cargo test`)
- Integration Testing: Standard `tests/` directory if needed

## Test Pyramid
- **Unit Tests**: Own configuration serialization/deserialization, time interval calculations, coordinate validation, hotkey string parsing.
- **Manual / Verification**: Real hardware mouse simulation & OS accessibility permissions (due to OS sandboxing and accessibility requirements on macOS/Windows).

## Unit Testing Conventions
- Place unit tests in a `tests` module at the bottom of the relevant file (`#[cfg(test)] mod tests { ... }`).
- Assert with `assert_eq!`, `assert!`, and test default configuration fallbacks.
- Test config roundtrip persistence (`to_string` -> `from_str`).

## What NOT to test in Automated Unit Tests
- Live OS mouse clicks simulating physical input in CI environments without virtual display servers.
- Global OS key listeners requiring active desktop session permissions.
