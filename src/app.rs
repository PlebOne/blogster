use crate::blossom_client::{BlossomClient, BlossomSettings};
use crate::components::{CredentialsDialog, EditorAction, MarkdownEditor, PublishDialog, Sidebar, SidebarAction};
use crate::nostr_client::NostrClient;
use crate::post::BlogPost;
use crate::storage::Storage;
use crate::theme::{apply_catppuccin_theme, CatppuccinMocha};
use egui::{CentralPanel, RichText, SidePanel, TopBottomPanel};
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct BlogsterApp {
    // Core components
    storage: Storage,
    nostr_client: Arc<Mutex<NostrClient>>,
    blossom_client: BlossomClient,
    
    // UI components
    sidebar: Sidebar,
    editor: MarkdownEditor,
    credentials_dialog: CredentialsDialog,
    publish_dialog: PublishDialog,
    
    // State
    posts: Vec<BlogPost>,
    error_message: Option<String>,
    success_message: Option<String>,
    is_loading: bool,
    show_settings: bool,
    blossom_settings: BlossomSettings,
    
    // Runtime
    runtime: tokio::runtime::Runtime,
}

impl BlogsterApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Apply theme
        apply_catppuccin_theme(&cc.egui_ctx);
        
        // Initialize storage
        let storage = Storage::new().expect("Failed to initialize storage");
        
        // Load posts
        let posts = storage.load_all_posts().unwrap_or_else(|e| {
            tracing::error!("Failed to load posts: {}", e);
            Vec::new()
        });

        // Load Blossom settings
        let blossom_settings = storage.load_blossom_settings().unwrap_or_else(|e| {
            tracing::error!("Failed to load Blossom settings: {}", e);
            BlossomSettings::default()
        });

        // Initialize Nostr client
        let nostr_client = Arc::new(Mutex::new(NostrClient::new()));

        // Initialize Blossom client and set Nostr client
        let mut blossom_client = BlossomClient::new(blossom_settings.clone());
        blossom_client.set_nostr_client(nostr_client.clone());
        
        // Create runtime for async operations
        let runtime = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");
        
        // Try to load credentials
        let mut app = Self {
            storage,
            nostr_client,
            blossom_client,
            sidebar: Sidebar::new(),
            editor: MarkdownEditor::new(),
            credentials_dialog: CredentialsDialog::new(),
            publish_dialog: PublishDialog::new(),
            posts,
            error_message: None,
            success_message: None,
            is_loading: false,
            show_settings: false,
            blossom_settings,
            runtime,
        };
        
        // Load credentials if available
        if let Ok(Some(credentials)) = app.storage.load_credentials() {
            app.runtime.block_on(async {
                if let Ok(mut client) = app.nostr_client.try_lock() {
                    if let Err(e) = client.set_credentials(credentials) {
                        app.error_message = Some(format!("Failed to load credentials: {}", e));
                    }
                }
            });
        }
        
        app
    }
    
    fn show_top_panel(&mut self, ctx: &egui::Context) {
        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(RichText::new("Blogster").size(18.0).strong().color(CatppuccinMocha::BLUE));
                
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    // Settings menu
                    ui.menu_button("⚙️ Settings", |ui| {
                        if ui.button("🔑 Nostr Credentials").clicked() {
                            self.credentials_dialog.open_with_storage(&self.storage);
                            ui.close_menu();
                        }

                        if ui.button("🌸 Blossom Settings").clicked() {
                            self.show_settings = true;
                            ui.close_menu();
                        }
                        
                        if ui.button("📁 Open Posts Folder").clicked() {
                            if let Err(e) = opener::open(self.storage.posts_dir()) {
                                self.error_message = Some(format!("Failed to open folder: {}", e));
                            }
                            ui.close_menu();
                        }
                        
                        if ui.button("📥 Import Post").clicked() {
                            self.import_post();
                            ui.close_menu();
                        }
                    });
                    
                    // Status indicators
                    ui.horizontal(|ui| {
                        // Credentials status
                        let has_credentials = self.runtime.block_on(async {
                            if let Ok(client) = self.nostr_client.try_lock() {
                                client.has_credentials()
                            } else {
                                false
                            }
                        });
                        
                        let creds_text = if has_credentials {
                            "🔑 Signed In"
                        } else {
                            "🔓 No Credentials"
                        };
                        
                        ui.label(RichText::new(creds_text).color(
                            if has_credentials {
                                CatppuccinMocha::GREEN
                            } else {
                                CatppuccinMocha::YELLOW
                            }
                        ));
                        
                        ui.separator();
                        
                        // Loading status
                        let status_text = if self.is_loading {
                            "⏳ Loading..."
                        } else {
                            "✅ Ready"
                        };
                        
                        ui.label(RichText::new(status_text).color(
                            if self.is_loading {
                                CatppuccinMocha::YELLOW
                            } else {
                                CatppuccinMocha::GREEN
                            }
                        ));
                    });
                });
            });
        });
    }
    
    fn show_bottom_panel(&mut self, ctx: &egui::Context) {
        TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                // Show messages
                if let Some(error) = &self.error_message {
                    ui.label(RichText::new(format!("❌ {}", error)).color(CatppuccinMocha::RED));
                    if ui.button("✖").clicked() {
                        self.error_message = None;
                    }
                } else if let Some(success) = &self.success_message {
                    ui.label(RichText::new(format!("✅ {}", success)).color(CatppuccinMocha::GREEN));
                    if ui.button("✖").clicked() {
                        self.success_message = None;
                    }
                } else {
                    ui.label(RichText::new(format!("{} posts", self.posts.len())).color(CatppuccinMocha::SUBTEXT1));
                }
            });
        });
    }
    
    fn handle_sidebar_action(&mut self, action: SidebarAction) {
        match action {
            SidebarAction::NewPost => {
                let mut new_post = BlogPost::new();
                new_post.title = "New Post".to_string();
                self.sidebar.set_selected_post_id(Some(new_post.id));
                self.editor.set_post(new_post);
            }
            SidebarAction::SelectPost(id) => {
                if let Some(post) = self.posts.iter().find(|p| p.id == id).cloned() {
                    self.editor.set_post(post);
                }
            }
            SidebarAction::DeletePost(id) => {
                if let Some(index) = self.posts.iter().position(|p| p.id == id) {
                    let post = &self.posts[index];
                    if let Err(e) = self.storage.delete_post(post) {
                        self.error_message = Some(format!("Failed to delete post: {}", e));
                    } else {
                        self.posts.remove(index);
                        self.success_message = Some("Post deleted successfully".to_string());
                        
                        // Clear editor if this post was selected
                        if self.sidebar.selected_post_id() == Some(id) {
                            self.sidebar.set_selected_post_id(None);
                            self.editor.take_post();
                        }
                    }
                }
            }
            SidebarAction::ExportPost(id) => {
                if let Some(post) = self.posts.iter().find(|p| p.id == id).cloned() {
                    self.export_post(&post);
                }
            }
            SidebarAction::PublishPost(id) => {
                if let Some(post) = self.posts.iter().find(|p| p.id == id).cloned() {
                    self.publish_dialog.open(post);
                }
            }
            SidebarAction::None => {}
        }
    }
    
    fn handle_editor_action(&mut self, action: EditorAction) {
        match action {
            EditorAction::Save => {
                if let Some(post) = self.editor.get_post() {
                    self.save_post(post.clone());
                }
            }
            EditorAction::Publish => {
                if let Some(post) = self.editor.get_post().cloned() {
                    self.publish_dialog.open(post);
                }
            }
            EditorAction::InsertImage => {
                self.insert_image();
            }
            EditorAction::UploadFeaturedImage => {
                self.upload_featured_image();
            }
            EditorAction::Changed => {
                // Auto-save on changes (optional)
                // self.save_current_post();
            }
            EditorAction::None => {}
        }
    }
    
    fn save_post(&mut self, mut post: BlogPost) {
        match self.storage.save_post(&post) {
            Ok(file_path) => {
                post.file_path = Some(file_path);
                
                // Update or add to posts list
                if let Some(existing_post) = self.posts.iter_mut().find(|p| p.id == post.id) {
                    *existing_post = post.clone();
                } else {
                    self.posts.push(post.clone());
                }
                
                // Update editor
                self.editor.set_post(post);
                
                self.success_message = Some("Post saved successfully".to_string());
            }
            Err(e) => {
                self.error_message = Some(format!("Failed to save post: {}", e));
            }
        }
    }
    
    fn export_post(&mut self, post: &BlogPost) {
        if let Some(path) = rfd::FileDialog::new()
            .set_file_name(&post.generate_filename())
            .add_filter("Markdown", &["md"])
            .save_file()
        {
            if let Err(e) = self.storage.export_post(post, &path) {
                self.error_message = Some(format!("Failed to export post: {}", e));
            } else {
                self.success_message = Some("Post exported successfully".to_string());
            }
        }
    }
    
    fn import_post(&mut self) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("Markdown", &["md"])
            .pick_file()
        {
            match self.storage.import_post(&path) {
                Ok(post) => {
                    self.posts.push(post);
                    self.success_message = Some("Post imported successfully".to_string());
                }
                Err(e) => {
                    self.error_message = Some(format!("Failed to import post: {}", e));
                }
            }
        }
    }
    
    fn insert_image(&mut self) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("Images", &["png", "jpg", "jpeg", "gif", "webp", "svg"])
            .pick_file()
        {
            self.is_loading = true;
            let blossom_client = self.blossom_client.clone();
            let path_clone = path.clone();
            
            // Upload image to Blossom server
            let upload_future = async move {
                blossom_client.upload_file(&path_clone).await
            };
            
            match self.runtime.block_on(upload_future) {
                Ok(blob_url) => {
                    // Insert the uploaded image URL into the post
                    if let Some(post) = self.editor.get_post_mut() {
                        let image_markdown = format!("![Image]({})", blob_url);
                        post.content.push_str(&format!("\n\n{}\n\n", image_markdown));
                        post.updated_at = chrono::Utc::now();
                    }
                    self.success_message = Some(format!("Image uploaded to Blossom server: {}", blob_url));
                }
                Err(e) => {
                    // Fallback to local file path if upload fails
                    tracing::warn!("Failed to upload to Blossom server: {}, using local path", e);
                    if let Some(post) = self.editor.get_post_mut() {
                        let image_markdown = format!("![Image]({})", path.display());
                        post.content.push_str(&format!("\n\n{}\n\n", image_markdown));
                        post.updated_at = chrono::Utc::now();
                    }
                    self.error_message = Some(format!("Failed to upload image to Blossom server: {}. Using local path instead.", e));
                }
            }
            
            self.is_loading = false;
        }
    }

    fn upload_featured_image(&mut self) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("Images", &["png", "jpg", "jpeg", "gif", "webp", "svg"])
            .pick_file()
        {
            self.is_loading = true;
            let blossom_client = self.blossom_client.clone();
            let path_clone = path.clone();
            
            // Upload image to Blossom server
            let upload_future = async move {
                blossom_client.upload_file(&path_clone).await
            };
            
            match self.runtime.block_on(upload_future) {
                Ok(blob_url) => {
                    // Set the uploaded image URL as the featured image
                    if let Some(post) = self.editor.get_post_mut() {
                        post.image_url = Some(blob_url.clone());
                        post.updated_at = chrono::Utc::now();
                    }
                    self.success_message = Some(format!("Featured image uploaded to Blossom server: {}", blob_url));
                }
                Err(e) => {
                    // Fallback to local file path if upload fails
                    tracing::warn!("Failed to upload featured image to Blossom server: {}, using local path", e);
                    if let Some(post) = self.editor.get_post_mut() {
                        post.image_url = Some(format!("file://{}", path.display()));
                        post.updated_at = chrono::Utc::now();
                    }
                    self.error_message = Some(format!("Failed to upload featured image to Blossom server: {}. Using local path instead.", e));
                }
            }
            
            self.is_loading = false;
        }
    }
}

