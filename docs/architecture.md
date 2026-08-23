# Architecture

## System Shape
A single-binary cross-platform desktop utility in Rust featuring an immediate-mode GUI (`eframe`/`egui`) with decoupled worker threads for mouse simulation (`enigo`) and global hotkey interception (`global_hotkey`).

## Layers & Modules
- `src/main.rs` — Application entry point, window initialization, runtime options.
- `src/app.rs` — Presentation layer (`eframe::App` implementation, UI controls, hotkey polling). Depends on `config`, `clicker`, `hotkey`.
- `src/clicker.rs` — Background clicking engine. Spawns dedicated thread for mouse automation via `enigo`. Must not block the UI thread.
- `src/hotkey.rs` — Cross-platform global hotkey management (`global_hotkey`). Delivers events to UI thread via non-blocking polling channel.
- `src/config.rs` — Pure configuration data structures, serialization (`serde`), and file persistence (`toml`, `directories`). No UI dependencies.

## Dependency Rules
- `config` must not depend on `app`, `clicker`, `hotkey`, or `eframe`.
- `clicker` must not depend on `app` or `eframe`.
- UI (`app.rs`) communicates with `clicker.rs` exclusively through thread-safe primitives (`Arc<AtomicBool>`, parameter structs) and does not perform synchronous heavy work on the main loop.
- All file system I/O for settings occurs through user configuration directories (`directories::ProjectDirs`).

## Data Flow
1. User adjusts settings or triggers hotkey in UI (`app.rs`).
2. UI updates runtime state or saves persistent config (`config.rs`).
3. Clicker engine (`clicker.rs`) receives start signal, running mouse loop on background thread until stopped.
4. Hotkey manager (`hotkey.rs`) catches global keypresses and notifies UI loop to toggle clicker state.

## Structural Constraints
- Never execute mouse simulation clicks on the main GUI thread (prevents UI freezing).
- Never block in a busy loop when waiting between click intervals; use sleep timers.
- Never hardcode Windows-only or macOS-only APIs in shared logic; rely on cross-platform abstractions (`enigo`, `global_hotkey`).
