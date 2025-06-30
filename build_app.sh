#!/bin/bash
# 🔥 QR Forge - Complete Build Script
# Creates both CLI and macOS App distributions

set -e  # Exit on any error

echo "🔥 Building QR Forge..."

# Clean previous builds
echo "🧹 Cleaning previous builds..."
cargo clean
rm -rf "QR Forge.app" dist/ dist-app/ *.tar.gz *.dmg *.png

# Build the project
echo "📦 Building executables..."
cargo build --release

if [ ! -f "target/release/qr-forge-gui" ]; then
    echo "❌ Build failed: qr-forge-gui not found"
    exit 1
fi

# Create CLI distribution
echo "📁 Creating CLI distribution..."
mkdir -p dist/
cp target/release/qr-forge dist/
cp target/release/qr-forge-gui dist/
cp README.md LICENSE dist/

# Create usage guide for CLI
cat > dist/USAGE.md << 'EOF'
# 🔥 QR Forge - Usage Guide

## GUI Application
Simply run:
```bash
./qr-forge-gui
```

## CLI Tool
```bash
./qr-forge --url "https://example.com" --format png
```

### Examples:
```bash
# Generate PNG QR code
./qr-forge --url "https://github.com" --output "github" --size 1200

# Generate SVG with custom colors  
./qr-forge --url "https://rust-lang.org" --format svg --color "ff6600" --background-color "f0f8ff"

# Launch GUI from CLI
./qr-forge --gui
```

## Features
- Multiple formats: PNG, JPG, BMP, SVG
- Custom colors for SVG
- High resolution support
- Error correction levels
- Real-time preview (GUI)

Made with ❤️ in Rust
EOF

# Create macOS App Bundle
echo "🍎 Creating macOS App Bundle..."
rm -rf "QR Forge.app"
mkdir -p "QR Forge.app/Contents/MacOS" "QR Forge.app/Contents/Resources"

# Copy executable
cp target/release/qr-forge-gui "QR Forge.app/Contents/MacOS/"
chmod +x "QR Forge.app/Contents/MacOS/qr-forge-gui"

# Create Info.plist
cat > "QR Forge.app/Contents/Info.plist" << 'EOF'
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleExecutable</key>
    <string>qr-forge-gui</string>
    <key>CFBundleIconFile</key>
    <string>icon.icns</string>
    <key>CFBundleIdentifier</key>
    <string>com.francesco.qr-forge</string>
    <key>CFBundleInfoDictionaryVersion</key>
    <string>6.0</string>
    <key>CFBundleName</key>
    <string>QR Forge</string>
    <key>CFBundleDisplayName</key>
    <string>QR Forge</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleShortVersionString</key>
    <string>1.0.0</string>
    <key>CFBundleVersion</key>
    <string>1.0.0</string>
    <key>LSMinimumSystemVersion</key>
    <string>10.12</string>
    <key>NSHighResolutionCapable</key>
    <true/>
    <key>LSApplicationCategoryType</key>
    <string>public.app-category.graphics-design</string>
</dict>
</plist>
EOF

# Create app icon
echo "🎨 Creating app icon..."
./target/release/qr-forge --url "QR Forge" --format png --size 512 --margin 2 --output "app_icon" --color "000000" >/dev/null 2>&1

if [ -f "app_icon.png" ] && command -v sips &> /dev/null && command -v iconutil &> /dev/null; then
    mkdir -p icon.iconset
    sips -z 16 16 app_icon.png --out icon.iconset/icon_16x16.png >/dev/null 2>&1
    sips -z 32 32 app_icon.png --out icon.iconset/icon_16x16@2x.png >/dev/null 2>&1
    sips -z 32 32 app_icon.png --out icon.iconset/icon_32x32.png >/dev/null 2>&1
    sips -z 64 64 app_icon.png --out icon.iconset/icon_32x32@2x.png >/dev/null 2>&1
    sips -z 128 128 app_icon.png --out icon.iconset/icon_128x128.png >/dev/null 2>&1
    sips -z 256 256 app_icon.png --out icon.iconset/icon_128x128@2x.png >/dev/null 2>&1
    sips -z 256 256 app_icon.png --out icon.iconset/icon_256x256.png >/dev/null 2>&1
    sips -z 512 512 app_icon.png --out icon.iconset/icon_256x256@2x.png >/dev/null 2>&1
    sips -z 512 512 app_icon.png --out icon.iconset/icon_512x512.png >/dev/null 2>&1
    sips -z 1024 1024 app_icon.png --out icon.iconset/icon_512x512@2x.png >/dev/null 2>&1
    
    iconutil -c icns icon.iconset -o "QR Forge.app/Contents/Resources/icon.icns" >/dev/null 2>&1
    rm -rf icon.iconset app_icon.png
    echo "   ✅ Icon created successfully"
else
    echo "   ⚠️  Icon creation skipped (tools not available)"
fi

# Create App distribution
mkdir -p dist-app/
cp -R "QR Forge.app" dist-app/
cp README.md LICENSE dist-app/

# Create archives
echo "📦 Creating distribution archives..."
tar -czf "QR-Forge-CLI-v1.0.0-macos.tar.gz" -C dist .
tar -czf "QR-Forge-App-v1.0.0-macos.tar.gz" -C dist-app .

# Summary
echo ""
echo "✅ Build completed successfully!"
echo ""
echo "� Distribution files created:"
echo "   📁 dist/ - CLI tools and executables"
echo "   🍎 QR Forge.app - macOS Application"
echo "   � QR-Forge-CLI-v1.0.0-macos.tar.gz - CLI distribution"
echo "   📦 QR-Forge-App-v1.0.0-macos.tar.gz - App distribution"
echo ""
echo "🚀 To test the app:"
echo "   open 'QR Forge.app'"
echo ""
echo "🔧 To test CLI:"
echo "   ./dist/qr-forge-gui"
echo "   ./dist/qr-forge --url 'https://example.com'"
