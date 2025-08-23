use crate::post::{BlogPost, NostrCredentials};
use crate::relay_settings::RelaySettings;
use anyhow::{Context, Result};
use nostr_sdk::prelude::*;
use std::time::Duration;

pub struct NostrClient {
    client: Client,
    credentials: Option<NostrCredentials>,
}

impl NostrClient {
    pub fn new() -> Self {
        let client = Client::new(&Keys::generate());
        Self {
            client,
            credentials: None,
        }
    }

    pub fn set_credentials(&mut self, credentials: NostrCredentials) -> Result<()> {
        let secret_key = if credentials.private_key.starts_with("nsec") {
            SecretKey::from_bech32(&credentials.private_key)
                .context("Invalid nsec format")?
        } else {
            SecretKey::from_hex(&credentials.private_key)
                .context("Invalid private key format")?
        };
        
        let keys = Keys::new(secret_key);
        
        self.client = Client::new(&keys);
        
        tracing::info!("Set Nostr credentials for pubkey: {}", credentials.public_key);
        self.credentials = Some(credentials);
        
        Ok(())
    }

    pub fn get_credentials(&self) -> Option<&NostrCredentials> {
        self.credentials.as_ref()
    }

    pub fn has_credentials(&self) -> bool {
        self.credentials.is_some()
    }

    pub async fn connect_to_relays(&self, relay_settings: &RelaySettings) -> Result<()> {
        let relays = relay_settings.get_active_relays();
        
        for relay_url in relays {
            if let Err(e) = self.client.add_relay(&relay_url).await {
                tracing::warn!("Failed to add relay {}: {}", relay_url, e);
            } else {
                tracing::info!("Added relay: {}", relay_url);
            }
        }

        self.client.connect().await;
        
        // Wait a bit for connections to establish
        tokio::time::sleep(Duration::from_secs(2)).await;
        
        Ok(())
    }

    pub async fn publish_long_form_post(&self, post: &BlogPost, relay_settings: &RelaySettings) -> Result<(EventId, Vec<String>)> {
        if self.credentials.is_none() {
            return Err(anyhow::anyhow!("No Nostr credentials configured"));
        }

        if !post.is_ready_to_publish() {
            return Err(anyhow::anyhow!("Post is not ready to publish (missing title or content)"));
        }

        // Create long-form content event (NIP-23)
        let mut tags = vec![
            Tag::title(&post.title),
        ];

        // Add summary if available
        if let Some(summary) = &post.summary {
            tags.push(Tag::custom(TagKind::Custom("summary".into()), vec![summary.clone()]));
        }

        // Add hashtags
        for tag in &post.tags {
            tags.push(Tag::hashtag(tag));
        }

        // Add image if available
        if let Some(image_url) = &post.image_url {
            tags.push(Tag::custom(TagKind::Custom("image".into()), vec![image_url.clone()]));
        }

        // Add published_at timestamp
        tags.push(Tag::custom(
            TagKind::Custom("published_at".into()),
            vec![post.created_at.timestamp().to_string()]
        ));

        // Add identifier for replaceable event (NIP-33)
        let identifier = format!("blogster-{}", post.id);
        tags.push(Tag::identifier(&identifier));

        // Connect to relays before publishing
        self.connect_to_relays(relay_settings).await?;

        // Create the event with kind 30023 for parameterized replaceable long-form content
        let kind = Kind::ParameterizedReplaceable(30023);
        let event_builder = EventBuilder::new(kind, &post.content, tags);
        let event = self.client.sign_event_builder(event_builder).await
            .context("Failed to sign event")?;

        tracing::info!("Publishing event: kind={}, tags={:?}", event.kind, event.tags);

        // Publish to relays
        let event_id = event.id;
        let output = self.client.send_event(event).await
            .context("Failed to publish event")?;

        // Collect successful relays
        let successful_relays: Vec<String> = output
            .success
            .into_iter()
            .map(|url| url.to_string())
            .collect();

        if successful_relays.is_empty() {
            return Err(anyhow::anyhow!("Failed to publish to any relay"));
        }

        tracing::info!(
            "Published post '{}' with event ID {} to {} relays",
            post.title,
            event_id,
            successful_relays.len()
        );

        Ok((event_id, successful_relays))
    }