impl eframe::App for BlogsterApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Handle dialogs
        self.credentials_dialog.show(ctx, &mut self.storage, &self.nostr_client, &self.runtime);
        
        // Show Blossom settings dialog
        if self.show_settings {
            egui::Window::new("🌸 Blossom Settings")
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.vertical(|ui| {
                        ui.label("Configure your Blossom server for image uploads:");
                        ui.add_space(10.0);

                        ui.horizontal(|ui| {
                            ui.label("Server URL:");
                            ui.text_edit_singleline(&mut self.blossom_settings.server_url);
                        });

                        ui.add_space(10.0);
                        ui.horizontal(|ui| {
                            if ui.button("Save").clicked() {
                                if let Err(e) = self.storage.save_blossom_settings(&self.blossom_settings) {
                                    self.error_message = Some(format!("Failed to save Blossom settings: {}", e));
                                } else {
                                    self.blossom_client.update_settings(self.blossom_settings.clone());
                                    self.success_message = Some("Blossom settings saved!".to_string());
                                }
                                self.show_settings = false;
                            }

                            if ui.button("Cancel").clicked() {
                                // Reload settings from storage to discard changes
                                if let Ok(settings) = self.storage.load_blossom_settings() {
                                    self.blossom_settings = settings;
                                }
                                self.show_settings = false;
                            }
                        });

                        ui.add_space(10.0);
                        ui.label("Default server: https://blossom.band");
                        ui.label("You can use any Blossom-compatible server for image hosting.");
                    });
                });
        }
        
        if let Some(published_post) = self.publish_dialog.show(ctx, &self.nostr_client, &self.runtime) {
            // Update the post in our list
            if let Some(existing_post) = self.posts.iter_mut().find(|p| p.id == published_post.id) {
                *existing_post = published_post.clone();
                
                // Save the updated post
                if let Err(e) = self.storage.save_post(&published_post) {
                    self.error_message = Some(format!("Failed to save published post: {}", e));
                } else {
                    self.success_message = Some("Post published successfully!".to_string());
                }
                
                // Update editor if this post is currently being edited
                if let Some(current_post) = self.editor.get_post() {
                    if current_post.id == published_post.id {
                        self.editor.set_post(published_post);
                    }
                }
            }
        }
        
        // Top panel
        self.show_top_panel(ctx);
        
        // Bottom panel
        self.show_bottom_panel(ctx);
        
        // Main content
        SidePanel::left("sidebar").resizable(true).show(ctx, |ui| {
            let action = self.sidebar.show(ui, &self.posts);
            self.handle_sidebar_action(action);
        });
        
        CentralPanel::default().show(ctx, |ui| {
            let action = self.editor.show(ui);
            self.handle_editor_action(action);
        });
    }
}
