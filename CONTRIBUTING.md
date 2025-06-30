
# Contributing to WasmRunner

We welcome contributions to WasmRunner! This document provides guidelines for contributing to the project.

## Development Setup

### Prerequisites
- Rust 1.75 or later
- Git
- Linux or macOS (Windows support coming soon)

### Setup Instructions

1. **Clone the repository**
   ```bash
   git clone https://github.com/yourorg/wasmrunner.git
   cd wasmrunner
   ```

2. **Install Rust toolchain**
   ```bash
   rustup target add wasm32-wasi
   rustup component add rustfmt clippy
   ```

3. **Install additional tools**
   ```bash
   cargo install cargo-audit cargo-fuzz
   ```

4. **Build the project**
   ```bash
   cargo build
   ```

5. **Run tests**
   ```bash
   cargo test
   ```

## Project Structure

```
wasmrunner/
├── crates/
│   ├── wasmrunner-cli/     # CLI interface
│   ├── wasmrunner-core/    # Core types and logic
│   ├── wasmrunner-runtime/ # WASM runtime abstraction
│   ├── wasmrunner-sandbox/ # Security and isolation
│   ├── wasmrunner-plugins/ # Plugin system
│   └── wasmrunner-store/   # Registry and storage
├── examples/               # Sample applications
├── docs/                   # Documentation
├── tests/                  # Integration tests
└── fuzz/                   # Fuzz testing targets
```

## Coding Standards

### Rust Style Guide
- Follow the official [Rust Style Guide](https://doc.rust-lang.org/nightly/style-guide/)
- Use `cargo fmt` to format code
- Run `cargo clippy` and fix all warnings
- Write documentation for public APIs

### Code Quality
- **Safety**: Minimize `unsafe` code usage
- **Error Handling**: Use `Result` types consistently
- **Testing**: Write unit tests for new functionality
- **Documentation**: Document public APIs with examples

### Security Considerations
- Review all security-sensitive code carefully
- Follow the principle of least privilege
- Validate all inputs from untrusted sources
- Use secure defaults in configurations

## Development Workflow

### 1. Issue Discussion
- Check existing issues before creating new ones
- Discuss significant changes in issues first
- Use issue templates when available

### 2. Feature Development
- Create a feature branch: `git checkout -b feature/description`
- Make atomic commits with clear messages
- Write tests for new functionality
- Update documentation as needed

### 3. Code Review Process
- Create a pull request with clear description
- Ensure all CI checks pass
- Address review feedback promptly
- Squash commits if requested

### 4. Testing Requirements
- All new code must have tests
- Maintain or improve code coverage
- Run integration tests locally
- Consider security implications

## Security Guidelines

### Vulnerability Reporting
- Report security issues privately to security@wasmrunner.dev
- Do not create public issues for security vulnerabilities
- Allow time for fixes before disclosure

### Security Code Review
- All security-sensitive changes require additional review
- Use static analysis tools (clippy, cargo-audit)
- Consider threat model implications
- Test against malicious inputs

## Documentation

### Types of Documentation
- **API Documentation**: Inline rustdoc comments
- **User Guides**: Markdown files in `docs/`
- **Examples**: Working code in `examples/`
- **Architecture**: High-level design documents

### Documentation Standards
- Write clear, concise explanations
- Include code examples where helpful
- Keep documentation up to date with code changes
- Use proper markdown formatting

## Release Process

### Version Numbering
- Follow [Semantic Versioning](https://semver.org/)
- Major: Breaking changes
- Minor: New features (backward compatible)
- Patch: Bug fixes

### Release Checklist
- [ ] Update version numbers in Cargo.toml files
- [ ] Update CHANGELOG.md
- [ ] Run full test suite
- [ ] Update documentation
- [ ] Create release branch
- [ ] Tag release
- [ ] Publish to crates.io

## Community Guidelines

### Code of Conduct
- Be respectful and inclusive
- Welcome newcomers and help them learn
- Focus on constructive feedback
- Maintain a professional tone

### Communication Channels
- **GitHub Issues**: Bug reports and feature requests
- **GitHub Discussions**: General questions and ideas
- **Discord**: Real-time chat (link in README)
- **Email**: Security issues and private matters

## Getting Help

### Resources
- **Documentation**: Check the docs/ directory
- **Examples**: Look at working examples
- **Issues**: Search existing issues for solutions
- **Community**: Ask questions in discussions

### Mentorship
- New contributors welcome!
- Tag issues with "good first issue"
- Maintainers available to help guide contributions
- Pair programming sessions available on request

## Recognition

### Contributors
- All contributors listed in CONTRIBUTORS.md
- Significant contributions recognized in release notes
- Top contributors invited to maintainer team

### Sponsorship
- Financial support through GitHub Sponsors
- Corporate sponsorship opportunities available
- Recognition for sponsors in documentation

Thank you for contributing to WasmRunner! 🚀
