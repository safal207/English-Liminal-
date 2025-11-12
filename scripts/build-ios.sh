#!/bin/bash

# Build script for iOS
# Requires: Xcode, Rust iOS targets

set -e

echo "🍎 Building Rust library for iOS..."

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check if on macOS
if [[ "$OSTYPE" != "darwin"* ]]; then
    echo -e "${RED}❌ iOS builds require macOS${NC}"
    exit 1
fi

# Project root
PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
CORE_DIR="$PROJECT_ROOT/core"
OUTPUT_DIR="$PROJECT_ROOT/app/ios/Runner"
FRAMEWORK_NAME="liminal_english_core"

cd "$CORE_DIR"

# iOS targets
TARGETS=(
    "aarch64-apple-ios"           # iOS devices (ARM64)
    "x86_64-apple-ios"            # iOS simulator (Intel)
    "aarch64-apple-ios-sim"       # iOS simulator (Apple Silicon)
)

echo -e "${YELLOW}📱 Building for iOS targets...${NC}"

# Add targets
for TARGET in "${TARGETS[@]}"; do
    echo -e "${GREEN}Adding target $TARGET...${NC}"
    rustup target add "$TARGET" 2>/dev/null || true
done

# Build for each target
for TARGET in "${TARGETS[@]}"; do
    echo -e "${GREEN}Building for $TARGET...${NC}"
    cargo build --target "$TARGET" --release

    if [ $? -eq 0 ]; then
        echo -e "${GREEN}✅ Built $TARGET${NC}"
    else
        echo -e "${RED}❌ Failed to build $TARGET${NC}"
        exit 1
    fi
done

echo -e "${YELLOW}📦 Creating XCFramework...${NC}"

# Create output directory
mkdir -p "$OUTPUT_DIR"

# Create universal library for simulator (Intel + Apple Silicon)
SIMULATOR_LIB="$CORE_DIR/target/universal-sim/release/lib$FRAMEWORK_NAME.a"
mkdir -p "$(dirname "$SIMULATOR_LIB")"

lipo -create \
    "$CORE_DIR/target/x86_64-apple-ios/release/lib$FRAMEWORK_NAME.a" \
    "$CORE_DIR/target/aarch64-apple-ios-sim/release/lib$FRAMEWORK_NAME.a" \
    -output "$SIMULATOR_LIB"

if [ $? -eq 0 ]; then
    echo -e "${GREEN}✅ Created universal simulator library${NC}"
else
    echo -e "${RED}❌ Failed to create universal library${NC}"
    exit 1
fi

# Create XCFramework
XCFRAMEWORK_PATH="$OUTPUT_DIR/$FRAMEWORK_NAME.xcframework"
rm -rf "$XCFRAMEWORK_PATH"

xcodebuild -create-xcframework \
    -library "$CORE_DIR/target/aarch64-apple-ios/release/lib$FRAMEWORK_NAME.a" \
    -library "$SIMULATOR_LIB" \
    -output "$XCFRAMEWORK_PATH"

if [ $? -eq 0 ]; then
    echo -e "${GREEN}✅ Created XCFramework${NC}"
else
    echo -e "${RED}❌ Failed to create XCFramework${NC}"
    exit 1
fi

# Show framework info
echo -e "\n${YELLOW}📊 XCFramework info:${NC}"
xcodebuild -list -xcframework "$XCFRAMEWORK_PATH"

# Show file sizes
echo -e "\n${YELLOW}📊 Library sizes:${NC}"
find "$XCFRAMEWORK_PATH" -name "*.a" -exec ls -lh {} \; | awk '{print $9, $5}'

echo -e "\n${GREEN}🎉 iOS build successful!${NC}"
echo -e "XCFramework saved to: $XCFRAMEWORK_PATH"
