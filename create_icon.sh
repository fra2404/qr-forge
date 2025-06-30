#!/bin/bash
# Script to create app icon using existing QR Forge logo

set -e

echo "🎨 Creating QR Forge app icon from image.png..."

# Check if the source image exists
if [ ! -f "image.png" ]; then
    echo "❌ Error: image.png not found in the current directory"
    echo "Please make sure image.png exists in the project root"
    exit 1
fi

echo "✅ Found source icon: image.png"
ICON_SOURCE="image.png"
# Convert to ICNS for macOS app
if command -v sips &> /dev/null; then
    echo "🔄 Converting image.png to macOS ICNS format..."
    mkdir -p icon.iconset
    
    # Create all required icon sizes from the source image
    sips -z 16 16 "$ICON_SOURCE" --out icon.iconset/icon_16x16.png
    sips -z 32 32 "$ICON_SOURCE" --out icon.iconset/icon_16x16@2x.png
    sips -z 32 32 "$ICON_SOURCE" --out icon.iconset/icon_32x32.png
    sips -z 64 64 "$ICON_SOURCE" --out icon.iconset/icon_32x32@2x.png
    sips -z 128 128 "$ICON_SOURCE" --out icon.iconset/icon_128x128.png
    sips -z 256 256 "$ICON_SOURCE" --out icon.iconset/icon_128x128@2x.png
    sips -z 256 256 "$ICON_SOURCE" --out icon.iconset/icon_256x256.png
    sips -z 512 512 "$ICON_SOURCE" --out icon.iconset/icon_256x256@2x.png
    sips -z 512 512 "$ICON_SOURCE" --out icon.iconset/icon_512x512.png
    sips -z 1024 1024 "$ICON_SOURCE" --out icon.iconset/icon_512x512@2x.png
    
    # Create the ICNS file in the correct location
    APP_DIR="build/apps"
    if [ -d "$APP_DIR/QR Forge.app" ]; then
        # App bundle exists in organized build directory
        mkdir -p "$APP_DIR/QR Forge.app/Contents/Resources"
        iconutil -c icns icon.iconset -o "$APP_DIR/QR Forge.app/Contents/Resources/icon.icns"
        echo "📱 Icon installed in: $APP_DIR/QR Forge.app/Contents/Resources/icon.icns"
    else
        # Fallback to root directory (for standalone icon creation)
        mkdir -p "QR Forge.app/Contents/Resources"
        iconutil -c icns icon.iconset -o "QR Forge.app/Contents/Resources/icon.icns"
        echo "📱 Icon installed in: QR Forge.app/Contents/Resources/icon.icns"
    fi
    
    # Cleanup temporary files
    rm -rf icon.iconset
    
    echo "✅ QR Forge icon created successfully from image.png!"
else
    echo "❌ sips command not found (macOS required for ICNS creation)"
    echo "📱 Copying source image as fallback..."
    APP_DIR="build/apps"
    if [ -d "$APP_DIR/QR Forge.app" ]; then
        mkdir -p "$APP_DIR/QR Forge.app/Contents/Resources"
        cp "$ICON_SOURCE" "$APP_DIR/QR Forge.app/Contents/Resources/icon.png"
    else
        mkdir -p "QR Forge.app/Contents/Resources"
        cp "$ICON_SOURCE" "QR Forge.app/Contents/Resources/icon.png"
    fi
fi

echo "🎉 Icon creation completed using your custom QR Forge logo!"
