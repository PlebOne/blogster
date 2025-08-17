use crate::nostr_client::NostrClient;
use crate::post::NostrCredentials;
use crate::storage::Storage;
use crate::theme::CatppuccinMocha;
use egui::{RichText, Window};
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct CredentialsDialog {
    open: bool,
    private_key: String,
    display_name: String,
    about: String,
    picture: String,
    nip05: String,
    error_message: Option<String>,
    success_message: Option<String>,
}

impl Default for CredentialsDialog {
    fn default() -> Self {
        Self {
            open: false,
            private_key: String::new(),
            display_name: String::new(),
            about: String::new(),
            picture: String::new(),
            nip05: String::new(),
            error_message: None,
            success_message: None,
        }
    }
}

impl CredentialsDialog {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn open(&mut self) {
        self.open = true;
        self.error_message = None;
        self.success_message = None;
    }

    pub fn open_with_storage(&mut self, storage: &Storage) {
        self.open = true;
        self.error_message = None;
        self.success_message = None;
        
        // Automatically load existing credentials if available
        match storage.load_credentials() {
            Ok(Some(credentials)) => {
                self.private_key = credentials.private_key.clone();
                self.display_name = credentials.display_name.unwrap_or_default();
                self.about = credentials.about.unwrap_or_default();
                self.picture = credentials.picture.unwrap_or_default();
                self.nip05 = credentials.nip05.unwrap_or_default();
                self.success_message = Some("Existing credentials loaded".to_string());
            }
            Ok(None) => {
                // No existing credentials, start fresh
                self.clear_fields();
            }
            Err(e) => {
                self.error_message = Some(format!("Failed to load credentials: {}", e));
                self.clear_fields();
            }
        }
    }

    fn clear_fields(&mut self) {
        self.private_key.clear();
        self.display_name.clear();
        self.about.clear();
        self.picture.clear();
        self.nip05.clear();
    }

