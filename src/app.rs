use crate::clicker::{ClickerEngine, ClickerTaskConfig};
use crate::config::{
    AppConfig, ClickTypeChoice, CursorModeChoice, MouseButtonChoice, RepeatModeChoice,
};
use crate::hotkey::HotkeyService;
use eframe::egui::{self, Color32, RichText, Vec2};
use std::time::{Duration, Instant};

pub struct ArthurClickerApp {
    config: AppConfig,
    clicker: ClickerEngine,
    hotkey_service: HotkeyService,
    hotkey_status_message: Option<String>,
    repeat_count_input: u32,
    fixed_x_input: i32,
    fixed_y_input: i32,
    capturing_hotkey: bool,
    picking_location_countdown: Option<Instant>,
}

impl ArthurClickerApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let config = AppConfig::load();
        let mut hotkey_service = HotkeyService::new();
        let hotkey_registered = hotkey_service.register(&config.hotkey);

        let initial_repeat_count = match config.repeat_mode {
            RepeatModeChoice::Count(n) => n,
            RepeatModeChoice::Indefinite => 100,
        };

        let (fixed_x, fixed_y) = match config.cursor_mode {
            CursorModeChoice::Fixed { x, y } => (x, y),
            CursorModeChoice::CurrentPosition => (0, 0),
        };

        Self {
            config,
            clicker: ClickerEngine::new(),
            hotkey_service,
            hotkey_status_message: if hotkey_registered {
                None
            } else {
                Some("Hotkey registration failed (grant Accessibility permissions)".to_string())
            },
            repeat_count_input: initial_repeat_count,
            fixed_x_input: fixed_x,
            fixed_y_input: fixed_y,
            capturing_hotkey: false,
            picking_location_countdown: None,
        }
    }

    fn toggle_clicking(&mut self, is_ui_button_trigger: bool) {
        if self.clicker.is_running() {
            self.clicker.stop();
        } else {
            // When starting via UI button, add an initial grace period (250ms)
            // so the cursor can move away and avoid clicking the START/STOP button itself
            let initial_delay = if is_ui_button_trigger {
                Duration::from_millis(250)
            } else {
                Duration::ZERO
            };

            let task_config = ClickerTaskConfig {
                interval: self.config.get_interval(),
                mouse_button: self.config.mouse_button,
                click_type: self.config.click_type,
                repeat_mode: self.config.repeat_mode,
                cursor_mode: self.config.cursor_mode,
                initial_delay,
            };
            self.clicker.start(task_config);
        }
    }
}

