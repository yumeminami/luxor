# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Luxor is a Rust implementation of the Python Rich library - a library for rich text and beautiful formatting in the terminal. This project aims to recreate Rich's functionality in Rust with improved performance and memory safety.

## Repository Structure

**Current State (Early Development):**
```
luxor/
├── crates/
│   └── luxor/              # Main Rust crate (currently just Hello World)
│       ├── Cargo.toml      # Rust package configuration
│       └── src/
│           └── main.rs     # Simple binary entry point
├── rich/                   # Python Rich source code for reference
├── uv/                     # UV Python package manager (reference)
├── CLAUDE.md              # This file
├── CLAUDE_CN.md           # Chinese version of this file
└── README.md
```

**Planned Structure (After Implementation):**
```
luxor/
├── crates/
│   └── luxor/
│       ├── Cargo.toml      # Main package configuration
│       └── src/
│           ├── lib.rs      # Library entry point
│           ├── color.rs    # Color system and color types
│           ├── console.rs  # Core rendering engine
│           ├── style.rs    # Text styling system
│           ├── segment.rs  # Minimal rendering units
│           ├── text.rs     # Rich text implementation
│           ├── protocol.rs # Core traits (Renderable, Measurable)
│           ├── measure.rs  # Width measurement system
│           ├── layout.rs   # Layout and positioning system
│           └── components/ # UI components
├── rich/                   # Python Rich source code for reference
└── examples/              # Usage examples and demos
```

## Development Setup

### Build Commands

**Current Commands (for the simple binary):**
```bash
# Navigate to the Rust crate
cd crates/luxor

# Build the project
cargo build

# Run the hello world program
cargo run

# Check code formatting
cargo fmt --check

# Run clippy for linting
cargo clippy -- -D warnings

# Run tests (when available)
cargo test
```

**Planned Commands (after library implementation):**
```bash
# Build the library
cargo build -p luxor

# Run all tests
cargo test -p luxor

# Run a specific test
cargo test -p luxor test_name

# Run benchmarks (when implemented)
cargo bench -p luxor

# Build documentation
cargo doc -p luxor --open

# Run examples (when implemented)
cargo run --example basic_usage
```

### Development Dependencies (Planned)
- `crossterm` - Cross-platform terminal manipulation
- `unicode-width` - Unicode character width calculation
- `serde` - Serialization support
- `thiserror` - Error handling
- `syntect` (optional) - Syntax highlighting
- `criterion` (dev) - Benchmarking

### Key Development Files
- `rich/` - Complete Python Rich source code for reference and API understanding
- `crates/luxor/src/main.rs` - Current simple binary (will be replaced with lib.rs)
- `CLAUDE_CN.md` - Chinese version of development guidance

### Development Configuration Files
- `.github/workflows/ci.yml` - GitHub Actions CI/CD pipeline
- `rustfmt.toml` - Rust code formatting configuration  
- `clippy.toml` - Clippy linting configuration
- `rust-toolchain.toml` - Rust toolchain specification
- `.gitignore` - Git ignore patterns for Rust and general development

## Implementation Progress

**Status: Planning/Early Development**

The project currently contains:
- ✅ Python Rich source code analysis (in `rich/` directory)
- ✅ Comprehensive implementation plan
- ✅ Basic Rust project structure
- ✅ CI/CD pipeline with GitHub Actions
- ✅ Code formatting and linting configuration
- ❌ Core Rust implementation (not started)
- ❌ Tests (not implemented)
- ❌ Examples (not implemented)

## Rich Python Library Analysis

**Key Files to Study:**
- `rich/console.py` - Core rendering engine (~2000+ lines)
- `rich/segment.py` - Fundamental rendering unit
- `rich/style.py` - Style system implementation
- `rich/text.py` - Rich text with markup
- `rich/protocol.py` - Renderable interface definition
- `rich/measure.py` - Width measurement system

### Core Architecture

Rich uses a layered rendering architecture:

1. **Console** (rich/console.py:Console) - Central rendering engine that manages output streams and rendering options
2. **Segment** (rich/segment.py:Segment) - Minimal rendering unit containing text, style, and control codes  
3. **Renderable Protocol** (rich/protocol.py) - Objects implement `__rich_console__()` or `__rich__()` methods
4. **Two-phase rendering** - Measurement phase followed by rendering phase

### Key Components

**Style System:**
- `Style` class manages text attributes (color, bold, italic, etc.)
- `Color` class supports standard, 8-bit, and truecolor
- `Theme` class manages style collections

**Layout System:**
- `Measurement` calculates minimum/maximum width requirements
- `Layout` provides flexible splitting (horizontal/vertical)
- `Padding` handles spacing around content

**Component System:**
- `Text` - Rich text with markup support
- `Table` - Auto-sizing tables with styling
- `Panel` - Bordered containers
- `Progress` - Progress bars and spinners
- `Tree`, `Columns`, `Rule` - Additional UI components

**Rendering Pipeline:**
1. Objects implement rendering protocol via `__rich_console__()`
2. Console calls measure phase to determine layout requirements
3. Console calls render phase to generate `Segment` objects
4. Segments are written to output stream with ANSI codes

### Design Patterns Used
- **Protocol-oriented design** - Duck typing with `__rich__()` methods
- **Composition pattern** - Components can contain other renderables
- **Visitor pattern** - Console "visits" renderables to measure and render
- **Strategy pattern** - Different alignment, overflow, and justify strategies

