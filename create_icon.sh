#!/bin/bash
# Script per creare una semplice icona PNG che poi convertiremo in ICNS

# Generiamo un QR code come icona usando l'app stessa
if [ -f "target/release/qr-forge" ]; then
    echo "Creating app icon..."
    ./target/release/qr-forge --url "https://github.com/francesco/qr-forge" --format png --size 512 --margin 2 --output "app_icon"
    
    # Su macOS puoi convertire PNG in ICNS con sips (se disponibile)
    if command -v sips &> /dev/null; then
        mkdir -p icon.iconset
        # Crea diverse dimensioni per l'iconset
        sips -z 16 16 app_icon.png --out icon.iconset/icon_16x16.png
        sips -z 32 32 app_icon.png --out icon.iconset/icon_16x16@2x.png
        sips -z 32 32 app_icon.png --out icon.iconset/icon_32x32.png
        sips -z 64 64 app_icon.png --out icon.iconset/icon_32x32@2x.png
        sips -z 128 128 app_icon.png --out icon.iconset/icon_128x128.png
        sips -z 256 256 app_icon.png --out icon.iconset/icon_128x128@2x.png
        sips -z 256 256 app_icon.png --out icon.iconset/icon_256x256.png
        sips -z 512 512 app_icon.png --out icon.iconset/icon_256x256@2x.png
        sips -z 512 512 app_icon.png --out icon.iconset/icon_512x512.png
        sips -z 1024 1024 app_icon.png --out icon.iconset/icon_512x512@2x.png
        
        # Crea il file ICNS
        iconutil -c icns icon.iconset -o "QR Forge.app/Contents/Resources/icon.icns"
        
        # Cleanup
        rm -rf icon.iconset app_icon.png
        echo "Icon created successfully!"
    else
        echo "sips command not found, copying PNG as placeholder"
        cp app_icon.png "QR Forge.app/Contents/Resources/icon.png"
    fi
else
    echo "Executable not found, skipping icon creation"
fi
