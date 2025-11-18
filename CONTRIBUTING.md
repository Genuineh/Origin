# Contributing to Origin

Thank you for your interest in contributing to Origin! This document provides guidelines for contributing to the project.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Process](#development-process)
- [The 15 Iron Rules](#the-15-iron-rules)
- [Coding Standards](#coding-standards)
- [Testing](#testing)
- [Pull Request Process](#pull-request-process)

## Code of Conduct

Origin is committed to providing a welcoming and inclusive environment for all contributors. Be respectful, constructive, and professional in all interactions.

## Getting Started

1. **Fork the repository** on GitHub
2. **Clone your fork** locally:
   ```bash
   git clone https://github.com/YOUR_USERNAME/Origin.git
   cd Origin
   ```
3. **Install dependencies**:
   ```bash
   just install-deps
   ```
4. **Build the project**:
   ```bash
   just build
   ```
5. **Run tests**:
   ```bash
   just test
   ```

## Development Process

### Branching Strategy

- `main`: Stable, production-ready code
- `develop`: Integration branch for features
- `feature/*`: Individual feature branches
- `fix/*`: Bug fix branches

### Workflow

1. Create a feature branch from `develop`:
   ```bash
   git checkout -b feature/my-awesome-feature develop
   ```

2. Make your changes, following the coding standards

3. Test your changes thoroughly:
   ```bash
   just test
   just lint
   ```

4. Commit with clear, descriptive messages:
   ```bash
   git commit -m "feat: add SDF rendering for rounded rectangles"
   ```

5. Push to your fork and create a Pull Request

## The 15 Iron Rules

**These rules are absolute and cannot be violated:**

1. **No Trees**: Never use Widget/Element/Node/Tree terminology or structures
2. **One-Frame Regeneration**: All UI must regenerate in 1 frame
3. **Simple State**: Only f32/u32/bool, packed into Uniforms
4. **No Native Controls**: Zero platform-native UI controls
5. **Pixel Perfect**: ≤ 1px error across platforms
6. **Fast Loading**: < 50ms for design files
7. **Instant Web**: < 100ms cold start
8. **Memory Efficient**: < 60MB peak
9. **Pure Shader Effects**: All effects via SDF + shaders
10. **Mathematical Layout**: Equations only, no trees
11. **u64 Tags**: High 32 bits type, low 32 bits instance ID
12. **No External UI Libs**: egui, iced, etc. forbidden
13. **Single wgpu Instance**: One instance across all platforms
14. **Small Binary**: < 12MB (release + strip)
15. **The Mission**: Origin — Where design returns to its origin.

**Any PR that violates these rules will be rejected.**

## Coding Standards

### Rust Style

- Follow the [Rust Style Guide](https://doc.rust-lang.org/style-guide/)
- Use `rustfmt` for formatting: `just fmt`
- Use `clippy` for linting: `just lint`
- All public items must have documentation

### Documentation

- All public APIs must have doc comments
- Include examples in doc comments where appropriate
- Update ARCHITECTURE.md for architectural changes
- Update TODO.md to track progress

### Comments

```rust
// Good: Explains WHY
// We use SDF for anti-aliasing because it provides perfect results at any scale

// Bad: Explains WHAT (code already shows this)
// Set x to 5
let x = 5;
```

### Naming Conventions

- Use descriptive names: `instance_buffer`, not `buf`
- Avoid abbreviations: `position`, not `pos` (except in math code)
- Constants: `UPPER_SNAKE_CASE`
- Functions: `snake_case`
- Types: `PascalCase`

### Performance

- Always profile before optimizing
- Document performance characteristics
- Add benchmarks for critical paths
- Target: 120fps on all platforms

## Testing

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tag_creation() {
        let tag = make_tag(1, 42);
        assert_eq!(extract_type(tag), 1);
        assert_eq!(extract_id(tag), 42);
    }
}
```

### Integration Tests

Place in `tests/` directory:

```rust
// tests/rendering.rs
#[test]
fn test_render_rounded_rect() {
    // Test rendering logic
}
```

### Benchmarks

Place in `benches/` directory:

```rust
// benches/layout.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_constraint_solver(c: &mut Criterion) {
    c.bench_function("solve 1000 constraints", |b| {
        b.iter(|| {
            // Benchmark code
        });
    });
}

criterion_group!(benches, bench_constraint_solver);
criterion_main!(benches);
```

## Pull Request Process

1. **Update documentation** for any changed functionality
2. **Add tests** for new features
3. **Run the full test suite**: `just test`
4. **Run linter**: `just lint`
5. **Check binary size**: `just size` (must be < 12MB)
6. **Update TODO.md** to check off completed items
7. **Fill out the PR template** completely
8. **Request review** from maintainers

### PR Title Format

Use conventional commits format:

- `feat: add new feature`
- `fix: fix bug`
- `docs: update documentation`
- `perf: improve performance`
- `refactor: refactor code`
- `test: add tests`
- `chore: update dependencies`

### PR Description

Include:

- **What**: What does this PR do?
- **Why**: Why is this change needed?
- **How**: How does it work?
- **Testing**: How was it tested?
- **Performance**: Any performance implications?
- **Screenshots**: For visual changes

### Review Process

1. Automated checks must pass (CI/CD)
2. At least one maintainer approval required
3. No unresolved comments
4. All 15 iron rules must be respected

## Areas Needing Contribution

### High Priority

- [ ] Implement SDF rendering for primitives
- [ ] Build constraint solver for layout
- [ ] Create Figma adapter
- [ ] Optimize instance buffer updates
- [ ] Add MSDF text rendering

### Medium Priority

- [ ] Additional design tool adapters (Sketch, XD)
- [ ] Advanced shader effects
- [ ] Animation system
- [ ] Accessibility features

### Low Priority

- [ ] Additional examples
- [ ] Documentation improvements
- [ ] Website enhancements

## Getting Help

- **Discord**: [Join our Discord](https://discord.gg/origin) (coming soon)
- **GitHub Discussions**: Ask questions in Discussions
- **GitHub Issues**: Report bugs or request features

## Recognition

Contributors will be:

- Listed in CONTRIBUTORS.md
- Acknowledged in release notes
- Credited in documentation

Thank you for contributing to Origin! Together, we're revolutionizing UI development.

**Origin — Where design returns to its origin.** ⚡
