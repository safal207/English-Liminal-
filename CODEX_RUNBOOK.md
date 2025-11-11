# CODEX RUNBOOK — Гайд для AI-агентов (Rust + Flutter)

Пошаговая инструкция для разработки English-Liminal MVP на **Rust core + Flutter UI**.

---

## Общая информация

**Проект:** English-Liminal
**Цель:** 30-дневный MVP приложения для изучения английского
**Стек:**
- **Rust** (бизнес-логика, retention-wave, хранилище)
- **Flutter** (кроссплатформенный UI)
- **flutter_rust_bridge** (FFI связь между Rust и Dart)

**Управление задачами:** См. [ISSUES.md](./ISSUES.md)

---

## Быстрый старт для агента

### 1. Установка инструментов

```bash
# Rust toolchain
rustup update
rustup target add aarch64-apple-ios x86_64-apple-ios aarch64-linux-android

# Flutter
flutter doctor  # Проверить установку

# FFI codegen
cargo install flutter_rust_bridge_codegen cargo-ndk
```

### 2. Генерация FFI биндингов

```bash
flutter_rust_bridge_codegen \
  --rust-input core/src/api.rs \
  --dart-output app/lib/bridge/bridge.generated.dart \
  --dart-decl-output app/lib/bridge/bridge_definitions.dart
```

### 3. Сборка и запуск

**Android:**
```bash
cd core
cargo ndk -t arm64-v8a -o ../app/android/app/src/main/jniLibs build --release
cd ../app && flutter run
```

**iOS:**
```bash
# Настроить Xcode build script (см. ниже)
cd app && flutter run
```

---

## Следующий шаг

Начать с **Issue A2** → настройка flutter_rust_bridge (см. [ISSUES.md](./ISSUES.md))

---

**Удачи в разработке! 🦀🎨**
