#!/bin/bash

# Blogster Installation Script
set -e

echo "🌸 Installing Blogster - Nostr Long-Form Blog Poster"
echo "=================================================="

# Create local bin directory if it doesn't exist
mkdir -p ~/.local/bin

# Copy the binary
if [ -f "./target/release/blogster" ]; then
    cp ./target/release/blogster ~/.local/bin/blogster
    chmod +x ~/.local/bin/blogster
    echo "✅ Blogster binary installed to ~/.local/bin/blogster"
else
    echo "❌ Release binary not found. Building now..."
    cargo build --release
    cp ./target/release/blogster ~/.local/bin/blogster
    chmod +x ~/.local/bin/blogster
    echo "✅ Blogster binary built and installed to ~/.local/bin/blogster"
fi

# Create desktop entry
mkdir -p ~/.local/share/applications

cat > ~/.local/share/applications/blogster.desktop << EOF
[Desktop Entry]
Version=1.0
Type=Application
Name=Blogster
GenericName=Blog Publisher
Comment=Nostr Long-Form Blog Poster
Exec=$HOME/.local/bin/blogster
Icon=accessories-text-editor
Terminal=false
Categories=Office;Publishing;Network;TextEditor;
Keywords=nostr;blog;publishing;markdown;editor;
StartupNotify=true
MimeType=text/markdown;text/plain;
StartupWMClass=blogster
EOF

echo "✅ Desktop entry created"

# Update desktop database
if command -v update-desktop-database &> /dev/null; then
    update-desktop-database ~/.local/share/applications/ 2>/dev/null || true
    echo "✅ Desktop database updated"
fi

# For Sway/Wayland environments, also try refreshing the applications cache
if command -v gtk-update-icon-cache &> /dev/null; then
    gtk-update-icon-cache ~/.local/share/applications/ 2>/dev/null || true
fi

# Check if ~/.local/bin is in PATH
if [[ ":$PATH:" != *":$HOME/.local/bin:"* ]]; then
    echo ""
    echo "⚠️  WARNING: ~/.local/bin is not in your PATH"
    echo "   Add this line to your ~/.bashrc or ~/.zshrc:"
    echo "   export PATH=\"\$HOME/.local/bin:\$PATH\""
    echo ""
    echo "   Or run Blogster directly with: ~/.local/bin/blogster"
else
    echo "✅ ~/.local/bin is in your PATH"
fi

echo ""
echo "🎉 Installation complete!"
echo ""
echo "You can now run Blogster by:"
echo "  • Typing 'blogster' in terminal (if ~/.local/bin is in PATH)"
echo "  • Running '~/.local/bin/blogster' directly"
echo "  • Finding 'Blogster' in your application menu"
echo "  • Clicking the desktop icon (if your desktop environment supports it)"
echo ""
echo "📝 First time setup:"
echo "  1. Open Blogster"
echo "  2. Go to Settings → Nostr Credentials"
echo "  3. Generate new keys or import existing ones"
echo "  4. Start writing your blog posts!"
echo ""
