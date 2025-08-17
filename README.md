# Blogster

A modern desktop application for creating and publishing long-form blog posts to the Nostr protocol.

## Features

- **Markdown Editor** - Write your posts in Markdown with live preview
- **Secure Credentials** - Safely store Nostr credentials using system keyring
- **Post Management** - Organize drafts and published posts in a clean sidebar
- **Tag System** - Add hashtags to categorize your posts
- **Image Support** - Insert images into your blog posts
- **Blossom Integration** - Upload images to decentralized Blossom servers
- **Multi-Relay Publishing** - Publish to multiple Nostr relays simultaneously
- **Dark Theme** - Beautiful Catppuccin Mocha theme
- **File-Based Storage** - Posts saved as `.md` files with frontmatter

## Installation

### Prerequisites

- Rust 1.70 or later
- Linux, macOS, or Windows

### From Source

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/blogster.git
   cd blogster
   ```

2. Build and install:
   ```bash
   ./install.sh
   ```

3. Or build manually:
   ```bash
   cargo build --release
   cp target/release/blogster ~/.local/bin/
   ```

## Usage

### First Time Setup

1. Launch Blogster from your application menu or run `blogster` in terminal
2. Go to Settings → Nostr Credentials
3. Either generate new keys or import existing ones
4. Optionally configure Blossom server settings (defaults to blossom.band)

### Creating Posts

1. Click the "+" button to create a new post
2. Enter title, tags, and optional featured image
3. Write your content in Markdown format
4. Use the preview toggle to see how it will look
5. Click the rocket button to publish to Nostr

### Image Handling

- **Featured Images**: Use the "Upload" button next to the image field to upload a cover image
- **Content Images**: Use the image button in the toolbar to insert images into your post content
- Both automatically upload to your configured Blossom server

## Configuration

### Data Storage

Posts and settings are stored in:
- Linux: `~/.config/blogster/`
- macOS: `~/Library/Application Support/blogster/`
- Windows: `%APPDATA%\blogster\`

### Nostr Credentials

Credentials are securely stored using your system's keyring:
- Linux: Secret Service API
- macOS: Keychain
- Windows: Credential Manager

### Blossom Servers

Default server is `blossom.band` but you can configure your own in Settings → Blossom Settings.

## Technical Details

### Built With

- **Rust** - Systems programming language
- **Egui** - Immediate mode GUI framework
- **nostr-sdk** - Nostr protocol implementation
- **keyring** - Secure credential storage
- **reqwest** - HTTP client for Blossom uploads
- **tokio** - Async runtime

### Nostr Protocol Support

- **NIP-23** - Long-form content events
- **NIP-98** - HTTP Auth for Blossom uploads
- **BUD-02** - Blossom server specification

### Relay Configuration

Publishes to these long-form content relays by default:
- `wss://relay.damus.io`
- `wss://nos.lol`
- `wss://relay.nostr.band`
- `wss://nostr-pub.wellorder.net`
- `wss://relay.snort.social`

## Development

### Building

```bash
# Debug build
cargo build

# Release build
cargo build --release

# Run with debug logging
RUST_LOG=debug cargo run
```

### Project Structure

```
src/
├── main.rs              # Application entry point
├── app.rs               # Main application logic
├── nostr_client.rs      # Nostr protocol handling
├── blossom_client.rs    # Blossom server integration
├── storage.rs           # File and credential storage
├── post.rs              # Blog post data structures
├── theme.rs             # UI theming
└── components/          # UI components
    ├── editor.rs        # Markdown editor
    ├── sidebar.rs       # Post sidebar
    ├── credentials_dialog.rs # Credentials management
    └── publish_dialog.rs # Publishing interface
```

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [Nostr Protocol](https://nostr.com/) - Decentralized social protocol
- [Blossom Protocol](https://github.com/hzrd149/blossom) - Decentralized file storage
- [Catppuccin](https://catppuccin.com/) - Soothing pastel theme
- [Egui](https://github.com/emilk/egui) - Immediate mode GUI framework
