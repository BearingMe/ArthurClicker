use crate::config::{ClickTypeChoice, CursorModeChoice, MouseButtonChoice, RepeatModeChoice};
use enigo::{Button, Coordinate, Direction, Enigo, Mouse, Settings};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::thread::{self, JoinHandle};
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct ClickerTaskConfig {
    pub interval: Duration,
    pub mouse_button: MouseButtonChoice,
    pub click_type: ClickTypeChoice,
    pub repeat_mode: RepeatModeChoice,
    pub cursor_mode: CursorModeChoice,
}

/// RAII Drop Guard ensuring `running` atomic flag is reset on normal exit or panic
struct RunningSentinel(Arc<AtomicBool>);

impl Drop for RunningSentinel {
    fn drop(&mut self) {
        self.0.store(false, Ordering::SeqCst);
    }
}

pub struct ClickerEngine {
    running: Arc<AtomicBool>,
    click_count: Arc<AtomicU64>,
    worker_handle: Option<JoinHandle<()>>,
}

impl Default for ClickerEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl ClickerEngine {
    pub fn new() -> Self {
        Self {
            running: Arc::new(AtomicBool::new(false)),
            click_count: Arc::new(AtomicU64::new(0)),
            worker_handle: None,
        }
    }

    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }

    pub fn get_click_count(&self) -> u64 {
        self.click_count.load(Ordering::Relaxed)
    }

    pub fn start(&mut self, config: ClickerTaskConfig) {
        if self.is_running() {
            return;
        }

        // Clean up previous thread handle if still present
        if let Some(handle) = self.worker_handle.take() {
            let _ = handle.join();
        }

        self.running.store(true, Ordering::SeqCst);
        self.click_count.store(0, Ordering::Relaxed);

        let running_flag = Arc::clone(&self.running);
        let click_counter = Arc::clone(&self.click_count);

        let handle = thread::spawn(move || {
            let _guard = RunningSentinel(Arc::clone(&running_flag));

            let mut enigo = match Enigo::new(&Settings::default()) {
                Ok(e) => e,
                Err(err) => {
                    eprintln!("Failed to initialize Enigo: {:?}", err);
                    return;
                }
            };

            let mut performed_clicks = 0u32;
            let target_limit = match config.repeat_mode {
                RepeatModeChoice::Count(n) => Some(n),
                RepeatModeChoice::Indefinite => None,
            };

            while running_flag.load(Ordering::SeqCst) {
                if let Some(limit) = target_limit {
                    if performed_clicks >= limit {
                        break;
                    }
                }

                // If fixed cursor mode, position mouse before clicking
                if let CursorModeChoice::Fixed { x, y } = config.cursor_mode {
                    let _ = enigo.move_mouse(x, y, Coordinate::Abs);
                }

                let btn = match config.mouse_button {
                    MouseButtonChoice::Left => Button::Left,
                    MouseButtonChoice::Middle => Button::Middle,
                    MouseButtonChoice::Right => Button::Right,
                };

                let iterations = match config.click_type {
                    ClickTypeChoice::Single => 1,
                    ClickTypeChoice::Double => 2,
                    ClickTypeChoice::Triple => 3,
                };

                // Sub-click delay proportional to interval but within standard OS double-click window
                let sub_click_delay = Duration::from_millis(35).min(config.interval / 2);

                for i in 0..iterations {
                    if !running_flag.load(Ordering::SeqCst) {
                        break;
                    }
                    if i > 0 && sub_click_delay > Duration::ZERO {
                        thread::sleep(sub_click_delay);
                    }
                    let _ = enigo.button(btn, Direction::Click);
                }

                performed_clicks += 1;
                click_counter.fetch_add(1, Ordering::Relaxed);

                // Responsive sleep chunking to allow instant stop
                let mut remaining_sleep = config.interval;
                let step = Duration::from_millis(10);
                while remaining_sleep > Duration::ZERO && running_flag.load(Ordering::SeqCst) {
                    let to_sleep = remaining_sleep.min(step);
                    thread::sleep(to_sleep);
                    remaining_sleep = remaining_sleep.saturating_sub(to_sleep);
                }
            }
        });

        self.worker_handle = Some(handle);
    }

    pub fn stop(&mut self) {
        self.running.store(false, Ordering::SeqCst);
        if let Some(handle) = self.worker_handle.take() {
            let _ = handle.join();
        }
    }

    pub fn get_current_cursor_pos() -> Option<(i32, i32)> {
        Enigo::new(&Settings::default())
            .ok()
            .and_then(|e| e.location().ok())
    }
}

impl Drop for ClickerEngine {
    fn drop(&mut self) {
        self.stop();
    }
}