    pub fn show(
        &mut self,
        ctx: &egui::Context,
        storage: &mut Storage,
        nostr_client: &Arc<Mutex<NostrClient>>,
        runtime: &tokio::runtime::Runtime,
    ) {
        if !self.open {
            return;
        }

        let mut close_dialog = false;

        Window::new("🔑 Nostr Credentials")
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.spacing_mut().item_spacing.y = 10.0;

                    // Load existing credentials button
                    if ui.button("📂 Load Existing Credentials").clicked() {
                        match storage.load_credentials() {
                            Ok(Some(credentials)) => {
                                self.private_key = credentials.private_key.clone();
                                self.display_name = credentials.display_name.unwrap_or_default();
                                self.about = credentials.about.unwrap_or_default();
                                self.picture = credentials.picture.unwrap_or_default();
                                self.nip05 = credentials.nip05.unwrap_or_default();
                                self.success_message = Some("Credentials loaded".to_string());
                            }
                            Ok(None) => {
                                self.error_message = Some("No saved credentials found".to_string());
                            }
                            Err(e) => {
                                self.error_message = Some(format!("Failed to load credentials: {}", e));
                            }
                        }
                    }

                    ui.separator();

                    // Private key input
                    ui.label("Private Key:");
                    ui.small("Supports both nsec (bech32) and hex formats");
                    let _private_key_response = ui.add(
                        egui::TextEdit::singleline(&mut self.private_key)
                            .password(true)
                            .hint_text("Enter your nsec1... or hex private key...")
                    );

                    // Validate private key format
                    if !self.private_key.is_empty() && !NostrClient::validate_private_key(&self.private_key) {
                        if self.private_key.starts_with("nsec") {
                            ui.label(RichText::new("⚠️ Invalid nsec format").color(CatppuccinMocha::YELLOW));
                        } else {
                            ui.label(RichText::new("⚠️ Invalid private key format").color(CatppuccinMocha::YELLOW));
                        }
                    }

                    // Generate new key button
                    if ui.button("🎲 Generate New Keys (nsec format)").clicked() {
                        let credentials = NostrClient::generate_credentials();
                        self.private_key = credentials.private_key;
                        self.success_message = Some("New nsec key generated".to_string());
                    }

                    ui.separator();

                    // Profile information
                    ui.label("Profile Information (Optional):");

                    ui.horizontal(|ui| {
                        ui.label("Display Name:");
                        ui.text_edit_singleline(&mut self.display_name);
                    });

                    ui.horizontal(|ui| {
                        ui.label("About:");
                        ui.text_edit_multiline(&mut self.about);
                    });

                    ui.horizontal(|ui| {
                        ui.label("Picture URL:");
                        ui.text_edit_singleline(&mut self.picture);
                    });

                    ui.horizontal(|ui| {
                        ui.label("NIP-05:");
                        ui.text_edit_singleline(&mut self.nip05);
                    });

                    ui.separator();

                    // Show messages
                    if let Some(error) = &self.error_message {
                        ui.label(RichText::new(format!("❌ {}", error)).color(CatppuccinMocha::RED));
                    }

                    if let Some(success) = &self.success_message {
                        ui.label(RichText::new(format!("✅ {}", success)).color(CatppuccinMocha::GREEN));
                    }

                    ui.separator();

                    // Buttons
                    ui.horizontal(|ui| {
                        let save_clicked = ui.button("💾 Save").clicked();
                        eprintln!("DEBUG: Button check - clicked: {}, private_key empty: {}", 
                                save_clicked, self.private_key.is_empty());
                        if save_clicked && !self.private_key.is_empty() {
                            eprintln!("DEBUG: Save button clicked with private key: {}", if self.private_key.is_empty() { "EMPTY" } else { "NOT_EMPTY" });
                            if NostrClient::validate_private_key(&self.private_key) {
                                eprintln!("DEBUG: Private key validation passed");
                                match NostrClient::get_public_key_from_private(&self.private_key) {
                                    Ok(public_key) => {
                                        eprintln!("DEBUG: Public key generated successfully");
                                        let mut credentials = NostrCredentials::new(
                                            self.private_key.clone(),
                                            public_key
                                        );
                                        
                                        if !self.display_name.is_empty() {
                                            credentials.display_name = Some(self.display_name.clone());
                                        }
                                        if !self.about.is_empty() {
                                            credentials.about = Some(self.about.clone());
                                        }
                                        if !self.picture.is_empty() {
                                            credentials.picture = Some(self.picture.clone());
                                        }
                                        if !self.nip05.is_empty() {
                                            credentials.nip05 = Some(self.nip05.clone());
                                        }

                                        eprintln!("DEBUG: About to save credentials...");

                                        // Save credentials
                                        match storage.save_credentials(&credentials) {
                                            Ok(()) => {
                                                // Set credentials in client
                                                let client = nostr_client.clone();
                                                let creds = credentials.clone();
                                                runtime.spawn(async move {
                                                    if let Ok(mut client) = client.try_lock() {
                                                        let _ = client.set_credentials(creds);
                                                    }
                                                });
                                                
                                                self.success_message = Some("Credentials saved successfully".to_string());
                                                close_dialog = true;
                                            }
                                            Err(e) => {
                                                self.error_message = Some(format!("Failed to save: {}", e));
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        self.error_message = Some(format!("Invalid private key: {}", e));
                                    }
                                }
                            } else {
                                eprintln!("DEBUG: Private key validation failed");
                                self.error_message = Some("Invalid private key format".to_string());
                            }
                        }

                        if ui.button("🗑️ Delete").clicked() {
                            match storage.delete_credentials() {
                                Ok(()) => {
                                    self.private_key.clear();
                                    self.display_name.clear();
                                    self.about.clear();
                                    self.picture.clear();
                                    self.nip05.clear();
                                    self.success_message = Some("Credentials deleted".to_string());
                                }
                                Err(e) => {
                                    self.error_message = Some(format!("Failed to delete: {}", e));
                                }
                            }
                        }

                        if ui.button("❌ Cancel").clicked() {
                            close_dialog = true;
                        }
                    });
                });
            });

        if close_dialog {
            self.open = false;
            self.error_message = None;
            self.success_message = None;
        }
    }
}