## Rust Implementation Strategy

### Phase 1: Core Foundation (2-3 weeks)
**Core Traits:**
```rust
trait Renderable {
    fn render(&self, console: &Console, options: &ConsoleOptions) -> RenderResult;
}

trait Measurable {
    fn measure(&self, console: &Console, options: &ConsoleOptions) -> Measurement;
}
```

**Basic Types:**
- `Style` struct with color, bold, italic flags
- `Color` enum supporting Standard/EightBit/TrueColor
- `Segment` struct with text, style, and control codes
- `Console` struct managing output and rendering

### Phase 2: Text and Measurement (2-3 weeks)
- `Text` implementation with rich text support
- Unicode width calculation using `unicode-width` crate
- `Measurement` system for layout calculations
- ANSI escape sequence generation

### Phase 3: Core Components (3-4 weeks)
- `Panel` component with borders
- `Table` component with auto-layout
- `Progress` bars and spinners
- `Rule` for horizontal lines

### Phase 4: Advanced Features (2-3 weeks)
- `Layout` system for flexible positioning
- Live updating support
- Syntax highlighting integration
- Markdown rendering

### Phase 5: Optimization (1-2 weeks)
- Performance tuning
- Memory optimization
- Concurrency safety
- Additional components

## Technical Challenges

### High Complexity (9/10)
- **Unicode width calculation** - Complex character width rules
- **ANSI escape sequences** - Terminal compatibility and color handling
- **Dynamic styling** - Style inheritance and composition in Rust's ownership model

### Medium Complexity (6/10)
- **Component composition** - Using `Box<dyn Trait>` for renderable collections
- **Layout algorithms** - Constraint solving for flexible layouts
- **Measurement system** - Recursive width calculation

### Lower Complexity (3/10)
- **Basic rendering** - String building and output
- **Color conversion** - RGB/HSL/terminal color mapping
- **Simple components** - Straightforward struct implementations

## Performance Goals

Target 10-100x performance improvement over Python Rich:
- Zero-cost abstractions where possible
- Efficient string handling with `String`/`&str`
- Minimal allocations in hot paths
- SIMD optimizations for text processing where applicable

## API Design Principles

1. **Familiarity** - Keep API similar to Python Rich where possible
2. **Rust idioms** - Use Result types, iterators, and ownership properly
3. **Zero-cost** - Abstract without runtime overhead
4. **Composability** - Components should work together seamlessly
5. **Thread safety** - All types should be Send + Sync where reasonable

## Testing Strategy

- Unit tests for each component
- Integration tests for full rendering pipeline
- Property-based testing for layout algorithms
- Performance regression tests
- Cross-platform terminal compatibility tests

## Time Estimate

**Total**: 10-15 weeks (2.5-4 months)
- **MVP (core features)**: 6-8 weeks
- **Feature complete**: 10-12 weeks
- **Optimized**: 13-15 weeks

The implementation should be done incrementally with each phase producing a working subset of functionality.

## Getting Started with Development

**For new contributors:**

1. **Study the Python Rich implementation first:**
   ```bash
   # Examine core files
   head -50 rich/rich/console.py    # Understand Console class
   head -50 rich/rich/segment.py    # Understand Segment structure
   head -50 rich/rich/protocol.py   # Understand renderable protocol
   ```

2. **Set up the Rust development environment:**
   ```bash
   cd crates/luxor
   cargo build    # Should work immediately
   cargo run      # Runs "Hello, world!"
   ```

3. **Begin implementation following the phase plan:**
   - Start with `src/lib.rs` and basic traits
   - Implement `Color` and `Style` structs
   - Add `Segment` and basic rendering
   - Build incrementally with tests

**Architecture Decision Records:**
- Use trait objects (`Box<dyn Renderable>`) for dynamic dispatch
- Prefer `&str` over `String` where possible for performance
- Use `Result<T, LuxorError>` for error handling
- Follow Rust naming conventions (snake_case for functions, PascalCase for types)

## Development Workflow

### Code Quality Standards
The project enforces code quality through automated checks:

1. **Formatting**: All code must pass `cargo fmt --check`
2. **Linting**: All code must pass `cargo clippy -- -D warnings`  
3. **Testing**: All tests must pass `cargo test`
4. **Documentation**: Public APIs must be documented

### CI/CD Pipeline
GitHub Actions automatically runs on:
- Push to main/master branch
- Pull requests
- Manual workflow dispatch

The pipeline includes:
- **Change Detection**: Skips unnecessary runs for documentation-only changes
- **Cross-platform Building**: Tests on Ubuntu, Windows, and macOS
- **Code Quality Checks**: Formatting, linting, and documentation
- **Testing**: Runs all tests across different configurations

### Pre-commit Workflow
Before committing code:
```bash
cd crates/luxor
cargo fmt                    # Format code
cargo clippy -- -D warnings # Check for lints
cargo test                   # Run tests
cargo doc --no-deps         # Build documentation
```

### Pre-commit Hooks
Pre-commit hooks are automatically installed and will run:
- **typos**: Spell checking across all files
- **cargo fmt**: Rust code formatting verification
- **cargo clippy**: Rust linting with warnings as errors
- **cargo test**: Run all Rust tests
- **prettier**: YAML/JSON formatting for CI files

Install and enable pre-commit hooks:
```bash
uv tool install pre-commit  # Install pre-commit
pre-commit install          # Enable hooks for this repo
pre-commit run --all-files  # Test all hooks manually
```