#!/bin/bash

# Generate Flutter/Dart bindings from Rust FFI
# Requires: flutter_rust_bridge_codegen, Flutter SDK

set -e

echo "🔗 Generating Flutter bindings..."

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check if flutter_rust_bridge_codegen is installed
if ! command -v flutter_rust_bridge_codegen &> /dev/null; then
    echo -e "${RED}❌ flutter_rust_bridge_codegen not found${NC}"
    echo "Install it with: cargo install flutter_rust_bridge_codegen"
    exit 1
fi

# Check if Flutter is installed
if ! command -v flutter &> /dev/null; then
    echo -e "${YELLOW}⚠️  Flutter SDK not found. Bindings will be generated but not formatted.${NC}"
    FLUTTER_AVAILABLE=false
else
    FLUTTER_AVAILABLE=true
fi

# Project root
PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
CORE_DIR="$PROJECT_ROOT/core"
APP_DIR="$PROJECT_ROOT/app"

# Input and output paths
RUST_INPUT="$CORE_DIR/src/api.rs"
DART_OUTPUT="$APP_DIR/lib/bridge/generated"

echo -e "${YELLOW}📝 Generating bindings...${NC}"
echo "Rust input: $RUST_INPUT"
echo "Dart output: $DART_OUTPUT"

# Create output directory
mkdir -p "$DART_OUTPUT"

# Generate bindings
flutter_rust_bridge_codegen \
    --rust-input "$RUST_INPUT" \
    --dart-output "$DART_OUTPUT" \
    --dart-decl-output "$DART_OUTPUT" \
    --c-output "$APP_DIR/ios/Runner/bridge_generated.h"

if [ $? -eq 0 ]; then
    echo -e "${GREEN}✅ Bindings generated successfully${NC}"
else
    echo -e "${RED}❌ Failed to generate bindings${NC}"
    exit 1
fi

# Format Dart code if Flutter is available
if [ "$FLUTTER_AVAILABLE" = true ]; then
    echo -e "${YELLOW}📝 Formatting Dart code...${NC}"
    cd "$APP_DIR"
    flutter format lib/bridge/
    echo -e "${GREEN}✅ Dart code formatted${NC}"
fi

# Show generated files
echo -e "\n${YELLOW}📊 Generated files:${NC}"
find "$DART_OUTPUT" -type f -name "*.dart" -exec ls -lh {} \; | awk '{print $9, $5}'

echo -e "\n${GREEN}🎉 Bindings generation complete!${NC}"
echo -e "Generated files in: $DART_OUTPUT"
