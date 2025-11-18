# Contributing to @sylphx/webgpu

## Development Setup

### Prerequisites
- Node.js 18+
- Rust (install via [rustup](https://rustup.rs))
- Cargo

### Getting Started

```bash
# Clone repository
git clone https://github.com/SylphxAI/webgpu.git
cd webgpu

# Install dependencies
npm install

# Build (first build takes longer)
npm run build

# Run tests
npm test

# Run examples
node examples/basic.js
```

## Project Structure

```
webgpu/
├── src/
│   ├── lib.rs         # Main library entry
│   ├── gpu.rs         # GPU instance
│   ├── adapter.rs     # GPU adapter
│   ├── device.rs      # GPU device
│   ├── buffer.rs      # Buffer operations
│   ├── texture.rs     # Texture operations
│   └── constants.rs   # WebGPU constants
├── examples/          # Example code
├── test/              # Test files
├── Cargo.toml         # Rust dependencies
└── package.json       # npm package config
```

## Building

```bash
# Debug build (faster, larger binary)
npm run build:debug

# Release build (slower, smaller binary)
npm run build
```

## Testing

```bash
# Run all tests
npm test

# Run specific example
node examples/basic.js
node examples/compute.js
```

## Code Style

### Rust
- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Run `cargo clippy` before committing
- Format code with `cargo fmt`

### JavaScript
- Use semicolons
- Use `const` by default
- Prefer async/await over promises

## Adding New Features

1. **Add Rust implementation** in `src/`
2. **Export via napi** using `#[napi]` macro
3. **Add example** in `examples/`
4. **Add test** in `test/`
5. **Update README** with new API

## Pull Request Process

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Run tests (`npm test`)
5. Commit changes (`git commit -m 'Add amazing feature'`)
6. Push to branch (`git push origin feature/amazing-feature`)
7. Open a Pull Request

## Commit Message Convention

```
type(scope): description

[optional body]
```

Types:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation
- `refactor`: Code refactoring
- `test`: Add tests
- `chore`: Maintenance

Examples:
```
feat(buffer): add mapped buffer support
fix(device): handle device lost error
docs(readme): update installation instructions
```

## Release Process

1. Update version in `package.json` and `Cargo.toml`
2. Update `CHANGELOG.md`
3. Commit: `git commit -m "chore: release v0.2.0"`
4. Tag: `git tag v0.2.0`
5. Push: `git push --tags`
6. CI builds and publishes automatically

## Questions?

Open an issue or discussion on GitHub!
