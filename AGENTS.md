# AGENTS.md

## Project Overview
Arthur Clicker is a lightweight, cross-platform desktop auto-clicker built in Rust using `eframe`/`egui`, `enigo`, and `global_hotkey`.

## Tech Stack
- Rust 2021 / 2024
- GUI: `eframe` / `egui` (0.36)
- Input Simulation: `enigo` (0.6)
- Global Hotkeys: `global-hotkey` (0.8)
- Serialization & Storage: `serde`, `toml`, `directories`

## Commands
```bash
# Build (Debug)
cargo build

# Run application
cargo run

# Build release binary
cargo build --release

# Run unit tests
cargo test

# Check code / lint
cargo check
cargo clippy
```

## Repository Structure & Architecture
- `src/main.rs`: Entry point and `eframe` window launcher.
- `src/app.rs`: UI interface, egui layout, and state orchestration.
- `src/clicker.rs`: Background clicking engine using `enigo`.
- `src/hotkey.rs`: Cross-platform global hotkey registration and event dispatching.
- `src/config.rs`: Settings definition and TOML persistence.
- `docs/`: Reference architecture, styleguide, design, and testing guides.

**Architectural Boundaries:**
- UI must remain reactive; no blocking click loops or synchronous OS waits on the main thread.
- Mouse simulation code is isolated to `clicker.rs`.
- Settings storage is isolated to `config.rs`.

## Rules & Conventions
- Adhere to `docs/styleguides/rust.md`.
- Consult `docs/architecture.md` before adding new modules or thread boundaries.
- Consult `docs/design.md` for UI styling, spacing, and color schemes.
- Consult `docs/testing.md` for test coverage conventions.

## Never
- Never introduce platform-exclusive APIs without cross-platform fallbacks or gating.
- Never busy-wait in background loops; always sleep.
- Never block UI update calls.
- Never commit user configuration files or build artifacts.

## Definition of Done
Work is complete ONLY when:
- `cargo check` and `cargo test` pass cleanly.
- Code compiles without unresolved warnings or errors.
- UI runs smoothly without UI thread blocking.
