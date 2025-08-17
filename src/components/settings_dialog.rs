use crate::theme::{Theme, ThemeColors, CustomThemeColors};
use crate::storage::Storage;
use egui::{Context, RichText, Window, Color32};

pub struct SettingsDialog {
    open: bool,
    current_theme: Theme,
    selected_theme: Theme,
    theme_changed: bool,
    custom_colors: CustomThemeColors,
    custom_colors_changed: bool,
    show_custom_colors: bool,
}

impl Default for SettingsDialog {
    fn default() -> Self {
        Self {
            open: false,
            current_theme: Theme::default(),
            selected_theme: Theme::default(),
            theme_changed: false,
            custom_colors: CustomThemeColors::default(),
            custom_colors_changed: false,
            show_custom_colors: false,
        }
    }
}

impl SettingsDialog {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn open(&mut self, current_theme: Theme, custom_colors: &CustomThemeColors) {
        self.open = true;
        self.current_theme = current_theme;
        self.selected_theme = current_theme;
        self.theme_changed = false;
        self.custom_colors = custom_colors.clone();
        self.custom_colors_changed = false;
        self.show_custom_colors = current_theme == Theme::Custom;
    }

    pub fn is_open(&self) -> bool {
        self.open
    }

    /// Returns the new theme and custom colors if they were changed
    pub fn show(&mut self, ctx: &Context, storage: &Storage, theme_colors: &ThemeColors, current_theme: &Theme, current_custom_colors: &CustomThemeColors) -> Option<(Theme, Option<CustomThemeColors>)> {
        let mut result = None;
        let mut should_close = false;
        
        if !self.open {
            return result;
        }

        Window::new("⚙️ Settings")
            .open(&mut self.open)
            .resizable(true)
            .default_width(500.0)
            .default_height(600.0)
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
                                    self.show_custom_colors = *theme == Theme::Custom;
                                }
                                
