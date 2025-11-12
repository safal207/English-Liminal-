#!/bin/bash

# Build script for Android
# Requires: cargo-ndk, Android NDK

set -e

echo "🤖 Building Rust library for Android..."

# Colors
GREEN='\033[0.32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check if cargo-ndk is installed
if ! command -v cargo-ndk &> /dev/null; then
    echo -e "${RED}❌ cargo-ndk not found${NC}"
    echo "Install it with: cargo install cargo-ndk"
    exit 1
fi

# Check if NDK is available
if [ -z "$ANDROID_NDK_HOME" ] && [ -z "$NDK_HOME" ]; then
    echo -e "${RED}❌ Android NDK not found${NC}"
    echo "Set ANDROID_NDK_HOME or NDK_HOME environment variable"
    exit 1
fi

# Project root
PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
CORE_DIR="$PROJECT_ROOT/core"
OUTPUT_DIR="$PROJECT_ROOT/app/android/app/src/main/jniLibs"

cd "$CORE_DIR"

# Android ABI targets
TARGETS=(
    "armv7-linux-androideabi"   # 32-bit ARM
    "aarch64-linux-android"     # 64-bit ARM
    "i686-linux-android"        # 32-bit x86
    "x86_64-linux-android"      # 64-bit x86
)

# ABI names for Android
ABI_NAMES=(
    "armeabi-v7a"
    "arm64-v8a"
    "x86"
    "x86_64"
)

echo -e "${YELLOW}📱 Building for Android targets...${NC}"

for i in "${!TARGETS[@]}"; do
    TARGET="${TARGETS[$i]}"
    ABI="${ABI_NAMES[$i]}"

    echo -e "${GREEN}Building for $ABI ($TARGET)...${NC}"

    # Add target if not already added
    rustup target add "$TARGET" 2>/dev/null || true

    # Build with cargo-ndk
    cargo ndk \
        --target "$TARGET" \
        --android-platform 21 \
        --output-dir "$OUTPUT_DIR" \
        build --release

    if [ $? -eq 0 ]; then
        echo -e "${GREEN}✅ Built $ABI${NC}"
    else
        echo -e "${RED}❌ Failed to build $ABI${NC}"
        exit 1
    fi
done

echo -e "${GREEN}✅ All Android builds complete!${NC}"
echo -e "Libraries saved to: $OUTPUT_DIR"

# Show file sizes
echo -e "\n${YELLOW}📊 Library sizes:${NC}"
find "$OUTPUT_DIR" -name "*.so" -exec ls -lh {} \; | awk '{print $9, $5}'

echo -e "\n${GREEN}🎉 Android build successful!${NC}"
