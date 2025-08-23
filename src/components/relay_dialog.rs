use crate::relay_settings::RelaySettings;
use crate::storage::Storage;
use crate::theme::ThemeColors;
use egui::{Context, RichText, Window, ScrollArea, TextEdit};

pub struct RelayDialog {
    open: bool,
    relay_settings: RelaySettings,
    new_relay_url: String,
    error_message: Option<String>,
    success_message: Option<String>,
    settings_changed: bool,
}

impl Default for RelayDialog {
    fn default() -> Self {
        Self {
            open: false,
            relay_settings: RelaySettings::default(),
            new_relay_url: String::new(),
            error_message: None,
            success_message: None,
            settings_changed: false,
        }
    }
}

impl RelayDialog {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn open(&mut self, current_settings: &RelaySettings) {
        self.open = true;
        self.relay_settings = current_settings.clone();
        self.new_relay_url.clear();
        self.error_message = None;
        self.success_message = None;
        self.settings_changed = false;
    }

    pub fn is_open(&self) -> bool {
        self.open
    }

    /// Returns the new relay settings if they were changed
    pub fn show(&mut self, ctx: &Context, storage: &Storage, theme_colors: &ThemeColors) -> Option<RelaySettings> {
        let mut result = None;
        let mut should_close = false;
        let mut add_relay_clicked = false;
        
        if !self.open {
            return result;
        }

        let mut window_open = self.open;
        
        Window::new("🌐 Relay Settings")
            .open(&mut window_open)
            .resizable(true)
            .default_width(600.0)
            .default_height(500.0)
            .show(ctx, |ui| {
                ui.heading(RichText::new("Nostr Relay Configuration").strong().color(theme_colors.primary));
                ui.separator();
                
                ui.vertical(|ui| {
                    // Relay source options
                    ui.label(RichText::new("Relay Sources:").strong().color(theme_colors.text));
                    ui.add_space(8.0);
                    
                    ui.horizontal(|ui| {
                        let mut use_default = self.relay_settings.use_default_relays;
                        if ui.checkbox(&mut use_default, "Use default relays").changed() {
                            self.relay_settings.use_default_relays = use_default;
                            self.settings_changed = true;
                        }
                        ui.label(RichText::new("(5 popular long-form content relays)").color(theme_colors.text_muted));
                    });
                    
                    ui.horizontal(|ui| {
                        let mut use_custom = self.relay_settings.use_custom_relays;
                        if ui.checkbox(&mut use_custom, "Use custom relays").changed() {
                            self.relay_settings.use_custom_relays = use_custom;
                            self.settings_changed = true;
                        }
                        ui.label(RichText::new("(your own relay list)").color(theme_colors.text_muted));
                    });
                    
                    ui.add_space(16.0);
                    ui.separator();
                    ui.add_space(8.0);
                    
                    // Default relays section
                    if self.relay_settings.use_default_relays {
                        ui.label(RichText::new("Default Relays:").strong().color(theme_colors.text));
                        ui.add_space(4.0);
                        
                        for relay in RelaySettings::get_default_relays() {
                            ui.horizontal(|ui| {
                                ui.label("🟢");
                                ui.label(RichText::new(&relay).color(theme_colors.text_secondary));
                            });
                        }
                        
                        ui.add_space(12.0);
                    }
                    
                    // Custom relays section
                    ui.label(RichText::new("Custom Relays:").strong().color(theme_colors.text));
                    ui.add_space(4.0);
                    
                    // Add new relay
                    ui.horizontal(|ui| {
                        ui.label("Add relay:");
                        let response = ui.add(
                            TextEdit::singleline(&mut self.new_relay_url)
                                .hint_text("wss://relay.example.com")
                                .desired_width(300.0)
                        );
                        
                        if ui.button(RichText::new("➕ Add").color(theme_colors.success)).clicked() || 
                           (response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter))) {
                            add_relay_clicked = true;
                        }
                    });
                    
                    ui.add_space(8.0);
                    
                    // List existing custom relays
                    if !self.relay_settings.custom_relays.is_empty() {
                        ScrollArea::vertical()
                            .max_height(200.0)
                            .show(ui, |ui| {
                                let mut to_remove = None;
                                
                                for (index, relay) in self.relay_settings.custom_relays.iter().enumerate() {
                                    ui.horizontal(|ui| {
                                        let status_color = if self.relay_settings.use_custom_relays {
                                            "🟢"
                                        } else {
                                            "🔴"
                                        };
                                        ui.label(status_color);
                                        ui.label(RichText::new(relay).color(theme_colors.text_secondary));
                                        
                                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                            if ui.button(RichText::new("🗑").color(theme_colors.error)).clicked() {
                                                to_remove = Some(index);
                                            }
                                        });
                                    });
                                }
                                
                                // Remove relay if requested
                                if let Some(index) = to_remove {
                                    self.relay_settings.custom_relays.remove(index);
                                    self.settings_changed = true;
                                    self.success_message = Some("Relay removed".to_string());
                                }
                            });
                    } else {
                        ui.label(RichText::new("No custom relays configured").color(theme_colors.text_muted).italics());
                    }
                    
                    ui.add_space(16.0);
                    
                    // Show error/success messages
                    if let Some(error) = &self.error_message {
                        ui.colored_label(theme_colors.error, format!("❌ {}", error));
                        ui.add_space(8.0);
                    }
                    
                    if let Some(success) = &self.success_message {
                        ui.colored_label(theme_colors.success, format!("✅ {}", success));
                        ui.add_space(8.0);
                    }
                    
                    // Active relays summary
                    ui.separator();
                    ui.add_space(8.0);
                    
                    let active_relays = self.relay_settings.get_active_relays();
                    ui.label(RichText::new(format!("Active relays: {}", active_relays.len())).strong().color(theme_colors.info));
                    
                    ui.add_space(16.0);
                    
                    // Action buttons
                    ui.horizontal(|ui| {
                        if ui.button(RichText::new("💾 Save").color(theme_colors.success)).clicked() {
                            if self.settings_changed {
                                if let Err(e) = storage.save_relay_settings(&self.relay_settings) {
                                    tracing::error!("Failed to save relay settings: {}", e);
                                    self.error_message = Some("Failed to save relay settings".to_string());
                                } else {
                                    result = Some(self.relay_settings.clone());
                                    self.settings_changed = false;
                                    self.success_message = Some("Relay settings saved!".to_string());
                                }
                            }
                        }
                        
                        if ui.button(RichText::new("🔄 Reset").color(theme_colors.warning)).clicked() {
                            self.relay_settings = RelaySettings::default();
                            self.settings_changed = true;
                            self.success_message = Some("Settings reset to defaults".to_string());
                        }
                        
                        if ui.button(RichText::new("❌ Cancel").color(theme_colors.error)).clicked() {
                            should_close = true;
                        }
                    });
                });
            });

        // Update the open state from the window
        self.open = window_open;

        if should_close {
            self.open = false;
        }
        
        // Handle add relay outside the UI closure
        if add_relay_clicked {
            self.add_relay();
        }

        // Clear messages after a delay
        if self.error_message.is_some() || self.success_message.is_some() {
            ctx.request_repaint_after(std::time::Duration::from_secs(3));
        }

        result
    }
    
    fn add_relay(&mut self) {
        let url = self.new_relay_url.trim().to_string();
        
        if url.is_empty() {
            self.error_message = Some("Please enter a relay URL".to_string());
            return;
        }
        
        match self.relay_settings.add_relay(url) {
            Ok(()) => {
                self.new_relay_url.clear();
                self.settings_changed = true;
                self.success_message = Some("Relay added successfully".to_string());
                self.error_message = None;
            }
            Err(e) => {
                self.error_message = Some(e);
                self.success_message = None;
            }
        }
    }
}