                                // Two columns
                                if (i + 1) % 2 == 0 {
                                    ui.end_row();
                                }
                            }
                        });
                    
                    // Custom colors section
                    if self.show_custom_colors {
                        ui.add_space(16.0);
                        ui.separator();
                        ui.add_space(8.0);
                        
                        ui.label(RichText::new("Custom Colors:").strong().color(theme_colors.text));
                        ui.add_space(8.0);
                        
                        egui::ScrollArea::vertical()
                            .max_height(200.0)
                            .show(ui, |ui| {
                                egui::Grid::new("custom_colors_grid")
                                    .num_columns(2)
                                    .spacing([10.0, 8.0])
                                    .show(ui, |ui| {
                                        // Primary colors
                                        ui.label("Primary:");
                                        let mut primary_color = Color32::from_rgb(self.custom_colors.primary[0], self.custom_colors.primary[1], self.custom_colors.primary[2]);
                                        if ui.color_edit_button_srgba(&mut primary_color).changed() {
                                            self.custom_colors.primary = [primary_color.r(), primary_color.g(), primary_color.b()];
                                            self.custom_colors_changed = true;
                                        }
                                        ui.end_row();
                                        
                                        ui.label("Secondary:");
                                        let mut secondary_color = Color32::from_rgb(self.custom_colors.secondary[0], self.custom_colors.secondary[1], self.custom_colors.secondary[2]);
                                        if ui.color_edit_button_srgba(&mut secondary_color).changed() {
                                            self.custom_colors.secondary = [secondary_color.r(), secondary_color.g(), secondary_color.b()];
                                            self.custom_colors_changed = true;
                                        }
                                        ui.end_row();
                                        
                                        ui.label("Success:");
                                        let mut success_color = Color32::from_rgb(self.custom_colors.success[0], self.custom_colors.success[1], self.custom_colors.success[2]);
                                        if ui.color_edit_button_srgba(&mut success_color).changed() {
                                            self.custom_colors.success = [success_color.r(), success_color.g(), success_color.b()];
                                            self.custom_colors_changed = true;
                                        }
                                        ui.end_row();
                                        
                                        ui.label("Warning:");
                                        let mut warning_color = Color32::from_rgb(self.custom_colors.warning[0], self.custom_colors.warning[1], self.custom_colors.warning[2]);
                                        if ui.color_edit_button_srgba(&mut warning_color).changed() {
                                            self.custom_colors.warning = [warning_color.r(), warning_color.g(), warning_color.b()];
                                            self.custom_colors_changed = true;
                                        }
                                        ui.end_row();
                                        
                                        ui.label("Error:");
                                        let mut error_color = Color32::from_rgb(self.custom_colors.error[0], self.custom_colors.error[1], self.custom_colors.error[2]);
                                        if ui.color_edit_button_srgba(&mut error_color).changed() {
                                            self.custom_colors.error = [error_color.r(), error_color.g(), error_color.b()];
                                            self.custom_colors_changed = true;
                                        }
                                        ui.end_row();
                                        
                                        ui.label("Info:");
                                        let mut info_color = Color32::from_rgb(self.custom_colors.info[0], self.custom_colors.info[1], self.custom_colors.info[2]);
                                        if ui.color_edit_button_srgba(&mut info_color).changed() {
                                            self.custom_colors.info = [info_color.r(), info_color.g(), info_color.b()];
                                            self.custom_colors_changed = true;
                                        }
                                        ui.end_row();
                                        
                                        ui.label("Text:");
                                        let mut text_color = Color32::from_rgb(self.custom_colors.text[0], self.custom_colors.text[1], self.custom_colors.text[2]);
                                        if ui.color_edit_button_srgba(&mut text_color).changed() {
                                            self.custom_colors.text = [text_color.r(), text_color.g(), text_color.b()];
                                            self.custom_colors_changed = true;
                                        }
                                        ui.end_row();
                                        
                                        ui.label("Text Secondary:");
                                        let mut text_secondary_color = Color32::from_rgb(self.custom_colors.text_secondary[0], self.custom_colors.text_secondary[1], self.custom_colors.text_secondary[2]);
                                        if ui.color_edit_button_srgba(&mut text_secondary_color).changed() {
                                            self.custom_colors.text_secondary = [text_secondary_color.r(), text_secondary_color.g(), text_secondary_color.b()];
                                            self.custom_colors_changed = true;
                                        }
                                        ui.end_row();
                                        
                                        ui.label("Text Muted:");
                                        let mut text_muted_color = Color32::from_rgb(self.custom_colors.text_muted[0], self.custom_colors.text_muted[1], self.custom_colors.text_muted[2]);
                                        if ui.color_edit_button_srgba(&mut text_muted_color).changed() {
                                            self.custom_colors.text_muted = [text_muted_color.r(), text_muted_color.g(), text_muted_color.b()];
                                            self.custom_colors_changed = true;
                                        }
                                        ui.end_row();
                                        
                                        ui.label("Background:");
                                        let mut background_color = Color32::from_rgb(self.custom_colors.background[0], self.custom_colors.background[1], self.custom_colors.background[2]);
                                        if ui.color_edit_button_srgba(&mut background_color).changed() {
                                            self.custom_colors.background = [background_color.r(), background_color.g(), background_color.b()];
                                            self.custom_colors_changed = true;
                                        }
                                        ui.end_row();
                                        
                                        ui.label("Surface:");
                                        let mut surface_color = Color32::from_rgb(self.custom_colors.surface[0], self.custom_colors.surface[1], self.custom_colors.surface[2]);
                                        if ui.color_edit_button_srgba(&mut surface_color).changed() {
                                            self.custom_colors.surface = [surface_color.r(), surface_color.g(), surface_color.b()];
                                            self.custom_colors_changed = true;
                                        }
                                        ui.end_row();
                                        
                                        ui.label("Border:");
                                        let mut border_color = Color32::from_rgb(self.custom_colors.border[0], self.custom_colors.border[1], self.custom_colors.border[2]);
                                        if ui.color_edit_button_srgba(&mut border_color).changed() {
                                            self.custom_colors.border = [border_color.r(), border_color.g(), border_color.b()];
                                            self.custom_colors_changed = true;
                                        }
                                        ui.end_row();
                                    });
                            });
                            
                        ui.add_space(8.0);
                        ui.horizontal(|ui| {
                            if ui.button("🔄 Reset Colors").clicked() {
                                self.custom_colors = CustomThemeColors::default();
                                self.custom_colors_changed = true;
                            }
                            
                            if ui.button("📋 Load from Current Theme").clicked() {
                                if self.current_theme != Theme::Custom {
                                    let theme_colors = self.current_theme.colors(None);
                                    self.custom_colors = CustomThemeColors::from_theme_colors(&theme_colors);
                                    self.custom_colors_changed = true;
                                }
                            }
                        });
                    }
                    
                    ui.add_space(16.0);
                    ui.separator();
                    ui.add_space(8.0);
                    
                    // Action buttons
                    ui.horizontal(|ui| {
                        if ui.button(RichText::new("💾 Apply").color(theme_colors.success)).clicked() {
                            let mut changes_made = false;
                            let mut new_custom_colors = None;
                            
                            if self.theme_changed {
                                // Save the theme
                                if let Err(e) = storage.save_theme(self.selected_theme) {
                                    tracing::error!("Failed to save theme: {}", e);
                                } else {
                                    self.current_theme = self.selected_theme;
                                    self.theme_changed = false;
                                    changes_made = true;
                                }
                            }
                            
                            if self.custom_colors_changed {
                                new_custom_colors = Some(self.custom_colors.clone());
                                self.custom_colors_changed = false;
                                changes_made = true;
                            }
                            
                            if changes_made {
                                result = Some((self.selected_theme, new_custom_colors));
                            }
                        }
                        
                        if ui.button(RichText::new("🔄 Reset").color(theme_colors.warning)).clicked() {
                            self.selected_theme = self.current_theme;
                            self.theme_changed = false;
                            self.custom_colors = current_custom_colors.clone();
                            self.custom_colors_changed = false;
                        }
                        
                        if ui.button(RichText::new("❌ Cancel").color(theme_colors.error)).clicked() {
                            should_close = true;
                            self.selected_theme = self.current_theme;
                            self.theme_changed = false;
                            self.custom_colors = current_custom_colors.clone();
                            self.custom_colors_changed = false;
                        }
                    });
                });
            });

        if should_close {
            self.open = false;
        }

        result
    }
}
