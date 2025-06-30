#!/bin/bash
# Script to prepare QR Forge for Homebrew Core submission

set -e

echo "🍺 QR Forge - Homebrew Core Preparation"
echo "======================================="

# Check if we have a recent release
LATEST_TAG=$(git describe --tags --abbrev=0 2>/dev/null || echo "none")
echo "📋 Latest tag: $LATEST_TAG"

if [ "$LATEST_TAG" = "none" ]; then
    echo "❌ No tags found. Please create a release first."
    exit 1
fi

# Calculate SHA256 for the latest release
echo "🔍 Calculating SHA256 for release $LATEST_TAG..."
TARBALL_URL="https://github.com/fra2404/qr-forge/archive/refs/tags/${LATEST_TAG}.tar.gz"
TEMP_FILE="/tmp/qr-forge-${LATEST_TAG}.tar.gz"

echo "📥 Downloading release tarball..."
curl -L "$TARBALL_URL" -o "$TEMP_FILE"

echo "🔐 Calculating SHA256..."
SHA256=$(shasum -a 256 "$TEMP_FILE" | cut -d' ' -f1)
echo "✅ SHA256: $SHA256"

# Update the formula with correct SHA256 and version
echo "📝 Updating Homebrew formula..."
if [ -f "homebrew/qr-forge.rb" ]; then
    # Create a backup
    cp homebrew/qr-forge.rb homebrew/qr-forge.rb.backup
    
    # Update SHA256 and version
    sed -i '' "s/PLACEHOLDER_SHA256/$SHA256/g" homebrew/qr-forge.rb
    sed -i '' "s/v1\.0\.4/$LATEST_TAG/g" homebrew/qr-forge.rb
    
    echo "✅ Formula updated successfully"
else
    echo "❌ Formula file not found at homebrew/qr-forge.rb"
    exit 1
fi

echo ""
echo "🎯 Next steps for Homebrew Core submission:"
echo "1. Fork https://github.com/Homebrew/homebrew-core"
echo "2. Clone your fork: git clone https://github.com/fra2404/homebrew-core.git"
echo "3. Copy homebrew/qr-forge.rb to Formula/qr-forge.rb in the fork"
echo "4. Test with: brew install --build-from-source ./Formula/qr-forge.rb"
echo "5. Audit with: brew audit --strict qr-forge"
echo "6. Create PR with the new formula"
echo ""
echo "📋 Formula ready at: homebrew/qr-forge.rb"
echo "🔗 SHA256: $SHA256"
echo "🏷️  Version: $LATEST_TAG"
echo ""
echo "💡 Alternative: Create personal tap for immediate availability:"
echo "   brew install fra2404/qr-forge/qr-forge"

# Cleanup
rm -f "$TEMP_FILE"

echo "🎉 Homebrew preparation complete!"
sed -i '' "s/PLACEHOLDER_SHA256/$SHA256/g" homebrew/qr-forge.rb
sed -i '' "s/v1.0.4/$LATEST_TAG/g" homebrew/qr-forge.rb

echo "✅ Homebrew formula ready!"
echo ""
echo "Next steps:"
echo "1. Create a tap repository: https://github.com/fra2404/homebrew-qr-forge"
echo "2. Copy homebrew/qr-forge.rb to the tap repository"
echo "3. Users can install with: brew install fra2404/qr-forge/qr-forge"
echo ""
echo "Or submit to homebrew-core for global availability!"

# Cleanup
rm "qr-forge-${LATEST_TAG#v}.tar.gz"
