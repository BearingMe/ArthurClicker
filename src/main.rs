use arthur_clicker::app::ArthurClickerApp;
use eframe::egui::{self, IconData, Vec2};
use std::sync::Arc;

fn load_app_icon() -> Option<IconData> {
    let icon_bytes = include_bytes!("../assets/icon.png");
    if let Ok(image) = image::load_from_memory(icon_bytes) {
        let rgba = image.to_rgba8();
        let (width, height) = rgba.dimensions();
        Some(IconData {
            rgba: rgba.into_raw(),
            width,
            height,
        })
    } else {
        None
    }
}

fn main() -> eframe::Result<()> {
    let mut viewport = egui::ViewportBuilder::default()
        .with_inner_size(Vec2::new(380.0, 520.0))
        .with_min_inner_size(Vec2::new(280.0, 350.0))
        .with_title("Arthur Auto Clicker");

    if let Some(icon) = load_app_icon() {
        viewport = viewport.with_icon(Arc::new(icon));
    }

    let native_options = eframe::NativeOptions {
        viewport,
        ..Default::default()
    };

    eframe::run_native(
        "Arthur Auto Clicker",
        native_options,
        Box::new(|cc| {
            cc.egui_ctx.set_visuals(egui::Visuals::dark());
            Ok(Box::new(ArthurClickerApp::new(cc)))
        }),
    )
}
