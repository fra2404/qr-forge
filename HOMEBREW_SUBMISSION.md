# QR Forge - Homebrew Core Submission Guide

This guide outlines the steps to submit QR Forge to Homebrew Core for global availability.

## Prerequisites

1. **Stable Release**: Ensure we have a stable tagged release
2. **Formula Tested**: Test the Homebrew formula locally
3. **Documentation**: Complete README and proper licensing

## Steps to Submit to Homebrew Core

### 1. Fork homebrew-core

```bash
# Go to GitHub and fork: https://github.com/Homebrew/homebrew-core
# Then clone your fork
git clone https://github.com/fra2404/homebrew-core.git
cd homebrew-core
```

### 2. Create Release with Proper SHA256

First, we need to create a proper release and get the SHA256:

```bash
# Create a new release tag
git tag v1.0.4
git push origin v1.0.4

# Download the release tarball and calculate SHA256
curl -L https://github.com/fra2404/qr-forge/archive/refs/tags/v1.0.4.tar.gz -o qr-forge-1.0.4.tar.gz
shasum -a 256 qr-forge-1.0.4.tar.gz
```

### 3. Update Formula with Correct SHA256

Update the `sha256` field in `homebrew/qr-forge.rb` with the calculated value.

### 4. Add Formula to homebrew-core

```bash
# Copy the formula to the correct location
cp /path/to/qr-forge/homebrew/qr-forge.rb Formula/qr-forge.rb
```

### 5. Test the Formula Locally

```bash
# Install dependencies
brew install --build-from-source ./Formula/qr-forge.rb

# Test the formula
brew test qr-forge

# Audit the formula
brew audit --strict qr-forge
```

### 6. Create Pull Request

```bash
# Create a new branch
git checkout -b qr-forge

# Add and commit the formula
git add Formula/qr-forge.rb
git commit -m "qr-forge: new formula

Professional QR code generator built with Rust.
Supports multiple output formats (SVG, PNG, JPG, BMP)
with customizable colors, sizes, and error correction levels."

# Push and create PR
git push origin qr-forge
```

### 7. Submit Pull Request

Go to GitHub and create a Pull Request with:

**Title**: `qr-forge: new formula`

**Description**:

```
## qr-forge

Professional QR code generator built with Rust.

### Features
- Multiple output formats: SVG, PNG, JPG, BMP
- Customizable colors, sizes, margins, error correction
- Both CLI and GUI interfaces available
- High-performance Rust implementation
- Professional quality for printing

### Testing
- [x] `brew install --build-from-source ./Formula/qr-forge.rb` works
- [x] `brew test qr-forge` passes
- [x] `brew audit --strict qr-forge` passes

### Links
- Homepage: https://github.com/fra2404/qr-forge
- License: MIT
```

## Homebrew Core Requirements Checklist

- [x] Open source with compatible license (MIT)
- [x] Stable, tagged releases
- [x] No GUI dependencies for CLI tool
- [x] Proper test coverage
- [x] Clean, auditable formula
- [x] Notable/useful software
- [x] Maintained project

## Test Results

### ARM64 (Apple Silicon) - ✅ PASSED

Tested on macOS with Apple Silicon M-series processor:

- **Architecture**: arm64
- **Formula syntax**: ✅ Valid
- **Installation**: ✅ Success via `brew install --build-from-source`
- **Binary functionality**: ✅ All features working
- **Format support**: ✅ PNG, SVG generation working
- **Custom parameters**: ✅ Size, margin, colors working
- **Performance**: ✅ ~0.2s per QR code generation
- **Homebrew test**: ✅ All formula tests pass
- **Memory usage**: ⚠️ Valgrind not available on macOS (normal)

### AMD64 (Intel) - ⏳ PENDING

Testing on Intel-based Mac or Linux system needed:

To test on AMD64 architecture:

```bash
# Run the comprehensive test script
./test_homebrew.sh
```

Expected results should mirror ARM64 performance with similar timing.

### Cross-Architecture Compatibility

The formula uses `cargo install` which automatically compiles for the target architecture, ensuring compatibility across:

- ✅ ARM64 (Apple Silicon M1/M2/M3)
- ⏳ AMD64 (Intel x86_64) - pending validation
- 🔄 Linux (via Rust cross-compilation)

## Alternative: Personal Tap

If Homebrew Core submission takes time, we can create a personal tap:

```bash
# Create homebrew-qr-forge repository
# Users can then install with:
brew install fra2404/qr-forge/qr-forge
```

## Notes

- Homebrew Core review process can take several weeks
- Maintainers may request changes to the formula
- Personal tap provides immediate availability
- Both approaches are valid distribution methods
