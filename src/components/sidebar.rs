use crate::post::{BlogPost, PostStatus};
use crate::theme::ThemeColors;
use egui::{Color32, RichText, Ui, Vec2};

pub struct Sidebar {
    search_query: String,
    selected_post_id: Option<uuid::Uuid>,
}

impl Default for Sidebar {
    fn default() -> Self {
        Self {
            search_query: String::new(),
            selected_post_id: None,
        }
    }
}

impl Sidebar {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn selected_post_id(&self) -> Option<uuid::Uuid> {
        self.selected_post_id
    }

    pub fn set_selected_post_id(&mut self, id: Option<uuid::Uuid>) {
        self.selected_post_id = id;
    }

    pub fn show(&mut self, ui: &mut Ui, posts: &[BlogPost], theme_colors: &ThemeColors) -> SidebarAction {
        let mut action = SidebarAction::None;

        ui.vertical(|ui| {
            // Header
            ui.horizontal(|ui| {
                ui.heading(RichText::new("📝 Blogster").color(theme_colors.primary));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button(RichText::new("➕").size(16.0)).clicked() {
                        action = SidebarAction::NewPost;
                    }
                });
            });

            ui.separator();

            // Search bar
            ui.horizontal(|ui| {
                ui.label(RichText::new("🔍").color(theme_colors.text));
                ui.text_edit_singleline(&mut self.search_query);
            });

            ui.separator();

            // Filter and display posts
            let filtered_posts: Vec<&BlogPost> = posts
                .iter()
                .filter(|post| {
                    if self.search_query.is_empty() {
                        true
                    } else {
                        let query = self.search_query.to_lowercase();
                        post.title.to_lowercase().contains(&query)
                            || post.content.to_lowercase().contains(&query)
                            || post.tags.iter().any(|tag| tag.to_lowercase().contains(&query))
                    }
                })
                .collect();

            // Group posts by status
            let mut drafts = Vec::new();
            let mut published = Vec::new();
            let mut failed = Vec::new();

            for post in filtered_posts {
                match post.status {
                    PostStatus::Draft => drafts.push(post),
                    PostStatus::Published => published.push(post),
                    PostStatus::Failed => failed.push(post),
                }
            }

            // Display post groups
            egui::ScrollArea::vertical()
                .auto_shrink([false, true])
                .show(ui, |ui| {
                    if !drafts.is_empty() {
                        self.show_post_group(ui, "📄 Drafts", &drafts, theme_colors.warning, &mut action, theme_colors);
                        ui.separator();
                    }

                    if !published.is_empty() {
                        self.show_post_group(ui, "✅ Published", &published, theme_colors.success, &mut action, theme_colors);
                        ui.separator();
                    }

                    if !failed.is_empty() {
                        self.show_post_group(ui, "❌ Failed", &failed, theme_colors.error, &mut action, theme_colors);
                    }

                    if drafts.is_empty() && published.is_empty() && failed.is_empty() {
                        ui.vertical_centered(|ui| {
                            ui.add_space(50.0);
                            ui.label(RichText::new("No posts found").color(theme_colors.text_secondary));
                            ui.label(RichText::new("Click ➕ to create your first post").color(theme_colors.text_muted));
                        });
                    }
                });
        });

        action
    }

    fn show_post_group(
        &mut self,
        ui: &mut Ui,
        title: &str,
        posts: &[&BlogPost],
        color: Color32,
        action: &mut SidebarAction,
        theme_colors: &ThemeColors,
    ) {
        ui.label(RichText::new(title).color(color).strong());
        ui.add_space(5.0);

        for post in posts {
            let is_selected = self.selected_post_id == Some(post.id);
            
            let response = ui.allocate_response(
                Vec2::new(ui.available_width(), 60.0),
                egui::Sense::click(),
            );

            // Background color for selection
            if is_selected {
                ui.painter().rect_filled(
                    response.rect,
                    4.0,
                    theme_colors.surface,
                );
            } else if response.hovered() {
                ui.painter().rect_filled(
                    response.rect,
                    4.0,
                    theme_colors.background,
                );
            }

            // Content
            ui.allocate_ui_at_rect(response.rect.shrink(8.0), |ui| {
                ui.vertical(|ui| {
                    // Title
                    let title_text = if post.title.is_empty() {
                        "Untitled"
                    } else {
                        &post.title
                    };
                    
                    ui.label(
                        RichText::new(title_text)
                            .strong()
                            .color(if post.title.is_empty() {
                                theme_colors.text_secondary
                            } else {
                                theme_colors.text
                            })
                    );

                    // Metadata
                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new(format!("{} words", post.word_count()))
                                .small()
                                .color(theme_colors.text_muted)
                        );
                        
                        if !post.tags.is_empty() {
                            ui.separator();
                            ui.label(
                                RichText::new(format!("{} tags", post.tags.len()))
                                    .small()
                                    .color(theme_colors.text_muted)
                            );
                        }

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label(
                                RichText::new(post.updated_at.format("%m/%d").to_string())
                                    .small()
                                    .color(theme_colors.text_muted)
                            );
                        });
                    });
                });
            });

            // Handle click
            if response.clicked() {
                self.selected_post_id = Some(post.id);
                *action = SidebarAction::SelectPost(post.id);
            }

            // Context menu
            response.context_menu(|ui| {
                if ui.button("🗑️ Delete").clicked() {
                    *action = SidebarAction::DeletePost(post.id);
                    ui.close_menu();
                }
                if ui.button("📤 Export").clicked() {
                    *action = SidebarAction::ExportPost(post.id);
                    ui.close_menu();
                }
                if post.status == PostStatus::Draft {
                    if ui.button("🚀 Publish").clicked() {
                        *action = SidebarAction::PublishPost(post.id);
                        ui.close_menu();
                    }
                }
            });

            ui.add_space(5.0);
        }
    }
}

#[derive(Debug, Clone)]
pub enum SidebarAction {
    None,
    NewPost,
    SelectPost(uuid::Uuid),
    DeletePost(uuid::Uuid),
    ExportPost(uuid::Uuid),
    PublishPost(uuid::Uuid),
}
