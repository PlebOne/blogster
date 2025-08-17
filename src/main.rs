mod app;
mod blossom_client;
mod components;
mod nostr_client;
mod post;
mod storage;
mod theme;

use app::BlogsterApp;
use tracing_subscriber;

fn main() -> eframe::Result {
    // Initialize logging
    tracing_subscriber::fmt::init();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_min_inner_size([800.0, 600.0])
            .with_title("Blogster - Nostr Blog Publisher"),
        ..Default::default()
    };

    eframe::run_native(
        "Blogster",
        options,
        Box::new(|cc| {
            // Setup custom fonts if needed
            setup_custom_fonts(&cc.egui_ctx);
            Ok(Box::new(BlogsterApp::new(cc)))
        }),
    )
}

fn setup_custom_fonts(ctx: &egui::Context) {
    let fonts = egui::FontDefinitions::default();
    
    // Add custom fonts here if needed
    // For now, we'll use the default fonts
    
    ctx.set_fonts(fonts);
}
