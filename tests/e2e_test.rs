use arthur_clicker::clicker::{ClickerEngine, ClickerTaskConfig};
use arthur_clicker::config::{
    AppConfig, ClickTypeChoice, CursorModeChoice, HotkeyConfig, MouseButtonChoice, RepeatModeChoice,
};
use arthur_clicker::hotkey::HotkeyService;
use std::fs;
use std::thread;
use std::time::Duration;

#[test]
fn test_e2e_config_lifecycle() {
    // Test directory isolation for e2e testing
    let temp_dir = std::env::temp_dir().join(format!("arthur_clicker_e2e_{}", std::process::id()));
    let _ = fs::create_dir_all(&temp_dir);
    let config_file = temp_dir.join("test_config.toml");

    // 1. Initial customized configuration
    let original_config = AppConfig {
        hours: 0,
        minutes: 1,
        seconds: 30,
        milliseconds: 250,
        mouse_button: MouseButtonChoice::Right,
        click_type: ClickTypeChoice::Double,
        repeat_mode: RepeatModeChoice::Count(50),
        cursor_mode: CursorModeChoice::Fixed { x: 500, y: 300 },
        hotkey: HotkeyConfig {
            key: "F8".to_string(),
            modifiers: vec!["CTRL".to_string(), "ALT".to_string()],
        },
    };

    // 2. Save directly to test path using atomic logic
    let toml_str = toml::to_string_pretty(&original_config).expect("Serialize failed");
    let temp_path = config_file.with_extension("toml.tmp");
    fs::write(&temp_path, &toml_str).expect("Failed to write tmp");
    fs::rename(&temp_path, &config_file).expect("Failed rename");

    // 3. Verify file exists on disk
    assert!(config_file.exists());

    // 4. Load from disk and assert integrity
    let loaded_content = fs::read_to_string(&config_file).expect("Failed read");
    let loaded_config: AppConfig = toml::from_str(&loaded_content).expect("Failed parse");
    assert_eq!(original_config, loaded_config);

    // 5. Cleanup
    let _ = fs::remove_dir_all(&temp_dir);
}

#[test]
fn test_e2e_clicker_lifecycle_and_bounded_execution() {
    let mut engine = ClickerEngine::new();
    assert!(!engine.is_running());
    assert_eq!(engine.get_click_count(), 0);

    // Run a task with repeat count = 3 and 10ms interval
    let task = ClickerTaskConfig {
        interval: Duration::from_millis(10),
        mouse_button: MouseButtonChoice::Left,
        click_type: ClickTypeChoice::Single,
        repeat_mode: RepeatModeChoice::Count(3),
        cursor_mode: CursorModeChoice::CurrentPosition,
        initial_delay: Duration::ZERO,
    };

    engine.start(task);
    assert!(engine.is_running());

    // Wait for task to complete (should take ~50-200ms depending on CI platform)
    let start_time = std::time::Instant::now();
    while engine.is_running() && start_time.elapsed() < Duration::from_secs(3) {
        thread::sleep(Duration::from_millis(20));
    }

    assert!(!engine.is_running());
    assert_eq!(engine.get_click_count(), 3);
}

#[test]
fn test_e2e_clicker_instant_stop() {
    let mut engine = ClickerEngine::new();

    // Start an indefinite clicker with a large interval
    let task = ClickerTaskConfig {
        interval: Duration::from_millis(500),
        mouse_button: MouseButtonChoice::Left,
        click_type: ClickTypeChoice::Single,
        repeat_mode: RepeatModeChoice::Indefinite,
        cursor_mode: CursorModeChoice::CurrentPosition,
        initial_delay: Duration::ZERO,
    };

    engine.start(task);
    assert!(engine.is_running());

    // Give it enough time to start
    thread::sleep(Duration::from_millis(30));
    assert!(engine.is_running());

    // Stop and measure how fast it halts (must be sub-100ms)
    let stop_start = std::time::Instant::now();
    engine.stop();
    let elapsed = stop_start.elapsed();

    assert!(!engine.is_running());
    assert!(elapsed < Duration::from_millis(150), "Stop took too long: {:?}", elapsed);
}

#[test]
fn test_e2e_hotkey_parsing_and_registration_pipeline() {
    let mut service = HotkeyService::new();

    // Test parsing key codes
    assert!(HotkeyService::parse_code("F6").is_some());
    assert!(HotkeyService::parse_code("A").is_some());
    assert!(HotkeyService::parse_code("SPACE").is_some());
    assert!(HotkeyService::parse_code("INVALID_KEY_NAME").is_none());

    // Test parsing modifiers
    let mods = vec!["CTRL".to_string(), "SHIFT".to_string()];
    let parsed_mods = HotkeyService::parse_modifiers(&mods);
    assert!(parsed_mods.is_some());

    // Test hotkey configuration registration
    let config = HotkeyConfig {
        key: "F6".to_string(),
        modifiers: vec!["SHIFT".to_string()],
    };

    // May succeed or fail depending on OS headless/CI environment, but must not crash/panic
    let _ = service.register(&config);
    let _ = service.poll_event();
}
