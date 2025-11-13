# Build Scripts

Automation scripts for building English-Liminal for various platforms.

## Prerequisites

### For All Platforms
- Rust toolchain (rustc, cargo)
- `cargo install flutter_rust_bridge_codegen`

### For Android
- Android NDK (set `ANDROID_NDK_HOME` or `NDK_HOME`)
- `cargo install cargo-ndk`
- Rust Android targets:
  ```bash
  rustup target add armv7-linux-androideabi
  rustup target add aarch64-linux-android
  rustup target add i686-linux-android
  rustup target add x86_64-linux-android
  ```

### For iOS
- macOS with Xcode
- Rust iOS targets:
  ```bash
  rustup target add aarch64-apple-ios
  rustup target add x86_64-apple-ios
  rustup target add aarch64-apple-ios-sim
  ```

## Scripts

### `build.sh` - Main Build Script

The main entry point for all builds. Supports multiple platforms and options.

**Usage:**
```bash
./scripts/build.sh [OPTIONS]

Options:
  --android     Build for Android
  --ios         Build for iOS
  --bindings    Generate Flutter bindings
  --all         Build everything
  --help        Show help
```

**Examples:**
```bash
# Build for Android only
./scripts/build.sh --android

# Generate bindings and build Android
./scripts/build.sh --bindings --android

# Build everything
./scripts/build.sh --all

# Just build Rust core (no arguments)
./scripts/build.sh
```

### `build-android.sh` - Android Build

Builds Rust library for all Android ABIs (armeabi-v7a, arm64-v8a, x86, x86_64).

**Usage:**
```bash
./scripts/build-android.sh
```

**Output:**
- `app/android/app/src/main/jniLibs/<abi>/libliminal_english_core.so`

### `build-ios.sh` - iOS Build

Builds Rust library for iOS devices and simulator, creates XCFramework.

**Usage:**
```bash
./scripts/build-ios.sh
```

**Output:**
- `app/ios/Runner/liminal_english_core.xcframework/`

**Note:** Requires macOS with Xcode installed.

### `generate-bindings.sh` - Flutter Bindings Generator

Generates Dart/Flutter bindings from Rust FFI code.

**Usage:**
```bash
./scripts/generate-bindings.sh
```

**Output:**
- `app/lib/bridge/generated/*.dart`
- `app/ios/Runner/bridge_generated.h`

## Quick Start

1. **Install prerequisites:**
   ```bash
   # Install flutter_rust_bridge_codegen
   cargo install flutter_rust_bridge_codegen

   # For Android (if needed)
   cargo install cargo-ndk
   ```

2. **Generate bindings:**
   ```bash
   ./scripts/generate-bindings.sh
   ```

3. **Build for your platform:**
   ```bash
   # Android
   ./scripts/build-android.sh

   # iOS (macOS only)
   ./scripts/build-ios.sh

   # Or build everything
   ./scripts/build.sh --all
   ```

## Integration with Makefile

These scripts can also be called via Makefile:

```bash
# From project root
make build-android    # Build for Android
make build-ios        # Build for iOS
make build-all        # Build everything
```

## Troubleshooting

### Android NDK not found
```bash
export ANDROID_NDK_HOME=/path/to/ndk
# or
export NDK_HOME=/path/to/ndk
```

### cargo-ndk not found
```bash
cargo install cargo-ndk
```

### flutter_rust_bridge_codegen not found
```bash
cargo install flutter_rust_bridge_codegen
```

### Rust targets not installed
```bash
# Android
rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android

# iOS (macOS only)
rustup target add aarch64-apple-ios x86_64-apple-ios aarch64-apple-ios-sim
```

## CI/CD

These scripts are designed to work in CI environments. See `.github/workflows/` for GitHub Actions configuration.

## License

Part of the English-Liminal project.
