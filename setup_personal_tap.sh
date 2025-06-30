#!/bin/bash
# Setup QR Forge Personal Homebrew Tap
# This script creates and maintains the personal tap for QR Forge

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}🍺 QR Forge Personal Homebrew Tap Setup${NC}"
echo "=================================================="

# Configuration
TAP_DIR="/Users/francesco/Desktop/homebrew-qr-forge"
TAP_REPO="fra2404/homebrew-qr-forge"
FORMULA_FILE="qr-forge.rb"

# Get latest version from Cargo.toml
VERSION=$(grep '^version = ' Cargo.toml | sed 's/version = "\(.*\)"/\1/')
echo -e "${BLUE}📦 Current version: ${GREEN}v$VERSION${NC}"

# Check if tap directory exists
if [ ! -d "$TAP_DIR" ]; then
    echo -e "${RED}❌ Tap directory not found: $TAP_DIR${NC}"
    echo -e "${YELLOW}💡 Please run this script from the project root directory${NC}"
    exit 1
fi

cd "$TAP_DIR"

# Check if it's a git repository
if [ ! -d ".git" ]; then
    echo -e "${RED}❌ Not a git repository${NC}"
    exit 1
fi

echo -e "${GREEN}✅ Tap directory found${NC}"

# Check if formula file exists
if [ ! -f "$FORMULA_FILE" ]; then
    echo -e "${RED}❌ Formula file not found: $FORMULA_FILE${NC}"
    exit 1
fi

echo -e "${GREEN}✅ Formula file found${NC}"

# Test formula syntax
echo -e "${BLUE}🔍 Testing formula syntax...${NC}"
if brew style "$FORMULA_FILE"; then
    echo -e "${GREEN}✅ Formula syntax is valid${NC}"
else
    echo -e "${RED}❌ Formula syntax errors found${NC}"
    exit 1
fi

# Instructions for GitHub setup
echo ""
echo -e "${YELLOW}📋 Next Steps:${NC}"
echo "1. Create a new GitHub repository: https://github.com/new"
echo "   - Repository name: homebrew-qr-forge"
echo "   - Description: Personal Homebrew tap for QR Forge"
echo "   - Public repository"
echo ""
echo "2. Add the remote and push:"
echo -e "${BLUE}   git remote add origin https://github.com/$TAP_REPO.git${NC}"
echo -e "${BLUE}   git branch -M main${NC}"
echo -e "${BLUE}   git push -u origin main${NC}"
echo ""
echo "3. Test the tap installation:"
echo -e "${BLUE}   brew tap $TAP_REPO${NC}"
echo -e "${BLUE}   brew install qr-forge${NC}"
echo ""
echo -e "${GREEN}🎉 Personal tap is ready!${NC}"
echo ""
echo -e "${BLUE}📖 Users can install QR Forge with:${NC}"
echo -e "${GREEN}   brew tap $TAP_REPO${NC}"
echo -e "${GREEN}   brew install qr-forge${NC}"
