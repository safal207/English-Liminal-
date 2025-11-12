# GitHub Actions Workflows

This directory contains CI/CD workflows for English-Liminal.

## Workflows

### `ci.yml` - Continuous Integration

**Triggers:** Push to main/develop/claude branches, Pull Requests

**Jobs:**
- **test**: Run all unit and integration tests (51 tests)
- **fmt**: Check Rust code formatting
- **clippy**: Run Rust linter
- **validate**: Validate YAML content using CLI
- **build**: Build release binary
- **coverage**: Generate code coverage report

**Badge:**
```markdown
![CI](https://github.com/safal207/English-Liminal-/workflows/CI/badge.svg)
```

### `release.yml` - Release Builds

**Triggers:** Push tags matching `v*.*.*` (e.g., v1.1.0)

**Jobs:**
- **create-release**: Create GitHub release
- **build-linux**: Build for Linux x86_64
- **build-macos**: Build for macOS x86_64
- **build-windows**: Build for Windows x86_64

**Usage:**
```bash
# Create and push a release tag
git tag v1.1.0
git push origin v1.1.0

# GitHub Actions will automatically:
# 1. Create a GitHub release
# 2. Build binaries for all platforms
# 3. Upload artifacts to the release
```

### `mobile.yml` - Mobile Builds

**Triggers:**
- Manual workflow dispatch (with platform selection)
- Push tags matching `mobile-v*.*.*`

**Jobs:**
- **build-android**: Build for all Android ABIs
- **build-ios**: Build iOS XCFramework
- **generate-bindings**: Generate Flutter/Dart bindings

**Manual Trigger:**
1. Go to Actions tab on GitHub
2. Select "Mobile Build" workflow
3. Click "Run workflow"
4. Choose platform: android, ios, or all

**Tag Trigger:**
```bash
git tag mobile-v1.1.0
git push origin mobile-v1.1.0
```

## Caching Strategy

All workflows use GitHub Actions cache for:
- Cargo registry (`~/.cargo/registry`)
- Cargo git index (`~/.cargo/git`)
- Build artifacts (`core/target`)

This significantly speeds up builds (2-5x faster).

## Required Secrets

No secrets required for public repositories. For private repos:
- `GITHUB_TOKEN`: Automatically provided by GitHub Actions

## Optional Secrets

For code coverage (codecov):
- `CODECOV_TOKEN`: Token from codecov.io

## Artifacts

**CI Workflow:**
- `liminal-cli-linux`: Linux CLI binary (7 days retention)

**Release Workflow:**
- `liminal-cli-linux-x86_64.tar.gz`: Linux binary
- `liminal-cli-macos-x86_64.tar.gz`: macOS binary
- `liminal-cli-windows-x86_64.zip`: Windows binary

**Mobile Workflow:**
- `android-libs`: Android .so files for all ABIs
- `ios-xcframework`: iOS XCFramework
- `flutter-bindings`: Generated Dart bindings

## Status Checks

For Pull Requests, the following checks must pass:
- ✅ All tests (unit + integration)
- ✅ Rustfmt (code formatting)
- ✅ Clippy (linting)
- ✅ Content validation

## Local Testing

Test CI locally before pushing:

```bash
# Run what CI runs
make test-all      # All tests
make fmt           # Check formatting
make lint          # Run clippy
make validate      # Validate content
make build         # Build release
```

## Troubleshooting

### Tests fail in CI but pass locally
- Ensure tests run with `--test-threads=1` for integration tests
- Check for race conditions in global state (storage)

### Android build fails
- Verify NDK version matches (r25c)
- Check all Android targets are installed

### iOS build fails
- macOS runner required
- Xcode must be available
- Check iOS targets are installed

### Cache issues
Clear cache manually:
1. Go to Actions tab
2. Click "Caches" in sidebar
3. Delete relevant caches

## Performance

**Typical CI run times:**
- Without cache: ~8-12 minutes
- With cache: ~2-4 minutes

**Mobile builds:**
- Android: ~15-20 minutes
- iOS: ~12-18 minutes
- Bindings: ~3-5 minutes

## Future Improvements

- [ ] Add Docker builds
- [ ] Add Flutter app builds
- [ ] Add E2E tests for mobile
- [ ] Add deployment workflows
- [ ] Add nightly builds
- [ ] Add benchmark tracking
