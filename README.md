# Arthur Clicker

<p align="center">
  <img src="assets/icon.png" width="128" alt="Arthur Clicker Logo" />
</p>

Arthur Clicker is a lightweight, cross-platform desktop auto-clicker built in Rust using [`eframe`](https://github.com/emilk/egui), [`enigo`](https://github.com/enigo-rs/enigo), and [`global-hotkey`](https://github.com/tauri-apps/global-hotkey).

## Features

- **Adjustable Click Interval**: Hours, Minutes, Seconds, and Milliseconds.
- **Mouse Button Selection**: Left, Middle, Right.
- **Click Types**: Single, Double, Triple clicks.
- **Repeat Modes**: Repeat indefinitely or repeat a fixed number of times.
- **Cursor Modes**: Current cursor location or fixed target coordinates (with a 3-second hover picker).
- **Global Hotkey**: Configurable start/stop global shortcut (default `F6`) with modifier support (`Ctrl`, `Alt`, `Shift`, `Cmd`).
- **Responsive Dark UI**: Centered, adaptive interface that scales cleanly across resolutions.
- **Settings Persistence**: Atomic TOML configuration saved across launches.

## Building and Running

### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) (2021 edition)

### Native Build (macOS / Linux / Windows)
```bash
# Run in development mode
cargo run

# Build optimized release binary
cargo build --release
```

### Windows Cross-Compilation via `cargo-xwin` (from macOS / Linux)

To compile a native Windows `.exe` (`x86_64-pc-windows-msvc`) without needing a Windows machine:

1. **Install `cargo-xwin` and the Windows MSVC target:**
   ```bash
   cargo install cargo-xwin
   rustup target add x86_64-pc-windows-msvc
   ```

2. **Build the release `.exe`:**
   ```bash
   cargo xwin build --release --target x86_64-pc-windows-msvc
   ```

3. The generated executable will be located at:
   ```text
   target/x86_64-pc-windows-msvc/release/arthur_clicker.exe
   ```

## Running Tests

```bash
# Run all unit and integration/e2e tests
cargo test

# Run linter checks
cargo clippy --all-targets
```
