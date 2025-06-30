# 🍺 QR Forge Personal Homebrew Tap

This repository contains the Homebrew formula for QR Forge, enabling easy installation via Homebrew package manager.

## 🚀 Quick Installation

```bash
# Add the QR Forge tap
brew tap fra2404/qr-forge

# Install QR Forge
brew install qr-forge
```

## 📦 What Gets Installed

- **qr-forge**: CLI interface for QR code generation
- **qr-forge-gui**: GUI application for interactive QR code creation
- All dependencies automatically resolved

## 🎯 Features

QR Forge provides professional QR code generation with:

- **Multiple formats**: SVG, PNG, JPG, BMP
- **Customizable styling**: Colors, sizes, margins
- **High error correction**: Levels L, M, Q, H
- **Dual interfaces**: Both CLI and GUI
- **Cross-platform**: macOS and Linux support

## 💻 Usage Examples

### CLI Interface
```bash
# Basic QR code generation
qr-forge --url "https://github.com/fra2404/qr-forge" --output qr --format png

# Custom styling
qr-forge --url "https://example.com" --output custom --format svg \
  --size 400 --margin 2 --foreground "0000FF" --background "FFFFFF"

# High error correction for print
qr-forge --url "https://business.com" --output business --format png \
  --size 600 --error-correction H
```

### GUI Interface
```bash
# Launch interactive GUI
qr-forge-gui
```

## 🔧 Requirements

- **macOS**: 10.15 (Catalina) or later
- **Linux**: Recent distribution with glibc 2.27+
- **Architecture**: ARM64 (Apple Silicon) or x86_64 (Intel/AMD)

## 🆘 Troubleshooting

### Formula Issues
```bash
# Update the tap
brew update

# Reinstall if needed
brew uninstall qr-forge
brew install qr-forge

# Check formula
brew doctor
```

### macOS Security Warning
If macOS shows a security warning for the GUI app:
1. Right-click on the app → "Open"
2. Click "Open" in the security dialog
3. The app will remember this permission

## 🔄 Updates

The formula automatically tracks new releases of QR Forge. To update:

```bash
brew update
brew upgrade qr-forge
```

## 📋 Support

- 🐛 **Issues**: [GitHub Issues](https://github.com/fra2404/qr-forge/issues)
- 📖 **Documentation**: [Project Website](https://fra2404.github.io/qr-forge/)
- 💡 **Feature Requests**: [GitHub Discussions](https://github.com/fra2404/qr-forge/discussions)

## 📄 License

MIT License - See [LICENSE](https://github.com/fra2404/qr-forge/blob/main/LICENSE) for details.

---

**Made with ❤️ by [Francesco](https://github.com/fra2404)**
