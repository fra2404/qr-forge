[![QR Forge](https://github.com/fra2404/qr-forge/assets/placeholder/qr-forge-banner.png)](https://github.com/fra2404/qr-forge)

# 🔥 QR Forge

> **Professional QR code generator built with Rust** - High-quality output, multiple formats, and dual interfaces

[![CI](https://github.com/fra2404/qr-forge/actions/workflows/ci.yml/badge.svg)](https://github.com/fra2404/qr-forge/actions/workflows/ci.yml)
[![Release](https://github.com/fra2404/qr-forge/actions/workflows/release.yml/badge.svg)](https://github.com/fra2404/qr-forge/actions/workflows/release.yml)
[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![GitHub release](https://img.shields.io/github/release/fra2404/qr-forge.svg)](https://github.com/fra2404/qr-forge/releases/latest)

## ✨ What makes QR Forge special?

- **🎨 Multiple formats**: SVG (infinite quality), PNG, JPG, BMP
- **🖥️ Dual interface**: Professional CLI + intuitive GUI
- **🍎 Native macOS app**: Complete .app bundle with icon
- **⚡ Blazing fast**: Written in Rust for maximum performance
- **🎯 Print ready**: Professional quality with error correction
- **🔧 Highly customizable**: Colors, sizes, margins, quality levels

## 🚀 Quick Start

### Download & Install

- **macOS**: [Download QR-Forge-GUI-macos.dmg](https://github.com/fra2404/qr-forge/releases/latest)
- **Linux**: [Download qr-forge-linux.tar.gz](https://github.com/fra2404/qr-forge/releases/latest)
- **Windows**: [Download qr-forge-windows.zip](https://github.com/fra2404/qr-forge/releases/latest)

### CLI Usage

```bash
# Generate SVG QR code
qr-forge "Hello World" -o qr.svg

# Generate high-res PNG with custom colors
qr-forge "https://example.com" -o qr.png -s 1000 --foreground "#FF0000" --background "#FFFFFF"
```

### GUI Usage

```bash
# Launch graphical interface
qr-forge-gui
```

## 🏗️ For Developers

```bash
# Clone and build
git clone https://github.com/fra2404/qr-forge.git
cd qr-forge
./setup.sh

# Run
cargo run --bin qr-forge-gui
```

## 📊 Features Comparison

| Feature             | CLI | GUI |
| ------------------- | --- | --- |
| All output formats  | ✅  | ✅  |
| Batch processing    | ✅  | ❌  |
| Real-time preview   | ❌  | ✅  |
| Color picker        | ❌  | ✅  |
| Automation friendly | ✅  | ❌  |
| Beginner friendly   | ❌  | ✅  |

## 🤝 Contributing

We welcome contributions! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## 📄 License

MIT License - see [LICENSE](LICENSE) for details.

---

<div align="center">
  <strong>Made with ❤️ and 🦀 Rust</strong>
</div>
