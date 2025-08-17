use crate::theme::{Theme, ThemeColors};
use crate::storage::Storage;
use egui::{Context, RichText, Window};

pub struct SettingsDialog {
    open: bool,
    current_theme: Theme,
    selected_theme: Theme,
    theme_changed: bool,
}

impl Default for SettingsDialog {
    fn default() -> Self {
        Self {
            open: false,
            current_theme: Theme::default(),
            selected_theme: Theme::default(),
            theme_changed: false,
        }
    }
}

impl SettingsDialog {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn open(&mut self, current_theme: Theme) {
        self.open = true;
        self.current_theme = current_theme;
        self.selected_theme = current_theme;
        self.theme_changed = false;
    }

    pub fn is_open(&self) -> bool {
        self.open
    }

    /// Returns the new theme if it was changed
    pub fn show(&mut self, ctx: &Context, storage: &Storage, theme_colors: &ThemeColors) -> Option<Theme> {
        let mut new_theme = None;
        let mut should_close = false;
        
        if !self.open {
            return new_theme;
        }

        Window::new("⚙️ Settings")
            .open(&mut self.open)
            .resizable(true)
            .default_width(400.0)
            .default_height(300.0)
            .show(ctx, |ui| {
                ui.heading(RichText::new("Appearance").strong().color(theme_colors.primary));
                ui.separator();
                
                ui.vertical_centered(|ui| {
                    ui.label(RichText::new("Theme:").strong().color(theme_colors.text));
                    ui.add_space(8.0);
                    
                    // Theme selection grid
                    egui::Grid::new("theme_grid")
                        .num_columns(2)
                        .spacing([10.0, 8.0])
                        .show(ui, |ui| {
                            for (i, theme) in Theme::all_themes().iter().enumerate() {
                                let is_selected = *theme == self.selected_theme;
                                let is_current = *theme == self.current_theme;
                                
                                // Create clear button text with visual indicators
                                let button_text = if is_current && !self.theme_changed {
                                    format!("✅ {} (Current)", theme.name())
                                } else if is_selected && self.theme_changed {
                                    format!("🔵 {} (Selected)", theme.name())
                                } else {
                                    theme.name().to_string()
                                };
                                
                                // Use selectable_label for better automatic contrast handling
                                let response = ui.selectable_label(is_selected, button_text);
                                if response.clicked() {
                                    self.selected_theme = *theme;
                                    self.theme_changed = self.selected_theme != self.current_theme;
                                }
                                
                                // Two columns
                                if (i + 1) % 2 == 0 {
                                    ui.end_row();
                                }
                            }
                        });
                    
                    ui.add_space(16.0);
                    ui.separator();
                    ui.add_space(8.0);
                    
                    // Action buttons
                    ui.horizontal(|ui| {
                        if ui.button(RichText::new("💾 Apply").color(theme_colors.success)).clicked() {
                            if self.theme_changed {
                                // Save the theme
                                if let Err(e) = storage.save_theme(self.selected_theme) {
                                    tracing::error!("Failed to save theme: {}", e);
                                } else {
                                    new_theme = Some(self.selected_theme);
                                    self.current_theme = self.selected_theme;
                                    self.theme_changed = false;
                                }
                            }
                        }
                        
                        if ui.button(RichText::new("🔄 Reset").color(theme_colors.warning)).clicked() {
                            self.selected_theme = self.current_theme;
                            self.theme_changed = false;
                        }
                        
                        if ui.button(RichText::new("❌ Cancel").color(theme_colors.error)).clicked() {
                            should_close = true;
                            self.selected_theme = self.current_theme;
                            self.theme_changed = false;
                        }
                    });
                });
            });

        if should_close {
            self.open = false;
        }

        new_theme
    }
}
