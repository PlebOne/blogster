use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlogPost {
    pub id: Uuid,
    pub title: String,
    pub content: String, // Markdown content
    pub summary: Option<String>,
    pub tags: Vec<String>,
    pub image_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub status: PostStatus,
    pub nostr_event_id: Option<String>,
    pub published_relays: Vec<String>,
    pub file_path: Option<PathBuf>, // Path to the .md file
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PostStatus {
    Draft,
    Published,
    Failed,
}

impl Default for BlogPost {
    fn default() -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            title: String::new(),
            content: String::new(),
            summary: None,
            tags: Vec::new(),
            image_url: None,
            created_at: now,
            updated_at: now,
            status: PostStatus::Draft,
            nostr_event_id: None,
            published_relays: Vec::new(),
            file_path: None,
        }
    }
}

impl BlogPost {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_title(mut self, title: String) -> Self {
        self.title = title;
        self.updated_at = Utc::now();
        self
    }

    pub fn with_content(mut self, content: String) -> Self {
        self.content = content;
        self.updated_at = Utc::now();
        self
    }

    pub fn add_tag(&mut self, tag: String) {
        if !self.tags.contains(&tag) {
            self.tags.push(tag);
            self.updated_at = Utc::now();
        }
    }

    pub fn remove_tag(&mut self, tag: &str) {
        self.tags.retain(|t| t != tag);
        self.updated_at = Utc::now();
    }

    pub fn set_image(&mut self, image_url: String) {
        self.image_url = Some(image_url);
        self.updated_at = Utc::now();
    }

    pub fn set_published(&mut self, event_id: String, relays: Vec<String>) {
        self.status = PostStatus::Published;
        self.nostr_event_id = Some(event_id);
        self.published_relays = relays;
        self.updated_at = Utc::now();
    }

    pub fn set_failed(&mut self) {
        self.status = PostStatus::Failed;
        self.updated_at = Utc::now();
    }

    pub fn word_count(&self) -> usize {
        self.content.split_whitespace().count()
    }

    pub fn reading_time(&self) -> usize {
        // Assuming 200 words per minute reading speed
        let words = self.word_count();
        (words / 200).max(1)
    }

    pub fn is_ready_to_publish(&self) -> bool {
        !self.title.trim().is_empty() && !self.content.trim().is_empty()
    }

    /// Generate the markdown file content with frontmatter
    pub fn to_markdown_with_frontmatter(&self) -> String {
        let mut content = String::new();
        
        // Add YAML frontmatter
        content.push_str("---\n");
        content.push_str(&format!("title: \"{}\"\n", self.title.replace('"', "\\\"")));
        content.push_str(&format!("id: \"{}\"\n", self.id));
        content.push_str(&format!("created_at: \"{}\"\n", self.created_at.to_rfc3339()));
        content.push_str(&format!("updated_at: \"{}\"\n", self.updated_at.to_rfc3339()));
        content.push_str(&format!("status: \"{:?}\"\n", self.status));
        
        if let Some(summary) = &self.summary {
            content.push_str(&format!("summary: \"{}\"\n", summary.replace('"', "\\\"")));
        }
        
        if !self.tags.is_empty() {
            content.push_str("tags:\n");
            for tag in &self.tags {
                content.push_str(&format!("  - \"{}\"\n", tag.replace('"', "\\\"")));
            }
        }
        
        if let Some(image_url) = &self.image_url {
            content.push_str(&format!("image: \"{}\"\n", image_url));
        }
        
        if let Some(event_id) = &self.nostr_event_id {
            content.push_str(&format!("nostr_event_id: \"{}\"\n", event_id));
        }
        
        if !self.published_relays.is_empty() {
            content.push_str("published_relays:\n");
            for relay in &self.published_relays {
                content.push_str(&format!("  - \"{}\"\n", relay));
            }
        }
        
        content.push_str("---\n\n");
        
        // Add the markdown content
        content.push_str(&self.content);
        
        content
    }

    /// Parse a markdown file with frontmatter into a BlogPost
    pub fn from_markdown_with_frontmatter(content: &str, file_path: Option<PathBuf>) -> anyhow::Result<Self> {
        let mut post = BlogPost::default();
        post.file_path = file_path;
        
        if content.starts_with("---\n") {
            if let Some(end_pos) = content[4..].find("\n---\n") {
                let frontmatter = &content[4..end_pos + 4];
                let markdown_content = &content[end_pos + 8..];
                
                // Parse YAML frontmatter (simplified parsing)
                post.content = markdown_content.to_string();
                
                for line in frontmatter.lines() {
                    if let Some((key, value)) = line.split_once(':') {
                        let key = key.trim();
                        let value = value.trim().trim_matches('"');
                        
                        match key {
                            "title" => post.title = value.to_string(),
                            "id" => {
                                if let Ok(uuid) = Uuid::parse_str(value) {
                                    post.id = uuid;
                                }
                            }
                            "summary" => post.summary = Some(value.to_string()),
                            "image" => post.image_url = Some(value.to_string()),
                            "nostr_event_id" => post.nostr_event_id = Some(value.to_string()),
                            "status" => {
                                post.status = match value {
                                    "Published" => PostStatus::Published,
                                    "Failed" => PostStatus::Failed,
                                    _ => PostStatus::Draft,
                                };
                            }
                            _ => {}
                        }
                    }
                }
            }
        } else {
            // No frontmatter, treat entire content as markdown
            post.content = content.to_string();
            // Try to extract title from first heading
            if let Some(title_line) = content.lines().find(|line| line.starts_with("# ")) {
                post.title = title_line[2..].trim().to_string();
            }
        }
        
        Ok(post)
    }

    /// Generate a safe filename for the markdown file
    pub fn generate_filename(&self) -> String {
        let safe_title = self.title
            .chars()
            .map(|c| if c.is_alphanumeric() || c == ' ' { c } else { '_' })
            .collect::<String>()
            .replace(' ', "_")
            .to_lowercase();
        
        if safe_title.is_empty() {
            format!("post_{}.md", self.id)
        } else {
            format!("{}_{}.md", safe_title, self.id)
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NostrCredentials {
    pub private_key: String,
    pub public_key: String,
    pub display_name: Option<String>,
    pub about: Option<String>,
    pub picture: Option<String>,
    pub nip05: Option<String>,
}

impl NostrCredentials {
    pub fn new(private_key: String, public_key: String) -> Self {
        Self {
            private_key,
            public_key,
            display_name: None,
            about: None,
            picture: None,
            nip05: None,
        }
    }
}
