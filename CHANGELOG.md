# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.0.0] - 2025-06-30

### Added
- Initial release of QR Forge
- CLI QR code generator with multiple output formats (PNG, SVG, JPEG)
- GUI application with real-time preview using egui/eframe
- Support for custom colors (foreground and background)
- Configurable output sizes and quality settings
- macOS app bundle with native .app format
- Cross-platform support (Linux, macOS, Windows)
- Comprehensive error handling and input validation
- English localization (converted from Italian)

### Features
- **CLI Interface**: Command-line tool for batch processing and automation
- **GUI Interface**: User-friendly graphical interface with live preview
- **Multiple Formats**: PNG, SVG, and JPEG output support
- **Customization**: Custom colors, sizes, and quality settings
- **Cross-Platform**: Native builds for Linux, macOS, and Windows
- **macOS Integration**: Proper .app bundle with icon and metadata

### Technical
- Built with Rust for performance and safety
- Uses qrcode crate for QR generation
- egui/eframe for cross-platform GUI
- Native file dialogs with rfd crate
- Automated builds with GitHub Actions
- Comprehensive test coverage
