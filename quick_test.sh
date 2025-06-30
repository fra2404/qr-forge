#!/bin/bash
# Quick test script for QR Forge on different architectures
# This script can be run on any system with Homebrew to validate the formula

set -e

echo "🧪 QR Forge Quick Architecture Test"
echo "==================================="

# Detect architecture
ARCH=$(uname -m)
OS=$(uname -s)
echo "🔍 System: $OS $ARCH"

# Quick syntax check
echo "🔍 Testing formula syntax..."
brew ruby -e "load 'homebrew/qr-forge.rb'" && echo "✅ Formula syntax valid"

# Install and test
echo "🔨 Installing QR Forge..."
brew install --build-from-source ./homebrew/qr-forge.rb --quiet

echo "🧪 Quick functionality test..."
qr-forge --url "https://github.com/fra2404/qr-forge" --output "arch_test" --format "png" --size "300"

if [[ -f "arch_test.png" ]]; then
    file_size=$(du -h arch_test.png | cut -f1)
    echo "✅ QR code generated successfully"
    echo "  Architecture: $ARCH"
    echo "  File size: $file_size"
    echo "  Test URL: https://github.com/fra2404/qr-forge"
    rm -f arch_test.png
else
    echo "❌ QR code generation failed"
    exit 1
fi

echo "🧪 Running official Homebrew test..."
brew test qr-forge && echo "✅ All tests passed"

echo ""
echo "🎉 SUCCESS: QR Forge works correctly on $OS $ARCH"
echo ""

# Cleanup
brew uninstall qr-forge --quiet
echo "🧹 Cleanup complete"
