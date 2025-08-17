#!/bin/bash
# Launch script for Blogster

echo "🚀 Starting Blogster - Nostr Long-Form Blog Poster..."
echo "Built with Rust + Egui + Catppuccin Mocha theme"
echo ""

# Check if cargo is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Cargo not found. Please install Rust first:"
    echo "   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi

# Build and run in release mode for better performance
echo "🔨 Building Blogster (this may take a moment on first run)..."
cargo run --release

echo ""
echo "👋 Thanks for using Blogster!"