impl eframe::App for ArthurClickerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut config_changed = false;

        // Repaint periodically when running or picking location
        if self.clicker.is_running() || self.picking_location_countdown.is_some() {
            ctx.request_repaint_after(Duration::from_millis(33)); // ~30 FPS capped
        }

        // Global hotkey listening (hotkey trigger needs no artificial button delay)
        if self.hotkey_service.poll_event() {
            self.toggle_clicking(false);
        }

        // Location picker countdown (allows placing cursor over external target window)
        if let Some(target_time) = self.picking_location_countdown {
            let now = Instant::now();
            if now >= target_time {
                if let Some((x, y)) = ClickerEngine::get_current_cursor_pos() {
                    self.fixed_x_input = x;
                    self.fixed_y_input = y;
                    self.config.cursor_mode = CursorModeChoice::Fixed { x, y };
                    config_changed = true;
                }
                self.picking_location_countdown = None;
            }
        }

        // Hotkey capture in UI
        if self.capturing_hotkey {
            ctx.input(|i| {
                let candidate_keys = [
                    (egui::Key::F1, "F1"),
                    (egui::Key::F2, "F2"),
                    (egui::Key::F3, "F3"),
                    (egui::Key::F4, "F4"),
                    (egui::Key::F5, "F5"),
                    (egui::Key::F6, "F6"),
                    (egui::Key::F7, "F7"),
                    (egui::Key::F8, "F8"),
                    (egui::Key::F9, "F9"),
                    (egui::Key::F10, "F10"),
                    (egui::Key::F11, "F11"),
                    (egui::Key::F12, "F12"),
                    (egui::Key::Num0, "0"),
                    (egui::Key::Num1, "1"),
                    (egui::Key::Num2, "2"),
                    (egui::Key::Num3, "3"),
                    (egui::Key::Num4, "4"),
                    (egui::Key::Num5, "5"),
                    (egui::Key::Num6, "6"),
                    (egui::Key::Num7, "7"),
                    (egui::Key::Num8, "8"),
                    (egui::Key::Num9, "9"),
                    (egui::Key::A, "A"),
                    (egui::Key::B, "B"),
                    (egui::Key::C, "C"),
                    (egui::Key::D, "D"),
                    (egui::Key::E, "E"),
                    (egui::Key::F, "F"),
                    (egui::Key::G, "G"),
                    (egui::Key::H, "H"),
                    (egui::Key::I, "I"),
                    (egui::Key::J, "J"),
                    (egui::Key::K, "K"),
                    (egui::Key::L, "L"),
                    (egui::Key::M, "M"),
                    (egui::Key::N, "N"),
                    (egui::Key::O, "O"),
                    (egui::Key::P, "P"),
                    (egui::Key::Q, "Q"),
                    (egui::Key::R, "R"),
                    (egui::Key::S, "S"),
                    (egui::Key::T, "T"),
                    (egui::Key::U, "U"),
                    (egui::Key::V, "V"),
                    (egui::Key::W, "W"),
                    (egui::Key::X, "X"),
                    (egui::Key::Y, "Y"),
                    (egui::Key::Z, "Z"),
                    (egui::Key::Space, "SPACE"),
                ];

                for (key, key_str) in candidate_keys {
                    if i.key_pressed(key) {
                        self.config.hotkey.key = key_str.to_string();
                        self.config.hotkey.modifiers.clear();
                        if i.modifiers.ctrl {
                            self.config.hotkey.modifiers.push("CTRL".to_string());
                        }
                        if i.modifiers.alt {
                            self.config.hotkey.modifiers.push("ALT".to_string());
                        }
                        if i.modifiers.shift {
                            self.config.hotkey.modifiers.push("SHIFT".to_string());
                        }
                        if i.modifiers.command {
                            self.config.hotkey.modifiers.push("CMD".to_string());
                        }

                        self.capturing_hotkey = false;
                        if self.hotkey_service.register(&self.config.hotkey) {
                            self.hotkey_status_message = None;
                        } else {
                            self.hotkey_status_message =
                                Some("Failed to register new hotkey".to_string());
                        }
                        config_changed = true;
                        break;
                    }
                }
                if i.key_pressed(egui::Key::Escape) {
                    self.capturing_hotkey = false;
                }
            });
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    ui.spacing_mut().item_spacing = Vec2::new(8.0, 10.0);

                    // Heading
                    ui.vertical_centered(|ui| {
                        ui.heading(RichText::new("Arthur Auto Clicker").size(18.0).strong());
                    });
                    ui.add_space(2.0);

                    // 1. Click Interval
                    egui::Frame::group(ui.style()).show(ui, |ui| {
                        ui.set_width(ui.available_width());
                        ui.vertical_centered(|ui| {
                            ui.label(RichText::new("Click Interval").strong());
                            ui.add_space(2.0);
                            ui.horizontal_wrapped(|ui| {
                                if ui
                                    .add(
                                        egui::DragValue::new(&mut self.config.hours)
                                            .speed(1)
                                            .range(0..=999)
                                            .suffix(" h"),
                                    )
                                    .changed()
                                {
                                    config_changed = true;
                                }
                                if ui
                                    .add(
                                        egui::DragValue::new(&mut self.config.minutes)
                                            .speed(1)
                                            .range(0..=59)
                                            .suffix(" m"),
                                    )
                                    .changed()
                                {
                                    config_changed = true;
                                }
                                if ui
                                    .add(
                                        egui::DragValue::new(&mut self.config.seconds)
                                            .speed(1)
                                            .range(0..=59)
                                            .suffix(" s"),
                                    )
                                    .changed()
                                {
                                    config_changed = true;
                                }
                                if ui
                                    .add(
                                        egui::DragValue::new(&mut self.config.milliseconds)
                                            .speed(5)
                                            .range(1..=999)
                                            .suffix(" ms"),
                                    )
                                    .changed()
                                {
                                    config_changed = true;
                                }
                            });
                        });
                    });

                    // 2. Click Options
                    egui::Frame::group(ui.style()).show(ui, |ui| {
                        ui.set_width(ui.available_width());
                        ui.vertical_centered(|ui| {
                            ui.label(RichText::new("Click Options").strong());
                            ui.add_space(2.0);
                            ui.horizontal_wrapped(|ui| {
                                ui.label("Button:");
                                egui::ComboBox::from_id_salt("mouse_button_combo")
                                    .selected_text(match self.config.mouse_button {
                                        MouseButtonChoice::Left => "Left",
                                        MouseButtonChoice::Middle => "Middle",
                                        MouseButtonChoice::Right => "Right",
                                    })
                                    .show_ui(ui, |ui| {
                                        if ui
                                            .selectable_value(
                                                &mut self.config.mouse_button,
                                                MouseButtonChoice::Left,
                                                "Left",
                                            )
                                            .clicked()
                                            || ui
                                                .selectable_value(
                                                    &mut self.config.mouse_button,
                                                    MouseButtonChoice::Middle,
                                                    "Middle",
                                                )
                                                .clicked()
                                            || ui
                                                .selectable_value(
                                                    &mut self.config.mouse_button,
                                                    MouseButtonChoice::Right,
                                                    "Right",
                                                )
                                                .clicked()
                                        {
                                            config_changed = true;
                                        }
                                    });

                                ui.add_space(8.0);

                                ui.label("Type:");
                                egui::ComboBox::from_id_salt("click_type_combo")
                                    .selected_text(match self.config.click_type {
                                        ClickTypeChoice::Single => "Single",
                                        ClickTypeChoice::Double => "Double",
                                        ClickTypeChoice::Triple => "Triple",
                                    })
                                    .show_ui(ui, |ui| {
                                        if ui
                                            .selectable_value(
                                                &mut self.config.click_type,
                                                ClickTypeChoice::Single,
                                                "Single",
                                            )
                                            .clicked()
                                            || ui
                                                .selectable_value(
                                                    &mut self.config.click_type,
                                                    ClickTypeChoice::Double,
                                                    "Double",
                                                )
                                                .clicked()
                                            || ui
                                                .selectable_value(
                                                    &mut self.config.click_type,
                                                    ClickTypeChoice::Triple,
                                                    "Triple",
                                                )
                                                .clicked()
                                        {
                                            config_changed = true;
                                        }
                                    });
                            });
                        });
                    });

                    // 3. Repeat Mode
                    egui::Frame::group(ui.style()).show(ui, |ui| {
                        ui.set_width(ui.available_width());
                        ui.vertical_centered(|ui| {
                            ui.label(RichText::new("Click Repeat").strong());
                            ui.add_space(2.0);
                            ui.horizontal_wrapped(|ui| {
                                let mut is_indefinite =
                                    matches!(self.config.repeat_mode, RepeatModeChoice::Indefinite);
                                if ui.radio_value(&mut is_indefinite, true, "Repeat until stopped").clicked() {
                                    self.config.repeat_mode = RepeatModeChoice::Indefinite;
                                    config_changed = true;
                                }

                                if ui.radio_value(&mut is_indefinite, false, "Repeat").clicked() {
                                    self.config.repeat_mode = RepeatModeChoice::Count(self.repeat_count_input);
                                    config_changed = true;
                                }

                                if !is_indefinite
                                    && ui
                                        .add(
                                            egui::DragValue::new(&mut self.repeat_count_input)
                                                .speed(1)
                                                .range(1..=100_000)
                                                .suffix(" times"),
                                        )
                                        .changed()
                                {
                                    self.config.repeat_mode =
                                        RepeatModeChoice::Count(self.repeat_count_input);
                                    config_changed = true;
                                }
                            });
                        });
                    });

                    // 4. Cursor Position
                    egui::Frame::group(ui.style()).show(ui, |ui| {
                        ui.set_width(ui.available_width());
                        ui.vertical_centered(|ui| {
                            ui.label(RichText::new("Cursor Position").strong());
                            ui.add_space(2.0);
                            let mut is_current_pos =
                                matches!(self.config.cursor_mode, CursorModeChoice::CurrentPosition);

                            ui.horizontal_wrapped(|ui| {
                                if ui.radio_value(&mut is_current_pos, true, "Current location").clicked() {
                                    self.config.cursor_mode = CursorModeChoice::CurrentPosition;
                                    config_changed = true;
                                }
                                if ui.radio_value(&mut is_current_pos, false, "Fixed coordinate").clicked() {
                                    self.config.cursor_mode = CursorModeChoice::Fixed {
                                        x: self.fixed_x_input,
                                        y: self.fixed_y_input,
                                    };
                                    config_changed = true;
                                }
                            });

                            if !is_current_pos {
                                ui.add_space(2.0);
                                ui.horizontal_wrapped(|ui| {
                                    ui.label("X:");
                                    if ui
                                        .add(egui::DragValue::new(&mut self.fixed_x_input).speed(1))
                                        .changed()
                                    {
                                        self.config.cursor_mode = CursorModeChoice::Fixed {
                                            x: self.fixed_x_input,
                                            y: self.fixed_y_input,
                                        };
                                        config_changed = true;
                                    }

                                    ui.label("Y:");
                                    if ui
                                        .add(egui::DragValue::new(&mut self.fixed_y_input).speed(1))
                                        .changed()
                                    {
                                        self.config.cursor_mode = CursorModeChoice::Fixed {
                                            x: self.fixed_x_input,
                                            y: self.fixed_y_input,
                                        };
                                        config_changed = true;
                                    }

                                    let pick_btn_label = if let Some(target_time) = self.picking_location_countdown {
                                        let remaining = target_time.saturating_duration_since(Instant::now()).as_secs_f32();
                                        format!("⏳ Move mouse ({:.1}s)...", remaining.max(0.1))
                                    } else {
                                        "📍 Pick in 3s".to_string()
                                    };

                                    if ui.button(pick_btn_label).clicked() && self.picking_location_countdown.is_none() {
                                        self.picking_location_countdown = Some(Instant::now() + Duration::from_secs(3));
                                    }
                                });
                            }
                        });
                    });

                    // 5. Hotkey & Controls
                    egui::Frame::group(ui.style()).show(ui, |ui| {
                        ui.set_width(ui.available_width());
                        ui.vertical_centered(|ui| {
                            ui.horizontal_wrapped(|ui| {
                                ui.label(RichText::new("Global Hotkey:").strong());
                                let hotkey_text = if self.capturing_hotkey {
                                    "Press key (Esc to cancel)...".to_string()
                                } else {
                                    let mut display = self.config.hotkey.modifiers.join("+");
                                    if !display.is_empty() {
                                        display.push('+');
                                    }
                                    display.push_str(&self.config.hotkey.key);
                                    display
                                };

                                if ui.button(hotkey_text).clicked() {
                                    self.capturing_hotkey = true;
                                }
                            });

                            if let Some(msg) = &self.hotkey_status_message {
                                ui.add_space(2.0);
                                ui.label(RichText::new(msg).color(Color32::from_rgb(255, 100, 100)).size(11.0));
                            }
                        });
                    });

                    ui.add_space(4.0);

                    // 6. Action Button & Live Status
                    let is_running = self.clicker.is_running();
                    let button_text = if is_running {
                        RichText::new("⏹ STOP (or press hotkey)").size(15.0).strong().color(Color32::WHITE)
                    } else {
                        RichText::new("▶ START (or press hotkey)").size(15.0).strong().color(Color32::WHITE)
                    };

                    let button_color = if is_running {
                        Color32::from_rgb(220, 50, 50)
                    } else {
                        Color32::from_rgb(40, 160, 60)
                    };

                    ui.vertical_centered(|ui| {
                        let btn_width = ui.available_width().clamp(180.0, 320.0);
                        let start_stop_btn = egui::Button::new(button_text)
                            .fill(button_color)
                            .min_size(Vec2::new(btn_width, 38.0));

                        if ui.add(start_stop_btn).clicked() {
                            self.toggle_clicking(true);
                        }

                        ui.add_space(4.0);

                        let status_indicator = if is_running {
                            RichText::new(format!("● RUNNING — {} clicks", self.clicker.get_click_count()))
                                .color(Color32::from_rgb(50, 220, 100))
                                .strong()
                        } else {
                            RichText::new("○ STOPPED")
                                .color(Color32::from_rgb(160, 160, 160))
                                .strong()
                        };

                        ui.label(status_indicator);
                    });

                    if config_changed {
                        self.config.save();
                    }
                });
        });
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        self.clicker.stop();
        self.config.save();
    }
}
