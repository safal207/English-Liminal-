# Contributing to English-Liminal

Thank you for your interest in contributing! 🎉

## Quick Start

1. **Fork the repository**
2. **Clone your fork:**
   ```bash
   git clone https://github.com/YOUR_USERNAME/English-Liminal-.git
   cd English-Liminal-
   ```
3. **Install dependencies:**
   ```bash
   pnpm install
   cd app && pnpm install
   ```
4. **Create a branch:**
   ```bash
   git checkout -b feature/your-feature-name
   ```

## Development Workflow

### Running the app

```bash
# Start Expo dev server
pnpm dev

# Or specific platform
pnpm ios
pnpm android
```

### Code style

We use Prettier and ESLint. Format before committing:

```bash
pnpm format
pnpm lint
```

Husky will run these automatically on commit.

## Project Structure

```
app/
  src/
    screens/      # UI screens
    components/   # Reusable components
    hooks/        # React hooks
    lib/          # Business logic
    data/         # YAML scenarios
```

## Adding a New Scenario

1. Create YAML file in `app/src/data/your-scenario-01.yaml`
2. Follow the schema (see existing examples)
3. Validate with zod schema
4. Test manually in app
5. Submit PR

**Example scenario structure:**
```yaml
id: your-scenario-01
title: "Your Title"
steps:
  - type: listen
    prompt: "..."
    content: "..."
  - type: speak_check
    prompt: "..."
    ref_text: "..."
rehearsal:
  decay_alpha: 0.82
  next_ping_sec_min: 90
  next_ping_sec_max: 3600
```

## Commit Messages

Follow conventional commits:

- `feat: add new scenario for grocery shopping`
- `fix: resolve TTS playback issue on Android`
- `docs: update README with new features`
- `refactor: simplify retention calculation`
- `test: add unit tests for scripts.ts`

## Pull Request Process

1. **Ensure tests pass** (when we have them)
2. **Update documentation** if needed
3. **Reference issue number:** "Fixes #123"
4. **Describe changes clearly** in PR description
5. **Request review** from maintainers

## Issue Reporting

Use our issue templates:
- **Bug Report:** [.github/ISSUE_TEMPLATE/bug.md](.github/ISSUE_TEMPLATE/bug.md)
- **Feature Request:** [.github/ISSUE_TEMPLATE/feature.md](.github/ISSUE_TEMPLATE/feature.md)

## Areas We Need Help

### Content Creation
- New YAML scenarios (work, travel, family contexts)
- Pronunciation hints and tips
- Translations (future)

### Development
- See [ISSUES.md](./ISSUES.md) for prioritized tasks
- P1 and P2 issues are good for contributors
- Look for "good first issue" label

### Documentation
- Improve README and guides
- Create video tutorials
- Write blog posts about the approach

### Testing
- Manual testing on different devices
- Report bugs with detailed steps
- Performance testing

## Code of Conduct

Be respectful, inclusive, and constructive.

## Questions?

- Open a discussion on GitHub
- Check [ISSUES.md](./ISSUES.md) for current priorities
- Read [ARCHITECTURE.md](./docs/ARCHITECTURE.md) for technical details

---

**Thank you for contributing to English-Liminal!** 🚀
