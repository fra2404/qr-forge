#!/bin/bash
# 🔥 QR Forge - Complete Build Script
# Creates both CLI and macOS App distributions in organized folders

set -e  # Exit on any error

echo "🔥 Building QR Forge..."

# Define build directories
BUILD_DIR="build"
CLI_DIR="$BUILD_DIR/cli"
APP_DIR="$BUILD_DIR/apps"
DIST_DIR="$BUILD_DIR/distributions"

# Clean previous builds (preserve image.png)
echo "🧹 Cleaning previous builds..."
cargo clean
rm -rf "$BUILD_DIR" dist/ dist-app/ *.tar.gz *.dmg
# Remove generated PNG files but keep image.png (our app icon source)
find . -name "*.png" -not -name "image.png" -not -path "./target/*" -delete

# Create build structure
echo "📁 Creating build directories..."
mkdir -p "$CLI_DIR" "$APP_DIR" "$DIST_DIR"

# Build the project
echo "📦 Building executables..."
cargo build --release

if [ ! -f "target/release/qr-forge-gui" ]; then
    echo "❌ Build failed: qr-forge-gui not found"
    exit 1
fi

# Create CLI distribution
echo "📁 Creating CLI distribution..."
cp target/release/qr-forge "$CLI_DIR/"
cp target/release/qr-forge-gui "$CLI_DIR/"
cp README.md LICENSE "$CLI_DIR/"

# Create usage guide for CLI
cat > "$CLI_DIR/USAGE.md" << 'EOF'
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
APP_NAME="QR Forge.app"
APP_PATH="$APP_DIR/$APP_NAME"
rm -rf "$APP_PATH"
mkdir -p "$APP_PATH/Contents/MacOS" "$APP_PATH/Contents/Resources"

# Copy executable
cp target/release/qr-forge-gui "$APP_PATH/Contents/MacOS/"
chmod +x "$APP_PATH/Contents/MacOS/qr-forge-gui"

# Create Info.plist
cat > "$APP_PATH/Contents/Info.plist" << 'EOF'
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
if [ -f "image.png" ]; then
    echo "   ✅ Using custom icon from image.png"
    # Use our custom icon creation script
    ./create_icon.sh
    # Move the icon to the app bundle if it was created in the wrong place
    if [ -f "QR Forge.app/Contents/Resources/icon.icns" ]; then
        cp "QR Forge.app/Contents/Resources/icon.icns" "$APP_PATH/Contents/Resources/"
        rm -rf "QR Forge.app"
    fi
else
    echo "   ⚠️  image.png not found, creating default icon..."
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
        
        iconutil -c icns icon.iconset -o "$APP_PATH/Contents/Resources/icon.icns" >/dev/null 2>&1
        rm -rf icon.iconset app_icon.png
        echo "   ✅ Icon created successfully"
    else
        echo "   ⚠️  Icon creation skipped (tools not available)"
    fi
fi

# Create distribution packages
echo "📦 Creating distribution packages..."

# Copy app and documentation to distributions folder
cp -R "$APP_PATH" "$DIST_DIR/"
cp README.md LICENSE "$DIST_DIR/"

# Create archives
echo "📦 Creating distribution archives..."
cd "$CLI_DIR"
tar -czf "../../$DIST_DIR/QR-Forge-CLI-v1.0.0-macos.tar.gz" .
cd - >/dev/null

cd "$APP_DIR"
tar -czf "../distributions/QR-Forge-App-v1.0.0-macos.tar.gz" .
cd - >/dev/null

# Summary
echo ""
echo "✅ Build completed successfully!"
echo ""
echo "📂 Organized build structure:"
echo "   📁 $CLI_DIR/ - CLI tools and executables"
echo "   🍎 $APP_PATH - macOS Application Bundle"
echo "   📦 $DIST_DIR/ - Distribution files and archives"
echo ""
echo "📦 Distribution archives created:"
echo "   🔧 QR-Forge-CLI-v1.0.0-macos.tar.gz - CLI distribution"
echo "   📱 QR-Forge-App-v1.0.0-macos.tar.gz - App distribution"
echo ""
echo "🚀 To test the app:"
echo "   open '$APP_PATH'"
echo ""
echo "🔧 To test CLI:"
echo "   ./$CLI_DIR/qr-forge-gui"
echo "   ./$CLI_DIR/qr-forge --url 'https://example.com'"
echo ""
echo "🎯 All builds are organized in the '$BUILD_DIR' directory!"
