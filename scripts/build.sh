#!/bin/bash

# Main build script for English-Liminal
# Builds Rust core, generates bindings, and builds for all platforms

set -e

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Project root
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

echo -e "${BLUE}╔════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║  English-Liminal Build Script         ║${NC}"
echo -e "${BLUE}╚════════════════════════════════════════╝${NC}\n"

# Parse arguments
BUILD_ANDROID=false
BUILD_IOS=false
GENERATE_BINDINGS=false
BUILD_ALL=false

while [[ $# -gt 0 ]]; do
    case $1 in
        --android)
            BUILD_ANDROID=true
            shift
            ;;
        --ios)
            BUILD_IOS=true
            shift
            ;;
        --bindings)
            GENERATE_BINDINGS=true
            shift
            ;;
        --all)
            BUILD_ALL=true
            BUILD_ANDROID=true
            BUILD_IOS=true
            GENERATE_BINDINGS=true
            shift
            ;;
        --help)
            echo "Usage: $0 [OPTIONS]"
            echo ""
            echo "Options:"
            echo "  --android     Build for Android"
            echo "  --ios         Build for iOS"
            echo "  --bindings    Generate Flutter bindings"
            echo "  --all         Build everything"
            echo "  --help        Show this help"
            echo ""
            echo "Examples:"
            echo "  $0 --android              # Build for Android only"
            echo "  $0 --bindings --android   # Generate bindings and build Android"
            echo "  $0 --all                  # Build everything"
            exit 0
            ;;
        *)
            echo -e "${RED}Unknown option: $1${NC}"
            echo "Use --help for usage information"
            exit 1
            ;;
    esac
done

# If no options provided, show help
if [ "$BUILD_ANDROID" = false ] && [ "$BUILD_IOS" = false ] && [ "$GENERATE_BINDINGS" = false ]; then
    echo -e "${YELLOW}No build targets specified. Use --help for options.${NC}"
    echo -e "${YELLOW}Building Rust core only...${NC}\n"
fi

# Always build Rust core first
echo -e "${YELLOW}🔨 Building Rust core...${NC}"
cd "$PROJECT_ROOT/core"
cargo build --release

if [ $? -eq 0 ]; then
    echo -e "${GREEN}✅ Rust core built successfully${NC}\n"
else
    echo -e "${RED}❌ Rust core build failed${NC}"
    exit 1
fi

# Generate bindings if requested
if [ "$GENERATE_BINDINGS" = true ]; then
    echo -e "${YELLOW}🔗 Generating Flutter bindings...${NC}"
    "$SCRIPT_DIR/generate-bindings.sh"
    echo ""
fi

# Build for Android if requested
if [ "$BUILD_ANDROID" = true ]; then
    echo -e "${YELLOW}🤖 Building for Android...${NC}"
    "$SCRIPT_DIR/build-android.sh"
    echo ""
fi

# Build for iOS if requested
if [ "$BUILD_IOS" = true ]; then
    echo -e "${YELLOW}🍎 Building for iOS...${NC}"
    "$SCRIPT_DIR/build-ios.sh"
    echo ""
fi

echo -e "${BLUE}╔════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║  Build Complete!                       ║${NC}"
echo -e "${BLUE}╚════════════════════════════════════════╝${NC}\n"

echo -e "${GREEN}✅ All requested builds completed successfully!${NC}"

if [ "$BUILD_ALL" = true ]; then
    echo -e "\n${YELLOW}📦 Build artifacts:${NC}"
    echo "  • Rust core: core/target/release/"
    echo "  • Android: app/android/app/src/main/jniLibs/"
    echo "  • iOS: app/ios/Runner/liminal_english_core.xcframework/"
    echo "  • Bindings: app/lib/bridge/generated/"
fi
