use crate::post::BlogPost;
use crate::theme::CatppuccinMocha;
use egui::{RichText, Ui};

pub struct MarkdownEditor {
    current_post: Option<BlogPost>,
    preview_mode: bool,
    new_tag: String,
}

impl Default for MarkdownEditor {
    fn default() -> Self {
        Self {
            current_post: None,
            preview_mode: false,
            new_tag: String::new(),
        }
    }
}

impl MarkdownEditor {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_post(&mut self, post: BlogPost) {
        self.current_post = Some(post);
        self.preview_mode = false;
    }

    pub fn get_post(&self) -> Option<&BlogPost> {
        self.current_post.as_ref()
    }

    pub fn get_post_mut(&mut self) -> Option<&mut BlogPost> {
        self.current_post.as_mut()
    }

    pub fn take_post(&mut self) -> Option<BlogPost> {
        self.current_post.take()
    }

    pub fn show(&mut self, ui: &mut Ui) -> EditorAction {
        let mut action = EditorAction::None;

        if let Some(post) = &mut self.current_post {
            ui.vertical(|ui| {
                // Header with controls
                ui.horizontal(|ui| {
                    ui.heading("✏️ Editor");
                    
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        // Save button
                        if ui.button(RichText::new("💾 Save").color(CatppuccinMocha::GREEN)).clicked() {
                            action = EditorAction::Save;
                        }

                        // Publish button
                        if post.is_ready_to_publish() {
                            if ui.button(RichText::new("🚀 Publish").color(CatppuccinMocha::BLUE)).clicked() {
                                action = EditorAction::Publish;
                            }
                        }

                        // Image button
                        if ui.button("🖼️ Image").clicked() {
                            action = EditorAction::InsertImage;
                        }

                        // Preview toggle
                        let preview_text = if self.preview_mode { "📝 Edit" } else { "👁 Preview" };
                        if ui.button(preview_text).clicked() {
                            self.preview_mode = !self.preview_mode;
                        }
                    });
                });

                ui.separator();

                // Title input
                ui.horizontal(|ui| {
                    ui.label("Title:");
                    let title_response = ui.text_edit_singleline(&mut post.title);
                    if title_response.changed() {
                        post.updated_at = chrono::Utc::now();
                        action = EditorAction::Changed;
                    }
                });

                // Tags section
                ui.horizontal(|ui| {
                    ui.label("Tags:");
                    
                    // Display existing tags
                    let tags_to_remove: Vec<String> = post.tags.iter().cloned().collect();
                    for tag in &tags_to_remove {
                        ui.horizontal(|ui| {
                            ui.label(RichText::new(&format!("#{}", tag)).color(CatppuccinMocha::BLUE));
                            if ui.small_button("❌").clicked() {
                                post.remove_tag(tag);
                                action = EditorAction::Changed;
                            }
                        });
                    }

                    // Add new tag
                    if ui.text_edit_singleline(&mut self.new_tag).lost_focus() 
                        && ui.input(|i| i.key_pressed(egui::Key::Enter)) 
                        && !self.new_tag.trim().is_empty() {
                        post.add_tag(self.new_tag.trim().to_string());
                        self.new_tag.clear();
                        action = EditorAction::Changed;
                    }
                    
                    if ui.button("➕").clicked() && !self.new_tag.trim().is_empty() {
                        post.add_tag(self.new_tag.trim().to_string());
                        self.new_tag.clear();
                        action = EditorAction::Changed;
                    }
                });

                // Image URL input
                ui.horizontal(|ui| {
                    ui.label("Image:");
                    let mut image_url = post.image_url.clone().unwrap_or_default();
                    let image_response = ui.text_edit_singleline(&mut image_url);
                    if image_response.changed() {
                        post.image_url = if image_url.is_empty() { None } else { Some(image_url) };
                        post.updated_at = chrono::Utc::now();
                        action = EditorAction::Changed;
                    }
                    
                    // Upload featured image button
                    if ui.button("📤 Upload").clicked() {
                        action = EditorAction::UploadFeaturedImage;
                    }
                });

                ui.separator();

                // Content area
                if self.preview_mode {
                    // Preview mode
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("Preview").strong().color(CatppuccinMocha::GREEN));
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label(RichText::new(format!("{} words", post.word_count())).small().color(CatppuccinMocha::SUBTEXT1));
                        });
                    });

                    egui::ScrollArea::vertical().show(ui, |ui| {
                        // Custom markdown preview with colored headers
                        let lines: Vec<&str> = post.content.lines().collect();
                        for line in lines {
                            if line.starts_with("# ") {
                                ui.heading(RichText::new(&line[2..]).color(CatppuccinMocha::LAVENDER));
                            } else if line.starts_with("## ") {
                                ui.add(egui::Label::new(RichText::new(&line[3..]).heading().color(CatppuccinMocha::BLUE)));
                            } else if line.starts_with("### ") {
                                ui.add(egui::Label::new(RichText::new(&line[4..]).strong().color(CatppuccinMocha::SKY)));
                            } else if line.starts_with("#### ") {
                                ui.add(egui::Label::new(RichText::new(&line[5..]).strong().color(CatppuccinMocha::TEAL)));
                            } else if line.starts_with("##### ") {
                                ui.add(egui::Label::new(RichText::new(&line[6..]).strong().color(CatppuccinMocha::GREEN)));
                            } else if line.starts_with("###### ") {
                                ui.add(egui::Label::new(RichText::new(&line[7..]).strong().color(CatppuccinMocha::YELLOW)));
                            } else if line.starts_with("**") && line.ends_with("**") && line.len() > 4 {
                                ui.add(egui::Label::new(RichText::new(&line[2..line.len()-2]).strong().color(CatppuccinMocha::PEACH)));
                            } else if line.starts_with("*") && line.ends_with("*") && line.len() > 2 && !line.starts_with("**") {
                                ui.add(egui::Label::new(RichText::new(&line[1..line.len()-1]).italics().color(CatppuccinMocha::MAUVE)));
                            } else if line.starts_with("> ") {
                                ui.add(egui::Label::new(RichText::new(&line[2..]).color(CatppuccinMocha::OVERLAY2)));
                            } else if line.starts_with("```") {
                                ui.add(egui::Label::new(RichText::new(line).monospace().color(CatppuccinMocha::SUBTEXT0)));
                            } else if line.trim().is_empty() {
                                ui.add_space(5.0);
                            } else {
                                ui.add(egui::Label::new(RichText::new(line).color(CatppuccinMocha::TEXT)));
                            }
                        }
                    });
                } else {
                    // Edit mode
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("Content (Markdown)").strong().color(CatppuccinMocha::BLUE));
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label(RichText::new(format!("{} words", post.word_count())).small().color(CatppuccinMocha::SUBTEXT1));
                        });
                    });

                    egui::ScrollArea::vertical().show(ui, |ui| {
                        let content_response = ui.add_sized(
                            [ui.available_width(), ui.available_height() - 50.0],
                            egui::TextEdit::multiline(&mut post.content)
                                .font(egui::TextStyle::Monospace)
                                .hint_text("Write your blog post in Markdown...")
                        );
                        
                        if content_response.changed() {
                            post.updated_at = chrono::Utc::now();
                            action = EditorAction::Changed;
                        }
                    });
                }

                // Status bar
                ui.separator();
                ui.horizontal(|ui| {
                    let status_color = match post.status {
                        crate::post::PostStatus::Draft => CatppuccinMocha::YELLOW,
                        crate::post::PostStatus::Published => CatppuccinMocha::GREEN,
                        crate::post::PostStatus::Failed => CatppuccinMocha::RED,
                    };
                    
                    ui.label(RichText::new(format!("Status: {:?}", post.status)).color(status_color));
                    
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(RichText::new(format!(
                            "Updated: {}",
                            post.updated_at.format("%Y-%m-%d %H:%M")
                        )).small().color(CatppuccinMocha::SUBTEXT1));
                    });
                });
            });
        } else {
            // No post selected
            ui.vertical_centered(|ui| {
                ui.add_space(100.0);
                ui.label(RichText::new("No post selected").size(20.0).color(CatppuccinMocha::SUBTEXT1));
                ui.add_space(10.0);
                ui.label(RichText::new("Select a post from the sidebar or create a new one").color(CatppuccinMocha::SUBTEXT0));
            });
        }

        action
    }
}

#[derive(Debug, Clone)]
pub enum EditorAction {
    None,
    Changed,
    Save,
    Publish,
    InsertImage,
    UploadFeaturedImage,
}