    pub async fn sign_event(&self, event_builder: EventBuilder) -> Result<Event> {
        self.client.sign_event_builder(event_builder).await
            .context("Failed to sign event")
    }

    pub async fn update_profile(&self, credentials: &NostrCredentials) -> Result<EventId> {
        let mut metadata = Metadata::new();
        
        if let Some(display_name) = &credentials.display_name {
            metadata = metadata.display_name(display_name);
        }
        
        if let Some(about) = &credentials.about {
            metadata = metadata.about(about);
        }
        
        if let Some(picture) = &credentials.picture {
            if let Ok(picture_url) = picture.parse::<nostr_sdk::Url>() {
                metadata = metadata.picture(picture_url);
            }
        }
        
        if let Some(nip05) = &credentials.nip05 {
            metadata = metadata.nip05(nip05);
        }

        let event_builder = EventBuilder::metadata(&metadata);
        let event = self.client.sign_event_builder(event_builder).await
            .context("Failed to sign metadata event")?;

        let event_id = event.id;
        self.client.send_event(event).await
            .context("Failed to publish metadata event")?;

        tracing::info!("Updated profile metadata with event ID: {}", event_id);
        Ok(event_id)
    }

    pub async fn get_relay_status(&self, relay_settings: &RelaySettings) -> Vec<(String, bool)> {
        let mut status = Vec::new();
        
        for relay_url in relay_settings.get_active_relays() {
            // For now, just assume connected. In a real implementation,
            // you would check the actual connection status
            let is_connected = true;
            status.push((relay_url, is_connected));
        }
        
        status
    }

    /// Generate new Nostr credentials
    pub fn generate_credentials() -> NostrCredentials {
        let keys = Keys::generate();
        let private_key = keys.secret_key().unwrap().to_bech32().unwrap();
        let public_key = keys.public_key().to_hex();
        
        NostrCredentials::new(private_key, public_key)
    }

    /// Import credentials from private key (supports both hex and nsec formats)
    pub fn import_credentials_from_private_key(private_key: &str) -> Result<NostrCredentials> {
        let secret_key = if private_key.starts_with("nsec") {
            // Handle bech32 nsec format
            SecretKey::from_bech32(private_key)
                .context("Invalid nsec format")?
        } else {
            // Handle hex format
            SecretKey::from_hex(private_key)
                .context("Invalid private key format")?
        };
        
        let keys = Keys::new(secret_key);
        let public_key = keys.public_key().to_hex();
        
        Ok(NostrCredentials::new(private_key.to_string(), public_key))
    }

    /// Validate a private key format (supports both hex and nsec)
    pub fn validate_private_key(private_key: &str) -> bool {
        if private_key.starts_with("nsec") {
            SecretKey::from_bech32(private_key).is_ok()
        } else {
            SecretKey::from_hex(private_key).is_ok()
        }
    }

    /// Get public key from private key (supports both hex and nsec formats)
    pub fn get_public_key_from_private(private_key: &str) -> Result<String> {
        let secret_key = if private_key.starts_with("nsec") {
            SecretKey::from_bech32(private_key)
                .context("Invalid nsec format")?
        } else {
            SecretKey::from_hex(private_key)
                .context("Invalid private key format")?
        };
        
        let keys = Keys::new(secret_key);
        Ok(keys.public_key().to_hex())
    }

    pub async fn sign_event_builder(&self, event_builder: EventBuilder) -> Result<Event> {
        self.client.sign_event_builder(event_builder).await
            .context("Failed to sign event")
    }
}
