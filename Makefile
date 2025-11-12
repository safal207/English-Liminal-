.PHONY: help build test test-integration test-all validate clean install cli db-inspect health build-android build-ios build-all gen-bindings

# Default target
help:
	@echo "English-Liminal Development Makefile"
	@echo ""
	@echo "Targets:"
	@echo "  help        - Show this help message"
	@echo "  build       - Build Rust core library"
	@echo "  test        - Run all tests"
	@echo "  validate    - Validate all YAML content"
	@echo "  cli         - Build CLI tool"
	@echo "  install     - Install CLI tool to system"
	@echo "  clean       - Clean build artifacts"
	@echo "  health      - Run health check"
	@echo "  db-inspect  - Inspect database (requires DB_PATH)"
	@echo ""
	@echo "Examples:"
	@echo "  make build"
	@echo "  make test"
	@echo "  make validate"
	@echo "  make db-inspect DB_PATH=liminal.db"

# Build Rust core
build:
	@echo "🔨 Building Rust core..."
	cd core && cargo build --release
	@echo "✅ Build complete"

# Run all unit tests
test:
	@echo "🧪 Running unit tests..."
	cd core && cargo test --lib
	@echo "✅ Unit tests complete"

# Run integration tests (sequentially to avoid race conditions)
test-integration:
	@echo "🧪 Running integration tests..."
	cd core && cargo test --tests -- --test-threads=1
	@echo "✅ Integration tests complete"

# Run all tests (unit + integration)
test-all: test test-integration
	@echo "✅ All tests complete"

# Build CLI tool
cli:
	@echo "🔧 Building CLI tool..."
	cd core && cargo build --bin liminal_cli --release
	@echo "✅ CLI built: core/target/release/liminal_cli"

# Install CLI tool to system
install: cli
	@echo "📦 Installing liminal_cli..."
	cp core/target/release/liminal_cli /usr/local/bin/ 2>/dev/null || \
		cp core/target/release/liminal_cli ~/bin/ || \
		echo "⚠️  Could not install to /usr/local/bin or ~/bin. Run manually from core/target/release/liminal_cli"
	@echo "✅ Installation complete"

# Validate all YAML content
validate: cli
	@echo "🔍 Validating content..."
	./core/target/release/liminal_cli validate

# Health check
health: cli
	@echo "🏥 Running health check..."
	./core/target/release/liminal_cli health

# Inspect database
db-inspect: cli
ifndef DB_PATH
	@echo "❌ Error: DB_PATH not specified"
	@echo "Usage: make db-inspect DB_PATH=path/to/liminal.db"
	@exit 1
endif
	@echo "🗄️  Inspecting database..."
	./core/target/release/liminal_cli db inspect $(DB_PATH)

# Clean build artifacts
clean:
	@echo "🧹 Cleaning build artifacts..."
	cd core && cargo clean
	@echo "✅ Clean complete"

# Quick development cycle: build + test + validate
dev: build test validate
	@echo "✅ Development cycle complete"

# Full CI pipeline
ci: clean build test validate
	@echo "✅ CI pipeline complete"

# Check code formatting
fmt:
	@echo "📝 Checking code formatting..."
	cd core && cargo fmt --check

# Apply code formatting
fmt-fix:
	@echo "📝 Applying code formatting..."
	cd core && cargo fmt

# Run clippy lints
lint:
	@echo "🔍 Running clippy..."
	cd core && cargo clippy -- -D warnings

# Documentation generation
docs:
	@echo "📚 Generating documentation..."
	cd core && cargo doc --no-deps
	@echo "✅ Docs generated: core/target/doc/liminal_english_core/index.html"

# Quick test cycle (no rebuild)
test-quick:
	@echo "🧪 Running quick tests..."
	cd core && cargo test --lib --no-fail-fast

# Benchmark (if we add benches later)
bench:
	@echo "⚡ Running benchmarks..."
	cd core && cargo bench

# Show project statistics
stats:
	@echo "📊 Project Statistics"
	@echo ""
	@echo "Rust Code:"
	@find core/src -name "*.rs" | xargs wc -l | tail -1
	@echo ""
	@echo "YAML Scenarios:"
	@find assets/scripts content/roles -name "*.yaml" | wc -l
	@echo ""
	@echo "Tests:"
	@grep -r "#\[test\]" core/src | wc -l

# ============================================================================
# Mobile Build Commands
# ============================================================================

# Generate Flutter/Dart bindings
gen-bindings:
	@echo "🔗 Generating Flutter bindings..."
	./scripts/generate-bindings.sh

# Build for Android
build-android:
	@echo "🤖 Building for Android..."
	./scripts/build-android.sh

# Build for iOS
build-ios:
	@echo "🍎 Building for iOS..."
	./scripts/build-ios.sh

# Build everything (bindings + Android + iOS)
build-all:
	@echo "🚀 Building for all platforms..."
	./scripts/build.sh --all

# Quick mobile dev cycle: bindings + Android
mobile-dev: gen-bindings build-android
	@echo "✅ Mobile dev build complete"
