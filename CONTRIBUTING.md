# Contributing to Oxide Message Framework

Thank you for your interest in contributing to the Oxide Message Framework! This document provides guidelines and instructions for contributing.

## Development Setup

1. **Install Rust**
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Clone the repository**
   ```bash
   git clone https://github.com/willgraham345/oxide_msg.git
   cd oxide_msg
   ```

3. **Build the project**
   ```bash
   cargo build
   ```

4. **Run tests**
   ```bash
   cargo test
   ```

## Project Structure

```
oxide_msg/
├── src/
│   ├── lib.rs           # Main library entry point
│   ├── error.rs         # Error types and handling
│   ├── message.rs       # Message structure and serialization
│   └── patterns/        # Messaging pattern implementations
│       ├── pubsub.rs    # Publisher/Subscriber pattern
│       ├── reqrep.rs    # Request/Reply pattern
│       └── pipeline.rs  # Push/Pull pattern
├── examples/            # Usage examples
└── tests/              # Integration tests (if any)
```

## How to Contribute

### Reporting Bugs

1. Check if the bug has already been reported in [Issues](https://github.com/willgraham345/oxide_msg/issues)
2. If not, create a new issue with:
   - Clear title and description
   - Steps to reproduce
   - Expected vs actual behavior
   - Code samples if applicable
   - Rust version and OS

### Suggesting Features

1. Open an issue with the `enhancement` label
2. Describe the feature and use case
3. Provide examples of how it would be used
4. Discuss implementation approach

### Pull Requests

1. **Fork the repository** and create a new branch
   ```bash
   git checkout -b feature/my-new-feature
   ```

2. **Make your changes**
   - Write clear, documented code
   - Follow Rust conventions (use `cargo fmt`)
   - Add tests for new functionality
   - Update documentation as needed

3. **Run tests and checks**
   ```bash
   cargo test
   cargo fmt --check
   cargo clippy -- -D warnings
   cargo doc --no-deps
   ```

4. **Commit your changes**
   ```bash
   git commit -m "Add feature: description"
   ```

5. **Push and create a pull request**
   ```bash
   git push origin feature/my-new-feature
   ```

## Code Style

- Follow standard Rust formatting (use `cargo fmt`)
- Use meaningful variable and function names
- Add documentation comments for public APIs
- Keep functions focused and single-purpose
- Use Result types for error handling

### Documentation

All public APIs should have documentation:

```rust
/// Creates a new publisher that binds to the specified address.
///
/// # Arguments
///
/// * `address` - The ZeroMQ address to bind to (e.g., "tcp://127.0.0.1:5555")
///
/// # Examples
///
/// ```
/// use oxide_msg::Publisher;
/// let publisher = Publisher::new("tcp://127.0.0.1:5555")?;
/// ```
pub fn new(address: &str) -> Result<Self> {
    // implementation
}
```

## Testing Guidelines

### Writing Tests

1. **Unit tests** should be in the same file as the code
   ```rust
   #[cfg(test)]
   mod tests {
       use super::*;
       
       #[test]
       fn test_feature() {
           // test code
       }
   }
   ```

2. **Integration tests** should be in the `tests/` directory

3. **Use unique ports** for each test to avoid conflicts
   ```rust
   let publisher = Publisher::new("tcp://127.0.0.1:15555")?;
   ```

4. **Add delays** when needed for ZeroMQ synchronization
   ```rust
   thread::sleep(Duration::from_millis(100));
   ```

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run with output
cargo test -- --nocapture

# Run tests in single thread (for debugging)
cargo test -- --test-threads=1
```

## Adding Examples

Examples should:
1. Be placed in the `examples/` directory
2. Demonstrate a complete use case
3. Include comments explaining key concepts
4. Be runnable with `cargo run --example <name>`

## Documentation

- Update README.md for user-facing changes
- Update USAGE.md for new features or patterns
- Add doc comments to all public APIs
- Run `cargo doc` to check documentation

## Release Process

(For maintainers)

1. Update version in `Cargo.toml`
2. Update CHANGELOG.md
3. Run tests: `cargo test`
4. Build docs: `cargo doc --no-deps`
5. Create git tag: `git tag v0.x.x`
6. Push: `git push && git push --tags`
7. Publish: `cargo publish`

## Code of Conduct

- Be respectful and inclusive
- Provide constructive feedback
- Focus on the code, not the person
- Help others learn and grow

## Questions?

If you have questions, feel free to:
- Open an issue for discussion
- Comment on existing issues
- Reach out to maintainers

## License

By contributing, you agree that your contributions will be licensed under the same license as the project.

Thank you for contributing to Oxide! 🦀
