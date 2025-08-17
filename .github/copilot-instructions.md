# Blogster - Nostr Long-Form Blog Poster

A modern desktop application built with Rust and Egui for creating and publishing long-form blog posts to the Nostr protocol.

## Features

- 🖋️ **Markdown Editor** - Write your posts in Markdown with live preview
- 🔐 **Secure Credentials** - Safely store Nostr credentials using system keyring
- 📂 **Post Management** - Organize drafts and published posts in a clean sidebar
- 🏷️ **Tag System** - Add hashtags to categorize your posts
- 🖼️ **Image Support** - Insert images into your blog posts
- 🚀 **Multi-Relay Publishing** - Publish to the top 5 long-form Nostr relays
- 🎨 **Catppuccin Mocha Theme** - Beautiful, modern dark theme
- 💾 **File-Based Storage** - Posts saved as `.md` files with frontmatter

## Project Status

- [x] Project structure created
- [x] Basic GUI implementation with Egui
- [x] Nostr SDK integration
- [x] Post management features
- [x] Image handling support
- [x] Relay publishing functionality
- [x] Catppuccin Mocha theme applied
- [x] Secure credential storage
- [x] Markdown file support with frontmatter
- [x] Compilation successful

## Tech Stack

- **Rust** - Systems programming language
- **Egui** - Immediate mode GUI framework
- **nostr-sdk** - Nostr protocol implementation
- **egui_commonmark** - Markdown rendering
- **keyring** - Secure credential storage
- **tokio** - Async runtime

## Usage

1. **Run the application**:
   ```bash
   cargo run --release
   ```

2. **Set up Nostr credentials**:
   - Click Settings → Nostr Credentials
   - Generate new keys or import existing ones

3. **Create and publish posts**:
   - Click ➕ to create a new post
   - Write in Markdown format
   - Add tags and images
   - Click 🚀 Publish when ready

## File Structure

Posts are stored as `.md` files with YAML frontmatter in:
- Linux: `~/.config/blogster/posts/`
- macOS: `~/Library/Application Support/blogster/posts/`
- Windows: `%APPDATA%\blogster\posts\`
