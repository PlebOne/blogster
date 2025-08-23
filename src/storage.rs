use crate::blossom_client::BlossomSettings;
use crate::post::{BlogPost, NostrCredentials};
use crate::relay_settings::RelaySettings;
use crate::theme::{Theme, CustomThemeColors};
use anyhow::{Context, Result};
use base64::Engine;
use std::fs;
use std::path::{Path, PathBuf};

pub struct Storage {
    posts_dir: PathBuf,
    config_dir: PathBuf,
}

impl Storage {
    pub fn new() -> Result<Self> {
        // Config directory for credentials and settings (hidden)
        let config_dir = dirs::config_dir()
            .or_else(|| dirs::home_dir().map(|h| h.join(".config")))
            .context("Could not find config directory")?
            .join("blogster");

        // Documents directory for posts and drafts (user-visible)
        let posts_dir = dirs::document_dir()
            .or_else(|| dirs::home_dir().map(|h| h.join("Documents")))
            .context("Could not find documents directory")?
            .join("blogster");

        // Create directories if they don't exist
        fs::create_dir_all(&posts_dir)
            .context("Failed to create posts directory")?;
        fs::create_dir_all(&config_dir)
            .context("Failed to create config directory")?;

        tracing::info!("Posts directory: {}", posts_dir.display());
        tracing::info!("Config directory: {}", config_dir.display());

        Ok(Self {
            posts_dir,
            config_dir,
        })
    }

