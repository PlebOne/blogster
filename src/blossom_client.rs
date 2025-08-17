use anyhow::{Context, Result};
use base64::{engine::general_purpose, Engine as _};
use crate::nostr_client::NostrClient;
use nostr_sdk::{EventBuilder, Kind, Tag, Timestamp, JsonUtil};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::path::Path;
use std::sync::Arc;
use tokio::fs;
use tokio::sync::Mutex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlossomSettings {
    pub server_url: String,
}

impl Default for BlossomSettings {
    fn default() -> Self {
        Self {
            server_url: "https://blossom.band".to_string(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct BlossomUploadResponse {
    pub url: String,
    pub sha256: String,
    #[serde(rename = "type")]
    pub content_type: String,
    pub size: u64,
}

#[derive(Clone)]
pub struct BlossomClient {
    settings: BlossomSettings,
    client: reqwest::Client,
    nostr_client: Option<Arc<Mutex<NostrClient>>>,
}

impl BlossomClient {
    pub fn new(settings: BlossomSettings) -> Self {
        Self {
            settings,
            client: reqwest::Client::new(),
            nostr_client: None,
        }
    }

    pub fn set_nostr_client(&mut self, nostr_client: Arc<Mutex<NostrClient>>) {
        self.nostr_client = Some(nostr_client);
    }

    async fn create_auth_header(&self, file_content: &[u8], filename: &str) -> Result<String> {
        let hash = Sha256::digest(file_content);
        let hash_hex = format!("{:x}", hash);
        
        let content = format!("Upload {}", filename);
        
        // Create authorization event according to BUD-02 specification
        // Must use Kind 24242 for Blossom authorization with expiration
        let expiration = Timestamp::now().as_u64() + 600; // 10 minutes from now
        let tags = vec![
            Tag::parse(&["t", "upload"])?,
            Tag::parse(&["x", &hash_hex])?,
            Tag::parse(&["expiration", &expiration.to_string()])?,
        ];
        
        let event_builder = EventBuilder::new(Kind::Custom(24242), content, tags);
        
        if let Some(client) = &self.nostr_client {
            let client_guard = client.lock().await;
            let event = client_guard.sign_event_builder(event_builder).await?;
            let event_json = event.as_json();
            let encoded = general_purpose::STANDARD.encode(&event_json);
            Ok(format!("Nostr {}", encoded))
        } else {
            anyhow::bail!("No Nostr client available for authorization")
        }
    }

    pub async fn upload_file(&self, file_path: &Path) -> Result<String> {
        // Read the file
        let file_content = fs::read(file_path).await
            .with_context(|| format!("Failed to read file: {}", file_path.display()))?;

        // Calculate SHA256 hash
        let mut hasher = Sha256::new();
        hasher.update(&file_content);
        let hash = hasher.finalize();
        let sha256_hex = format!("{:x}", hash);

        // Get file name and content type
        let file_name = file_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("image")
            .to_string();

        let content_type = match file_path
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| ext.to_lowercase())
            .as_deref()
        {
            Some("png") => "image/png",
            Some("jpg") | Some("jpeg") => "image/jpeg",
            Some("gif") => "image/gif",
            Some("webp") => "image/webp",
            Some("svg") => "image/svg+xml",
            _ => "application/octet-stream",
        };

        // Upload to Blossom server
        let upload_url = format!("{}/upload", self.settings.server_url);
        
        tracing::info!("Uploading file to Blossom server: {}", upload_url);

        // Create authorization header according to BUD-02 spec
        let auth_header = self.create_auth_header(&file_content, &file_name).await
            .context("Failed to create authorization header")?;

        tracing::debug!("Using Blossom authorization header: {}", auth_header);

        // Send binary data as request body according to BUD-02 specification
        let response = self
            .client
            .put(&upload_url)
            .header("Authorization", auth_header)
            .header("Content-Type", content_type)
            .header("Content-Length", file_content.len())
            .body(file_content)
            .send()
            .await
            .context("Failed to upload file to Blossom server")?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(anyhow::anyhow!(
                "Blossom upload failed with status {}: {}",
                status,
                error_text
            ));
        }

        let upload_response: BlossomUploadResponse = response
            .json()
            .await
            .context("Failed to parse Blossom upload response")?;

        tracing::info!(
            "Successfully uploaded file. URL: {}, SHA256: {}",
            upload_response.url,
            upload_response.sha256
        );

        // Verify SHA256 hash matches
        if upload_response.sha256 != sha256_hex {
            tracing::warn!(
                "SHA256 mismatch: expected {}, got {}",
                sha256_hex,
                upload_response.sha256
            );
        }

        Ok(upload_response.url)
    }

    pub fn get_server_url(&self) -> &str {
        &self.settings.server_url
    }

    pub fn update_settings(&mut self, settings: BlossomSettings) {
        self.settings = settings;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_blossom_settings() {
        let settings = BlossomSettings::default();
        assert_eq!(settings.server_url, "https://blossom.band");
    }

    #[test]
    fn test_blossom_client_creation() {
        let settings = BlossomSettings::default();
        let client = BlossomClient::new(settings.clone());
        assert_eq!(client.get_server_url(), &settings.server_url);
    }
}
