use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelaySettings {
    pub custom_relays: Vec<String>,
    pub use_default_relays: bool,
    pub use_custom_relays: bool,
}

impl Default for RelaySettings {
    fn default() -> Self {
        Self {
            custom_relays: Vec::new(),
            use_default_relays: true,
            use_custom_relays: false,
        }
    }
}

impl RelaySettings {
    pub fn new() -> Self {
        Self::default()
    }

    /// Get the default long-form content relays for Nostr
    pub fn get_default_relays() -> Vec<String> {
        vec![
            "wss://relay.damus.io".to_string(),
            "wss://nos.lol".to_string(),
            "wss://relay.nostr.band".to_string(),
            "wss://nostr-pub.wellorder.net".to_string(),
            "wss://relay.snort.social".to_string(),
        ]
    }

    /// Get all active relays based on current settings
    pub fn get_active_relays(&self) -> Vec<String> {
        let mut relays = Vec::new();

        // Add default relays if enabled
        if self.use_default_relays {
            relays.extend(Self::get_default_relays());
        }

        // Add custom relays if enabled
        if self.use_custom_relays {
            relays.extend(self.custom_relays.clone());
        }

        // Remove duplicates and ensure we have at least one relay
        relays.sort();
        relays.dedup();

        // If no relays are selected, fall back to defaults
        if relays.is_empty() {
            relays = Self::get_default_relays();
        }

        relays
    }

    /// Add a custom relay
    pub fn add_relay(&mut self, relay_url: String) -> Result<(), String> {
        // Basic validation
        if !relay_url.starts_with("wss://") && !relay_url.starts_with("ws://") {
            return Err("Relay URL must start with wss:// or ws://".to_string());
        }

        if relay_url.len() < 10 {
            return Err("Relay URL is too short".to_string());
        }

        // Check for duplicates
        if self.custom_relays.contains(&relay_url) {
            return Err("Relay already exists".to_string());
        }

        self.custom_relays.push(relay_url);
        Ok(())
    }

    /// Remove a custom relay
    pub fn remove_relay(&mut self, relay_url: &str) -> bool {
        if let Some(index) = self.custom_relays.iter().position(|r| r == relay_url) {
            self.custom_relays.remove(index);
            true
        } else {
            false
        }
    }

    /// Validate a relay URL
    pub fn validate_relay_url(url: &str) -> Result<(), String> {
        if !url.starts_with("wss://") && !url.starts_with("ws://") {
            return Err("URL must start with wss:// or ws://".to_string());
        }

        if url.len() < 10 {
            return Err("URL is too short".to_string());
        }

        // Basic URL validation
        if !url.contains('.') {
            return Err("Invalid URL format".to_string());
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_relay_validation() {
        assert!(RelaySettings::validate_relay_url("wss://relay.damus.io").is_ok());
        assert!(RelaySettings::validate_relay_url("ws://localhost:8080").is_ok());
        assert!(RelaySettings::validate_relay_url("https://example.com").is_err());
        assert!(RelaySettings::validate_relay_url("wss://").is_err());
    }

    #[test]
    fn test_add_remove_relay() {
        let mut settings = RelaySettings::new();
        
        assert!(settings.add_relay("wss://test.relay.com".to_string()).is_ok());
        assert_eq!(settings.custom_relays.len(), 1);
        
        assert!(settings.add_relay("wss://test.relay.com".to_string()).is_err()); // Duplicate
        
        assert!(settings.remove_relay("wss://test.relay.com"));
        assert_eq!(settings.custom_relays.len(), 0);
    }

    #[test]
    fn test_get_active_relays() {
        let mut settings = RelaySettings::new();
        settings.use_default_relays = true;
        settings.use_custom_relays = false;
        
        let relays = settings.get_active_relays();
        assert_eq!(relays.len(), 5); // Default relays
        
        settings.add_relay("wss://custom.relay.com".to_string()).unwrap();
        settings.use_custom_relays = true;
        
        let relays = settings.get_active_relays();
        assert_eq!(relays.len(), 6); // Default + custom
    }
}