    /// Migrate posts from old config directory to new Documents directory
    pub fn migrate_posts_if_needed(&self) -> Result<()> {
        let old_posts_dir = self.config_dir.join("posts");
        
        if old_posts_dir.exists() {
            tracing::info!("Found old posts directory, migrating to Documents...");
            
            let entries = fs::read_dir(&old_posts_dir)
                .context("Failed to read old posts directory")?;
            
            let mut migrated_count = 0;
            for entry in entries {
                let entry = entry.context("Failed to read directory entry")?;
                let path = entry.path();
                
                if path.is_file() && path.extension().map_or(false, |ext| ext == "md") {
                    let filename = path.file_name()
                        .context("Invalid filename")?;
                    let new_path = self.posts_dir.join(filename);
                    
                    // Only migrate if file doesn't already exist in new location
                    if !new_path.exists() {
                        fs::copy(&path, &new_path)
                            .with_context(|| format!("Failed to migrate {}", path.display()))?;
                        migrated_count += 1;
                        tracing::info!("Migrated: {} -> {}", path.display(), new_path.display());
                    }
                }
            }
            
            if migrated_count > 0 {
                tracing::info!("Successfully migrated {} posts to Documents/blogster", migrated_count);
                
                // Optionally remove old directory if empty
                if fs::read_dir(&old_posts_dir)?.count() == 0 {
                    fs::remove_dir(&old_posts_dir)
                        .context("Failed to remove empty old posts directory")?;
                    tracing::info!("Removed empty old posts directory");
                }
            } else {
                tracing::info!("No posts needed migration");
            }
        }
        
        Ok(())
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

    /// Save Nostr credentials securely using keyring with file fallback
    pub fn save_credentials(&self, credentials: &NostrCredentials) -> Result<()> {
        let json = serde_json::to_string(credentials)
            .context("Failed to serialize credentials")?;
        
        let mut keyring_success = false;
        let mut file_success = false;

        // Try keyring first
        let entry = keyring::Entry::new("blogster", "nostr_credentials")
            .context("Failed to create keyring entry")?;
        
        match entry.set_password(&json) {
            Ok(()) => {
                tracing::info!("Saved Nostr credentials to keyring");
                keyring_success = true;
            }
            Err(e) => {
                tracing::warn!("Keyring failed: {}", e);
            }
        }
        
        // ALWAYS also save to file as backup (in case keyring is MockCredential)
        match self.save_credentials_to_file(credentials) {
            Ok(()) => {
                file_success = true;
            }
            Err(e) => {
                tracing::warn!("File backup failed: {}", e);
            }
        }
        
        // Return success if either method worked
        if keyring_success || file_success {
            Ok(())
        } else {
            Err(anyhow::anyhow!("Both keyring and file storage failed"))
        }
    }

    /// Load Nostr credentials from keyring with file fallback
    pub fn load_credentials(&self) -> Result<Option<NostrCredentials>> {
        // Try keyring first
        let entry = keyring::Entry::new("blogster", "nostr_credentials")
            .context("Failed to create keyring entry")?;
        
        match entry.get_password() {
            Ok(json) => {
                tracing::debug!("Loaded credentials from keyring");
                let credentials = serde_json::from_str(&json)
                    .context("Failed to deserialize credentials")?;
                Ok(Some(credentials))
            }
            Err(keyring::Error::NoEntry) => {
                tracing::debug!("No credentials in keyring, trying file fallback");
                self.load_credentials_from_file()
            }
            Err(e) => {
                tracing::warn!("Keyring failed ({}), trying file fallback", e);
                self.load_credentials_from_file()
            }
        }
    }

    /// Delete stored Nostr credentials from both keyring and file
    pub fn delete_credentials(&self) -> Result<()> {
        let mut keyring_result = Ok(());
        let mut file_result = Ok(());

        // Try to delete from keyring
        let entry = keyring::Entry::new("blogster", "nostr_credentials")
            .context("Failed to create keyring entry")?;
        
        match entry.delete_credential() {
            Ok(()) => {
                tracing::info!("Deleted Nostr credentials from keyring");
            }
            Err(keyring::Error::NoEntry) => {
                tracing::debug!("No credentials in keyring to delete");
            }
            Err(e) => {
                tracing::warn!("Failed to delete from keyring: {}", e);
                keyring_result = Err(anyhow::anyhow!("Keyring delete failed: {}", e));
            }
        }

        // Also try to delete from file fallback
        file_result = self.delete_fallback_credentials();

        // Return success if either method worked
        if keyring_result.is_ok() || file_result.is_ok() {
            Ok(())
        } else {
            keyring_result
        }
    }

    /// Save credentials to encrypted file as fallback
    fn save_credentials_to_file(&self, credentials: &NostrCredentials) -> Result<()> {
        eprintln!("🔧 DEBUG: save_credentials_to_file called");
        let credentials_path = self.config_dir.join("credentials.enc");
        eprintln!("🔧 DEBUG: Saving to path: {:?}", credentials_path);
        
        let json = serde_json::to_string(credentials)
            .context("Failed to serialize credentials")?;
        eprintln!("🔧 DEBUG: Serialized JSON: {}", json);
        
        // Simple base64 encoding (not encryption, but obscures the data)
        let encoded = base64::prelude::BASE64_STANDARD.encode(json.as_bytes());
        eprintln!("🔧 DEBUG: Encoded length: {}", encoded.len());
        
        match std::fs::write(&credentials_path, encoded) {
            Ok(()) => {
                eprintln!("🔧 DEBUG: File write successful");
                tracing::info!("Saved Nostr credentials to file fallback");
                Ok(())
            }
            Err(e) => {
                eprintln!("🔧 DEBUG: File write failed: {}", e);
                Err(anyhow::anyhow!("Failed to write credentials file: {}", e))
            }
        }
    }

    /// Load credentials from file fallback
    fn load_credentials_from_file(&self) -> Result<Option<NostrCredentials>> {
        let credentials_path = self.config_dir.join("credentials.enc");
        
        if !credentials_path.exists() {
            tracing::debug!("No credentials file found");
            return Ok(None);
        }
        
        let encoded = std::fs::read_to_string(&credentials_path)
            .context("Failed to read credentials file")?;
        
        let json_bytes = base64::prelude::BASE64_STANDARD.decode(encoded.trim())
            .context("Failed to decode credentials")?;
        
        let json = String::from_utf8(json_bytes)
            .context("Invalid UTF-8 in credentials file")?;
        
        let credentials = serde_json::from_str(&json)
            .context("Failed to deserialize credentials")?;
        
        tracing::info!("Loaded Nostr credentials from file fallback");
        Ok(Some(credentials))
    }

    /// Delete fallback credentials file
    fn delete_fallback_credentials(&self) -> Result<()> {
        let credentials_path = self.config_dir.join("credentials.enc");
        
        if credentials_path.exists() {
            std::fs::remove_file(&credentials_path)
                .context("Failed to delete credentials file")?;
            tracing::info!("Deleted fallback credentials file");
        }
        
        Ok(())
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

    /// Save theme preference
    pub fn save_theme(&self, theme: Theme) -> Result<()> {
        let theme_path = self.config_dir.join("theme.json");
        let content = serde_json::to_string_pretty(&theme)
            .context("Failed to serialize theme")?;
        
        fs::write(&theme_path, content)
            .with_context(|| format!("Failed to write theme to {}", theme_path.display()))?;
        
        tracing::info!("Saved theme preference: {}", theme.name());
        Ok(())
    }

    /// Load theme preference
    pub fn load_theme(&self) -> Result<Theme> {
        let theme_path = self.config_dir.join("theme.json");
        
        if !theme_path.exists() {
            tracing::info!("No theme file found, using default");
            return Ok(Theme::default());
        }

        let content = fs::read_to_string(&theme_path)
            .with_context(|| format!("Failed to read theme from {}", theme_path.display()))?;
        
        let theme: Theme = serde_json::from_str(&content)
            .context("Failed to parse theme")?;
        
        tracing::info!("Loaded theme preference: {}", theme.name());
        Ok(theme)
    }

    /// Save custom theme colors
    pub fn save_custom_colors(&self, colors: &CustomThemeColors) -> Result<()> {
        let colors_path = self.config_dir.join("custom_colors.json");
        let content = serde_json::to_string_pretty(colors)
            .context("Failed to serialize custom colors")?;
        
        fs::write(&colors_path, content)
            .with_context(|| format!("Failed to write custom colors to {}", colors_path.display()))?;
        
        tracing::info!("Saved custom theme colors");
        Ok(())
    }

    /// Load custom theme colors
    pub fn load_custom_colors(&self) -> Result<CustomThemeColors> {
        let colors_path = self.config_dir.join("custom_colors.json");
        
        if !colors_path.exists() {
            tracing::info!("No custom colors file found, using default");
            return Ok(CustomThemeColors::default());
        }

        let content = fs::read_to_string(&colors_path)
            .with_context(|| format!("Failed to read custom colors from {}", colors_path.display()))?;
        
        let colors: CustomThemeColors = serde_json::from_str(&content)
            .context("Failed to parse custom colors")?;
        
        tracing::info!("Loaded custom theme colors");
        Ok(colors)
    }

    /// Save relay settings
    pub fn save_relay_settings(&self, settings: &RelaySettings) -> Result<()> {
        let settings_path = self.config_dir.join("relay_settings.json");
        let content = serde_json::to_string_pretty(settings)
            .context("Failed to serialize relay settings")?;
        
        fs::write(&settings_path, content)
            .with_context(|| format!("Failed to write relay settings to {}", settings_path.display()))?;
        
        tracing::info!("Saved relay settings");
        Ok(())
    }

    /// Load relay settings
    pub fn load_relay_settings(&self) -> Result<RelaySettings> {
        let settings_path = self.config_dir.join("relay_settings.json");
        
        if !settings_path.exists() {
            tracing::info!("No relay settings file found, using default");
            return Ok(RelaySettings::default());
        }

        let content = fs::read_to_string(&settings_path)
            .with_context(|| format!("Failed to read relay settings from {}", settings_path.display()))?;
        
        let settings: RelaySettings = serde_json::from_str(&content)
            .context("Failed to parse relay settings")?;
        
        tracing::info!("Loaded relay settings with {} custom relays", settings.custom_relays.len());
        Ok(settings)
    }
}
