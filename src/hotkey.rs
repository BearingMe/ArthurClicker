use crate::config::HotkeyConfig;
use global_hotkey::hotkey::{Code, HotKey, Modifiers};
use global_hotkey::{GlobalHotKeyEvent, GlobalHotKeyManager, HotKeyState};

pub struct HotkeyService {
    manager: Option<GlobalHotKeyManager>,
    current_hotkey: Option<HotKey>,
}

impl Default for HotkeyService {
    fn default() -> Self {
        Self::new()
    }
}

impl HotkeyService {
    pub fn new() -> Self {
        let manager = GlobalHotKeyManager::new().ok();
        Self {
            manager,
            current_hotkey: None,
        }
    }

    pub fn parse_code(key_str: &str) -> Option<Code> {
        match key_str.to_uppercase().as_str() {
            "F1" => Some(Code::F1),
            "F2" => Some(Code::F2),
            "F3" => Some(Code::F3),
            "F4" => Some(Code::F4),
            "F5" => Some(Code::F5),
            "F6" => Some(Code::F6),
            "F7" => Some(Code::F7),
            "F8" => Some(Code::F8),
            "F9" => Some(Code::F9),
            "F10" => Some(Code::F10),
            "F11" => Some(Code::F11),
            "F12" => Some(Code::F12),
            "A" => Some(Code::KeyA),
            "B" => Some(Code::KeyB),
            "C" => Some(Code::KeyC),
            "D" => Some(Code::KeyD),
            "E" => Some(Code::KeyE),
            "F" => Some(Code::KeyF),
            "G" => Some(Code::KeyG),
            "H" => Some(Code::KeyH),
            "I" => Some(Code::KeyI),
            "J" => Some(Code::KeyJ),
            "K" => Some(Code::KeyK),
            "L" => Some(Code::KeyL),
            "M" => Some(Code::KeyM),
            "N" => Some(Code::KeyN),
            "O" => Some(Code::KeyO),
            "P" => Some(Code::KeyP),
            "Q" => Some(Code::KeyQ),
            "R" => Some(Code::KeyR),
            "S" => Some(Code::KeyS),
            "T" => Some(Code::KeyT),
            "U" => Some(Code::KeyU),
            "V" => Some(Code::KeyV),
            "W" => Some(Code::KeyW),
            "X" => Some(Code::KeyX),
            "Y" => Some(Code::KeyY),
            "Z" => Some(Code::KeyZ),
            "SPACE" => Some(Code::Space),
            "ESCAPE" => Some(Code::Escape),
            _ => None,
        }
    }

    pub fn parse_modifiers(mods: &[String]) -> Option<Modifiers> {
        let mut result = Modifiers::empty();
        for m in mods {
            match m.to_uppercase().as_str() {
                "CTRL" | "CONTROL" => result |= Modifiers::CONTROL,
                "ALT" | "OPTION" => result |= Modifiers::ALT,
                "SHIFT" => result |= Modifiers::SHIFT,
                "SUPER" | "CMD" | "COMMAND" | "WIN" => result |= Modifiers::SUPER,
                _ => {}
            }
        }
        if result.is_empty() {
            None
        } else {
            Some(result)
        }
    }

    pub fn register(&mut self, config: &HotkeyConfig) -> bool {
        let Some(manager) = &self.manager else {
            return false;
        };

        // Unregister previous hotkey if present
        if let Some(old_hotkey) = self.current_hotkey.take() {
            let _ = manager.unregister(old_hotkey);
        }

        let Some(code) = Self::parse_code(&config.key) else {
            return false;
        };

        let modifiers = Self::parse_modifiers(&config.modifiers);
        let hotkey = HotKey::new(modifiers, code);

        if manager.register(hotkey).is_ok() {
            self.current_hotkey = Some(hotkey);
            true
        } else {
            false
        }
    }

    pub fn poll_event(&self) -> bool {
        let mut pressed = false;
        while let Ok(event) = GlobalHotKeyEvent::receiver().try_recv() {
            if event.state == HotKeyState::Pressed {
                if let Some(current) = self.current_hotkey {
                    if event.id == current.id() {
                        pressed = true;
                    }
                }
            }
        }
        pressed
    }
}

impl Drop for HotkeyService {
    fn drop(&mut self) {
        if let (Some(manager), Some(hotkey)) = (&self.manager, self.current_hotkey) {
            let _ = manager.unregister(hotkey);
        }
    }
}
