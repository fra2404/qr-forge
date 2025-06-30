# Contributing to QR Forge

Thank you for your interest in contributing to QR Forge! This document provides guidelines for contributing to the project.

## Development Setup

1. **Install Rust**

   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Clone the repository**

   ```bash
   git clone https://github.com/fra2404/qr-forge.git
   cd qr-forge
   ```

3. **Install dependencies**

   **Linux:**

   ```bash
   sudo apt-get install libgtk-3-dev
   ```

   **macOS:**

   ```bash
   # No additional dependencies needed
   ```

4. **Build the project**
   ```bash
   cargo build
   ```

## Building

- **CLI only**: `cargo build --bin qr-forge`
- **GUI only**: `cargo build --bin qr-forge-gui`
- **Both**: `cargo build`
- **Release**: `cargo build --release`

## Testing

```bash
cargo test
cargo clippy
cargo fmt --check
```

## Code Style

- Follow Rust standard formatting (`cargo fmt`)
- Address all clippy warnings (`cargo clippy`)
- Write tests for new functionality
- Update documentation as needed

## Submitting Changes

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## Release Process

Releases are automatically created when a tag starting with `v` is pushed:

```bash
git tag v1.0.1
git push origin v1.0.1
```

This will trigger the GitHub Actions workflow to build and publish releases for all platforms.

## Project Structure

```
qr-forge/
├── src/
│   ├── main.rs          # CLI entry point
│   ├── gui_main.rs      # GUI entry point
│   └── gui_core.rs      # GUI implementation
├── .github/workflows/   # CI/CD pipelines
├── build_app.sh         # macOS app builder
├── create_icon.sh       # Icon generator
└── QR Forge.app/        # macOS app bundle
```

## Questions?

Open an issue for questions, bug reports, or feature requests.
