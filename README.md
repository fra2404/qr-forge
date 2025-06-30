# 🔥 QR Forge

> **Professional QR code generator built with Rust** - High-quality output, multiple formats, and dual interfaces

[![CI](https://github.com/fra2404/qr-forge/actions/workflows/ci.yml/badge.svg)](https://github.com/fra2404/qr-forge/actions/workflows/ci.yml)
[![Release](https://github.com/fra2404/qr-forge/actions/workflows/release.yml/badge.svg)](https://github.com/fra2404/qr-forge/actions/workflows/release.yml)
[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![GitHub release](https://img.shields.io/github/release/fra2404/qr-forge.svg)](https://github.com/fra2404/qr-forge/releases/latest)
[![Downloads](https://img.shields.io/github/downloads/fra2404/qr-forge/total.svg)](https://github.com/fra2404/qr-forge/releases)

**QR Forge** is a powerful QR code generator written in Rust that produces **highest quality** QR codes with support for scalable SVG formats and high-resolution bitmaps.

## ✨ Features

- 🎨 **Multiple formats**: SVG (scalable), PNG, JPG, BMP
- 🔧 **Highly customizable**: colors, sizes, margins, error correction
- 📱 **Print optimized**: support for professional printing
- ⚡ **Fast and efficient**: written in Rust for maximum performance
- 🎯 **Professional quality**: error correction up to level H
- 📐 **Scalable SVG**: infinite quality without loss of detail
- 🖥️ **Dual interface**: Both CLI and GUI modes available
- 🍎 **Native macOS app**: Complete .app bundle with icon and integration

## 🚀 Quick Start

### Download Pre-built Binaries

Get the latest release from [GitHub Releases](https://github.com/fra2404/qr-forge/releases/latest):

- **macOS**: Download `QR-Forge-GUI-macos.dmg` for the GUI app, or `qr-forge-cli-macos.tar.gz` for CLI
- **Linux**: Download `qr-forge-gui-linux.tar.gz` or `qr-forge-cli-linux.tar.gz`
- **Windows**: Download `qr-forge-gui-windows.zip` or `qr-forge-cli-windows.zip`

#### 🍎 macOS Installation Note

If macOS says the app is "damaged" or "cannot be opened":

1. **Right-click** on QR Forge.app and select **"Open"**
2. Click **"Open"** in the security dialog
3. Alternatively, run in Terminal: `sudo xattr -c "/Applications/QR Forge.app"`

This happens because the app isn't signed with an Apple Developer certificate ($99/year).

### Build from Source

```bash
git clone https://github.com/fra2404/qr-forge.git
cd qr-forge
cargo build --release
```

## 📖 Usage

### GUI Mode (Recommended)

**macOS**: Double-click the QR Forge app or run:

```bash
qr-forge-gui
```

**Other platforms**: Run the GUI executable:

```bash
./qr-forge-gui
```

The GUI provides an intuitive interface with:

- 🎯 Real-time QR code preview
- 🎨 Visual color picker for SVG formats
- 📁 Easy file saving with system dialogs
- ⚙️ Interactive parameter adjustment
- 📊 Instant feedback and status messages

### CLI Mode (Advanced users)

### Basic syntax

```bash
qr-forge --url "https://example.com"
```

### Examples

**Scalable SVG QR code:**

```bash
qr-forge --url "https://github.com" --format svg --output "github_qr"
```

**QR code with custom colors:**

```bash
qr-forge --url "https://rust-lang.org" --format svg --color "ff6600" --background-color "f5f5dc"
```

**High-resolution QR code for printing:**

```bash
qr-forge --url "https://example.com" --format png --size 2400 --margin 4
```

**Print-optimized QR code (no border):**

```bash
qr-forge --url "https://example.com" --format svg --margin 0 --size 1200
```

### Available parameters

| Parameter            | Description                         | Default  | Example                    |
| -------------------- | ----------------------------------- | -------- | -------------------------- |
| `--url`              | URL to encode (required)            | -        | `https://example.com`      |
| `--output`           | Output filename (without extension) | `qrcode` | `my_qr`                    |
| `--size`             | Size in pixels                      | `800`    | `1200`                     |
| `--format`           | Output format                       | `png`    | `svg`, `png`, `jpg`, `bmp` |
| `--margin`           | Margin in modules                   | `4`      | `0`, `2`, `8`              |
| `--error-correction` | Error correction level              | `H`      | `L`, `M`, `Q`, `H`         |
| `--color`            | QR color (hex, SVG only)            | `000000` | `ff0000`                   |
| `--background-color` | Background color (hex, SVG only)    | `ffffff` | `f0f8ff`                   |

## 🎯 Use Cases

### 📱 **For digital use**

```bash
qr-forge --url "https://mywebsite.com" --format svg --size 500
```

### 🖨️ **For professional printing**

```bash
qr-forge --url "https://mywebsite.com" --format png --size 2400 --margin 2
```

### 🎨 **For custom design**

```bash
qr-forge --url "https://mywebsite.com" --format svg --color "2c3e50" --background-color "ecf0f1"
```

### 📋 **For business cards**

```bash
qr-forge --url "https://linkedin.com/in/myprofile" --format svg --margin 1 --size 800
```

## 📊 Supported Formats

| Format  | Description         | Ideal Use Case                    |
| ------- | ------------------- | --------------------------------- |
| **SVG** | Scalable vector     | Web, professional printing, logos |
| **PNG** | High-quality bitmap | Printing, presentations           |
| **JPG** | Compressed bitmap   | Web, sharing                      |
| **BMP** | Uncompressed bitmap | Image processing                  |

## 🔧 Error Correction Levels

| Level | Correction | Capacity | Recommended Use           |
| ----- | ---------- | -------- | ------------------------- |
| **L** | ~7%        | Maximum  | Perfect environments      |
| **M** | ~15%       | High     | General use               |
| **Q** | ~25%       | Medium   | Difficult environments    |
| **H** | ~30%       | Minimum  | **Printing/Professional** |

## 🎨 Output Examples

### Classic QR Code

```bash
qr-forge --url "https://rust-lang.org"
```

- Output: `qrcode.png` (800x800px, black on white)

### QR Code for printing

```bash
qr-forge --url "https://example.com" --format svg --margin 2 --size 1200
```

- Output: `qrcode.svg` (scalable, excellent for printing)

### Colored QR Code

```bash
qr-forge --url "https://github.com" --format svg --color "0066cc" --background-color "f0f8ff"
```

- Output: `qrcode.svg` (blue on light blue)

## 🏗️ Development

### Building

```bash
cargo build
```

### Testing

```bash
cargo test
```

### Optimized build

```bash
cargo build --release
```

## 🤝 Contributing

Contributions are welcome! Please:

1. Fork the project
2. Create a feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## 📝 License

This project is released under the MIT License. See the `LICENSE` file for details.

## 🙏 Acknowledgments

- [qrcode-rust](https://github.com/kennytm/qrcode-rust) - Core library for QR generation
- [image-rs](https://github.com/image-rs/image) - Image processing
- [clap](https://github.com/clap-rs/clap) - CLI argument parsing
- [svg](https://github.com/bodoni/svg) - SVG generation

---

**Made with ❤️ in Rust**
