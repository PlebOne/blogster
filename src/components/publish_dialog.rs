use crate::nostr_client::NostrClient;
use crate::post::BlogPost;
use crate::theme::CatppuccinMocha;
use egui::{RichText, Window};
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct PublishDialog {
    open: bool,
    post: Option<BlogPost>,
    is_publishing: bool,
    error_message: Option<String>,
    progress_message: Option<String>,
}

impl Default for PublishDialog {
    fn default() -> Self {
        Self {
            open: false,
            post: None,
            is_publishing: false,
            error_message: None,
            progress_message: None,
        }
    }
}

impl PublishDialog {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn open(&mut self, post: BlogPost) {
        self.open = true;
        self.post = Some(post);
        self.is_publishing = false;
        self.error_message = None;
        self.progress_message = None;
    }

    pub fn show(
        &mut self,
        ctx: &egui::Context,
        nostr_client: &Arc<Mutex<NostrClient>>,
        runtime: &tokio::runtime::Runtime,
    ) -> Option<BlogPost> {
        if !self.open {
            return None;
        }

        let mut close_dialog = false;
        let published_post = None;
        let mut should_start_publishing = false;

        Window::new("🚀 Publish to Nostr")
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
            .show(ctx, |ui| {
                if let Some(post) = &self.post {
                    ui.vertical(|ui| {
                        ui.spacing_mut().item_spacing.y = 10.0;

                        // Post preview
                        ui.group(|ui| {
                            ui.vertical(|ui| {
                                ui.label(RichText::new(&post.title).strong().size(16.0));
                                ui.label(format!("📝 {} words", post.word_count()));
                                ui.label(format!("🏷️ {} tags", post.tags.len()));
                                
                                if !post.tags.is_empty() {
                                    ui.horizontal(|ui| {
                                        for tag in &post.tags {
                                            ui.label(RichText::new(&format!("#{}", tag)).color(CatppuccinMocha::BLUE));
                                        }
                                    });
                                }
                            });
                        });

                        ui.separator();

                        // Relay information
                        ui.label(RichText::new("Publishing to relays:").strong());
                        for relay in NostrClient::get_long_form_relays() {
                            ui.label(format!("• {}", relay));
                        }

                        ui.separator();

                        // Show messages
                        if let Some(error) = &self.error_message {
                            ui.label(RichText::new(format!("❌ {}", error)).color(CatppuccinMocha::RED));
                        }

                        if let Some(progress) = &self.progress_message {
                            ui.label(RichText::new(format!("⏳ {}", progress)).color(CatppuccinMocha::YELLOW));
                        }

                        ui.separator();

                        // Buttons
                        ui.horizontal(|ui| {
                            if !self.is_publishing {
                                if ui.button(RichText::new("🚀 Publish").color(CatppuccinMocha::GREEN)).clicked() {
                                    // Check if credentials are available
                                    let has_credentials = {
                                        let client = nostr_client.clone();
                                        let result = if let Ok(client_guard) = client.try_lock() {
                                            client_guard.get_credentials().is_some()
                                        } else {
                                            false
                                        };
                                        result
                                    };
                                    
                                    if has_credentials {
                                        should_start_publishing = true;
                                    } else {
                                        self.error_message = Some("No Nostr credentials configured. Please set up your credentials first.".to_string());
                                    }
                                }

                                if ui.button("❌ Cancel").clicked() {
                                    close_dialog = true;
                                }
                            } else {
                                ui.spinner();
                                ui.label("Publishing...");
                            }
                        });
                    });
                }
            });

        // Handle publishing outside the UI closure
        if should_start_publishing {
            self.start_publishing(nostr_client, runtime);
        }

        if close_dialog {
            self.open = false;
            self.post = None;
            self.is_publishing = false;
            self.error_message = None;
            self.progress_message = None;
        }

        published_post
    }

    fn start_publishing(
        &mut self,
        nostr_client: &Arc<Mutex<NostrClient>>,
        runtime: &tokio::runtime::Runtime,
    ) {
        if let Some(post) = self.post.take() {
            self.is_publishing = true;
            self.progress_message = Some("Connecting to relays...".to_string());

            let client = nostr_client.clone();
            
            // Clone the post for the async operation
            let post_clone = post.clone();
            
            // Spawn the publishing task
            runtime.spawn(async move {
                let result = {
                    let client_guard = client.lock().await;
                    
                    // Connect to relays first
                    if let Err(e) = client_guard.connect_to_relays().await {
                        Err(format!("Failed to connect to relays: {}", e))
                    } else {
                        // Publish the post
                        client_guard.publish_long_form_post(&post_clone).await
                            .map_err(|e| e.to_string())
                    }
                };
                
                match result {
                    Ok((event_id, relays)) => {
                        // Update post status
                        let mut _updated_post = post_clone;
                        _updated_post.set_published(event_id.to_hex(), relays);
                        
                        // In a real implementation, you'd need to communicate this back to the UI
                        // This is a simplified version
                        tracing::info!("Successfully published post: {}", _updated_post.title);
                    }
                    Err(e) => {
                        tracing::error!("Failed to publish post: {}", e);
                    }
                }
            });
        }
    }
}
