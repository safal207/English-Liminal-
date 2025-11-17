# Integration Tests

This directory contains integration tests for English-Liminal core.

## Test Suites

### `role_lifecycle.rs` - Role Lifecycle Tests
Tests the complete user journey through a role:
- Starting a role and completing scenes
- Emotion tagging
- Coherence calculation
- Social resonance with reflections
- Multiple roles isolation

**Run:** These tests can run in parallel.
```bash
cargo test --test role_lifecycle
```

### `ffi_integration.rs` - FFI API Tests
Tests the Flutter/FFI API layer:
- Health checks and initialization
- Role progress flow via JSON API
- Resonance traces and reflections
- Statistics and events
- JSON serialization

**⚠️ IMPORTANT:** These tests **MUST** be run sequentially due to global storage state.

## Running Tests

### Sequential (Required for FFI tests)
```bash
# Run all integration tests sequentially
cargo test --tests -- --test-threads=1

# Run only FFI tests
cargo test --test ffi_integration -- --test-threads=1
```

### Using Makefile (Recommended)
```bash
# Run all integration tests (handles threading automatically)
make test-integration

# Run unit + integration tests
make test-all
```

## Why Sequential Execution?

The FFI API uses global state (`static STORE`) which is shared across all tests. When tests run in parallel, they:
1. Overwrite each other's storage
2. Cause race conditions
3. Lead to assertion failures

**Example of failure:**
```
cargo test --test ffi_integration  # ❌ Will fail (parallel by default)
cargo test --test ffi_integration -- --test-threads=1  # ✅ Will pass
```

## Test Statistics

```
role_lifecycle.rs:     5 tests
ffi_integration.rs:   11 tests
Total:                16 integration tests
```

All tests run in < 100ms when sequential.

## CI/CD

In GitHub Actions (`.github/workflows/ci.yml`), integration tests run with:
```yaml
- name: Run integration tests
  run: cd core && cargo test --tests -- --test-threads=1 --verbose
```

## Future Improvements

To allow parallel execution of FFI tests, we would need to:
1. Refactor FFI API to not use global state, OR
2. Use unique storage paths per test with proper cleanup, OR
3. Implement a test-only mutex/lock mechanism

For now, sequential execution is the simplest and most reliable approach.
