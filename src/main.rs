use arthur_clicker::app::ArthurClickerApp;
use eframe::egui::{self, Vec2};

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size(Vec2::new(380.0, 520.0))
            .with_min_inner_size(Vec2::new(280.0, 350.0))
            .with_title("Arthur Auto Clicker"),
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
