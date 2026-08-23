# Rust Style Guide

Source: Rust API Guidelines & standard `rustfmt` / `clippy` conventions.

## Naming
- Types, Traits, Enums: `UpperCamelCase` (e.g., `ClickerEngine`, `MouseButton`)
- Functions, Methods, Variables, Modules: `snake_case` (e.g., `perform_click`, `interval_ms`)
- Constants & Statics: `SCREAMING_SNAKE_CASE` (e.g., `DEFAULT_INTERVAL_MS`)
- File names: `snake_case.rs` (e.g., `clicker.rs`, `config.rs`)

## Syntax Choices
- Use explicit type imports over glob imports (`use enigo::{Button, Enigo, Mouse};`).
- Use pattern matching with `match` or `if let` for enum handling and error unwrapping.
- Prefer `derive` macros (`Clone`, `Debug`, `Serialize`, `Deserialize`, `PartialEq`) on domain models.
- Prefer `Arc<AtomicBool>` for thread cancellation flags instead of raw lock synchronization when boolean flags suffice.

## Idioms and Good Practices
- Use `std::time::Duration` for time intervals instead of raw integers in operational logic.
- Gracefully handle missing configuration files by creating defaults with clean error context.
- Keep the egui `update` function responsive and non-blocking using `try_recv()` on channels.

## Anti-Patterns
- `unwrap()` or `expect()` on user-facing I/O or background channels that can fail; always provide fallbacks or error logs.
- Busy waiting (`while condition {}`) without thread sleep or condition variables.
- Direct platform API calls bypassing cross-platform wrapper crates unless gated behind `#[cfg(...)]`.
