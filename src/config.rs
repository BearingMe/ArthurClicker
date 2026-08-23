use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum MouseButtonChoice {
    #[default]
    Left,
    Middle,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum ClickTypeChoice {
    #[default]
    Single,
    Double,
    Triple,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum RepeatModeChoice {
    #[default]
    Indefinite,
    Count(u32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum CursorModeChoice {
    #[default]
    CurrentPosition,
    Fixed { x: i32, y: i32 },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct HotkeyConfig {
    pub key: String,
    pub modifiers: Vec<String>,
}

impl Default for HotkeyConfig {
    fn default() -> Self {
        Self {
            key: "F6".to_string(),
            modifiers: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AppConfig {
    pub hours: u32,
    pub minutes: u32,
    pub seconds: u32,
    pub milliseconds: u32,
    pub mouse_button: MouseButtonChoice,
    pub click_type: ClickTypeChoice,
    pub repeat_mode: RepeatModeChoice,
    pub cursor_mode: CursorModeChoice,
    pub hotkey: HotkeyConfig,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            hours: 0,
            minutes: 0,
            seconds: 0,
            milliseconds: 100,
            mouse_button: MouseButtonChoice::default(),
            click_type: ClickTypeChoice::default(),
            repeat_mode: RepeatModeChoice::default(),
            cursor_mode: CursorModeChoice::default(),
            hotkey: HotkeyConfig::default(),
        }
    }
}

impl AppConfig {
    pub fn get_interval(&self) -> Duration {
        let total_ms = (self.hours as u64 * 3600 * 1000)
            + (self.minutes as u64 * 60 * 1000)
            + (self.seconds as u64 * 1000)
            + (self.milliseconds as u64);
        Duration::from_millis(total_ms.max(1))
    }

    fn config_path() -> Option<PathBuf> {
        ProjectDirs::from("com", "ArthurClicker", "ArthurClicker")
            .map(|proj_dirs| proj_dirs.config_dir().join("config.toml"))
    }

    pub fn load() -> Self {
        if let Some(path) = Self::config_path() {
            if path.exists() {
                if let Ok(contents) = fs::read_to_string(&path) {
                    if let Ok(config) = toml::from_str::<Self>(&contents) {
                        return config;
                    }
                }
            }
        }
        Self::default()
    }

    /// Atomically save configuration via temporary file rename to prevent corrupted writes
    pub fn save(&self) {
        if let Some(path) = Self::config_path() {
            if let Some(parent) = path.parent() {
                let _ = fs::create_dir_all(parent);
            }
            if let Ok(toml_str) = toml::to_string_pretty(self) {
                let temp_path = path.with_extension("toml.tmp");
                if fs::write(&temp_path, toml_str).is_ok() {
                    let _ = fs::rename(&temp_path, &path);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config_interval() {
        let config = AppConfig::default();
        assert_eq!(config.get_interval(), Duration::from_millis(100));
    }

    #[test]
    fn test_custom_interval() {
        let config = AppConfig {
            hours: 1,
            minutes: 2,
            seconds: 3,
            milliseconds: 4,
            ..Default::default()
        };
        let expected_ms = (3600 + 120 + 3) * 1000 + 4;
        assert_eq!(config.get_interval(), Duration::from_millis(expected_ms));
    }

    #[test]
    fn test_serialization_roundtrip() {
        let config = AppConfig::default();
        let serialized = toml::to_string(&config).expect("Serialize failed");
        let deserialized: AppConfig = toml::from_str(&serialized).expect("Deserialize failed");
        assert_eq!(config, deserialized);
    }
}
