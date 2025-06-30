#!/bin/bash

# QR Forge Development Setup Script
# This script sets up the development environment for QR Forge

set -e

echo "🔥 Setting up QR Forge development environment..."

# Check if Rust is installed
if ! command -v rustc &> /dev/null; then
    echo "❌ Rust is not installed. Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    source $HOME/.cargo/env
else
    echo "✅ Rust is already installed"
fi

# Check Rust version
RUST_VERSION=$(rustc --version | awk '{print $2}')
echo "📦 Rust version: $RUST_VERSION"

# Install platform-specific dependencies
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    echo "🐧 Linux detected - installing GTK development packages..."
    if command -v apt-get &> /dev/null; then
        sudo apt-get update
        sudo apt-get install -y libgtk-3-dev
    elif command -v dnf &> /dev/null; then
        sudo dnf install gtk3-devel
    elif command -v pacman &> /dev/null; then
        sudo pacman -S gtk3
    else
        echo "⚠️  Please install GTK3 development packages manually"
    fi
elif [[ "$OSTYPE" == "darwin"* ]]; then
    echo "🍎 macOS detected - no additional dependencies needed"
else
    echo "🪟 Windows or other OS detected"
fi

# Update Rust toolchain
echo "🔄 Updating Rust toolchain..."
rustup update

# Install useful tools
echo "🛠️  Installing development tools..."
rustup component add clippy rustfmt

# Build the project
echo "🔨 Building QR Forge..."
cargo build

# Run tests
echo "🧪 Running tests..."
cargo test

# Check formatting and linting
echo "✨ Checking code style..."
cargo fmt --check || (echo "⚠️  Code needs formatting. Run 'cargo fmt' to fix." && exit 1)
cargo clippy -- -D warnings

echo ""
echo "🎉 Development environment setup complete!"
echo ""
echo "Available commands:"
echo "  cargo run --bin qr-forge          # Run CLI version"
echo "  cargo run --bin qr-forge-gui      # Run GUI version"
echo "  cargo test                        # Run tests"
echo "  cargo fmt                         # Format code"
echo "  cargo clippy                      # Lint code"
echo "  ./build_app.sh                    # Build macOS app (macOS only)"
echo ""
echo "Happy coding! 🦀"
