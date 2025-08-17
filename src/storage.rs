use crate::blossom_client::BlossomSettings;
use crate::post::{BlogPost, NostrCredentials};
use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

pub struct Storage {
    posts_dir: PathBuf,
    config_dir: PathBuf,
}

impl Storage {
    pub fn new() -> Result<Self> {
        let app_dir = dirs::config_dir()
            .or_else(|| dirs::home_dir().map(|h| h.join(".config")))
            .context("Could not find config directory")?
            .join("blogster");

        let posts_dir = app_dir.join("posts");
        let config_dir = app_dir;

        // Create directories if they don't exist
        fs::create_dir_all(&posts_dir)
            .context("Failed to create posts directory")?;
        fs::create_dir_all(&config_dir)
            .context("Failed to create config directory")?;

        Ok(Self {
            posts_dir,
            config_dir,
        })
    }

    /// Save a blog post as a markdown file
    pub fn save_post(&self, post: &BlogPost) -> Result<PathBuf> {
        let filename = post.generate_filename();
        let file_path = self.posts_dir.join(&filename);
        
        let content = post.to_markdown_with_frontmatter();
        fs::write(&file_path, content)
            .with_context(|| format!("Failed to save post to {}", file_path.display()))?;
        
        tracing::info!("Saved post '{}' to {}", post.title, file_path.display());
        Ok(file_path)
    }

    /// Load a blog post from a markdown file
    pub fn load_post(&self, file_path: &Path) -> Result<BlogPost> {
        let content = fs::read_to_string(file_path)
            .with_context(|| format!("Failed to read post from {}", file_path.display()))?;
        
        BlogPost::from_markdown_with_frontmatter(&content, Some(file_path.to_path_buf()))
    }

    /// Load all blog posts from the posts directory
    pub fn load_all_posts(&self) -> Result<Vec<BlogPost>> {
        let mut posts = Vec::new();
        
        if !self.posts_dir.exists() {
            return Ok(posts);
        }

        let entries = fs::read_dir(&self.posts_dir)
            .context("Failed to read posts directory")?;

        for entry in entries {
            let entry = entry.context("Failed to read directory entry")?;
            let path = entry.path();
            
            if path.extension().and_then(|s| s.to_str()) == Some("md") {
                match self.load_post(&path) {
                    Ok(post) => {
                        tracing::debug!("Loaded post: {}", post.title);
                        posts.push(post);
                    }
                    Err(e) => {
                        tracing::warn!("Failed to load post from {}: {}", path.display(), e);
                    }
                }
            }
        }

        // Sort posts by updated_at descending (newest first)
        posts.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
        
        Ok(posts)
    }

    /// Delete a blog post file
    pub fn delete_post(&self, post: &BlogPost) -> Result<()> {
        if let Some(file_path) = &post.file_path {
            if file_path.exists() {
                fs::remove_file(file_path)
                    .with_context(|| format!("Failed to delete post file {}", file_path.display()))?;
                tracing::info!("Deleted post file: {}", file_path.display());
            }
        } else {
            // If no file path is stored, try to find and delete by generated filename
            let filename = post.generate_filename();
            let file_path = self.posts_dir.join(&filename);
            if file_path.exists() {
                fs::remove_file(&file_path)
                    .with_context(|| format!("Failed to delete post file {}", file_path.display()))?;
                tracing::info!("Deleted post file: {}", file_path.display());
            }
        }
        Ok(())
    }

    /// Save Nostr credentials securely using keyring
    pub fn save_credentials(&self, credentials: &NostrCredentials) -> Result<()> {
        let entry = keyring::Entry::new("blogster", "nostr_credentials")
            .context("Failed to create keyring entry")?;
        
        let json = serde_json::to_string(credentials)
            .context("Failed to serialize credentials")?;
        
        entry.set_password(&json)
            .context("Failed to save credentials to keyring")?;
        
        tracing::info!("Saved Nostr credentials securely");
        Ok(())
    }

    /// Load Nostr credentials from keyring
    pub fn load_credentials(&self) -> Result<Option<NostrCredentials>> {
        let entry = keyring::Entry::new("blogster", "nostr_credentials")
            .context("Failed to create keyring entry")?;
        
        match entry.get_password() {
            Ok(json) => {
                let credentials = serde_json::from_str(&json)
                    .context("Failed to deserialize credentials")?;
                Ok(Some(credentials))
            }
            Err(keyring::Error::NoEntry) => Ok(None),
            Err(e) => Err(anyhow::anyhow!("Failed to load credentials: {}", e)),
        }
    }

    /// Delete stored Nostr credentials
    pub fn delete_credentials(&self) -> Result<()> {
        let entry = keyring::Entry::new("blogster", "nostr_credentials")
            .context("Failed to create keyring entry")?;
        
        match entry.delete_credential() {
            Ok(()) => {
                tracing::info!("Deleted Nostr credentials");
                Ok(())
            }
            Err(keyring::Error::NoEntry) => {
                tracing::debug!("No credentials to delete");
                Ok(())
            }
            Err(e) => Err(anyhow::anyhow!("Failed to delete credentials: {}", e)),
        }
    }

    /// Get the posts directory path
    pub fn posts_dir(&self) -> &Path {
        &self.posts_dir
    }

    /// Get the config directory path
    pub fn config_dir(&self) -> &Path {
        &self.config_dir
    }

    /// Export a post to a specific location
    pub fn export_post(&self, post: &BlogPost, destination: &Path) -> Result<()> {
        let content = post.to_markdown_with_frontmatter();
        fs::write(destination, content)
            .with_context(|| format!("Failed to export post to {}", destination.display()))?;
        
        tracing::info!("Exported post '{}' to {}", post.title, destination.display());
        Ok(())
    }

    /// Import a post from a specific location
    pub fn import_post(&self, source: &Path) -> Result<BlogPost> {
        let content = fs::read_to_string(source)
            .with_context(|| format!("Failed to read post from {}", source.display()))?;
        
        let mut post = BlogPost::from_markdown_with_frontmatter(&content, None)?;
        
        // Save the imported post to our posts directory
        let file_path = self.save_post(&post)?;
        post.file_path = Some(file_path);
        
        tracing::info!("Imported post '{}' from {}", post.title, source.display());
        Ok(post)
    }

    pub fn save_blossom_settings(&self, settings: &BlossomSettings) -> Result<()> {
        let settings_path = self.config_dir.join("blossom_settings.json");
        let content = serde_json::to_string_pretty(settings)
            .context("Failed to serialize Blossom settings")?;
        
        fs::write(&settings_path, content)
            .with_context(|| format!("Failed to save Blossom settings to {}", settings_path.display()))?;
        
        tracing::info!("Saved Blossom settings to {}", settings_path.display());
        Ok(())
    }

    pub fn load_blossom_settings(&self) -> Result<BlossomSettings> {
        let settings_path = self.config_dir.join("blossom_settings.json");
        
        if !settings_path.exists() {
            tracing::info!("No Blossom settings file found, using defaults");
            return Ok(BlossomSettings::default());
        }

        let content = fs::read_to_string(&settings_path)
            .with_context(|| format!("Failed to read Blossom settings from {}", settings_path.display()))?;
        
        let settings: BlossomSettings = serde_json::from_str(&content)
            .context("Failed to parse Blossom settings")?;
        
        tracing::info!("Loaded Blossom settings from {}", settings_path.display());
        Ok(settings)
    }
}
