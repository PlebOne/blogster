use egui::Ui;

pub struct MarkdownViewer;

impl MarkdownViewer {
    pub fn show(ui: &mut Ui, content: &str) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            let mut commonmark_cache = egui_commonmark::CommonMarkCache::default();
            egui_commonmark::CommonMarkViewer::new("markdown_viewer")
                .show(ui, &mut commonmark_cache, content);
        });
    }
}
