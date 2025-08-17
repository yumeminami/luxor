# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Luxor is a Rust implementation of the Python Rich library - a library for rich text and beautiful formatting in the terminal. This project aims to recreate Rich's functionality in Rust with improved performance and memory safety.

## Repository Structure

**Current Implementation State:**
```
luxor/
├── crates/
│   └── luxor/              # Main Rust library crate
│       ├── Cargo.toml      # Package configuration with dependencies
│       ├── src/
│       │   ├── lib.rs      # Library entry point with re-exports
│       │   ├── ansi.rs     # ANSI escape sequence generation
│       │   ├── color.rs    # Color system (Standard/8-bit/TrueColor)
│       │   ├── console.rs  # Core rendering engine and Text struct
│       │   ├── error.rs    # Error handling with thiserror
│       │   ├── measure.rs  # Width measurement and layout calculations
│       │   ├── protocol.rs # Core traits (Renderable, Measurable)
│       │   ├── segment.rs  # Fundamental rendering units
│       │   └── style.rs    # Text styling with composition
│       ├── tests/
│       │   ├── integration_tests.rs  # End-to-end functionality tests
│       │   └── property_tests.rs     # Property-based testing with proptest
│       └── benches/
│           ├── render_bench.rs       # Basic benchmarks
│           └── comprehensive_bench.rs # Detailed performance tests
├── rich/                   # Python Rich source code for reference
├── CLAUDE.md              # This file
└── CLAUDE_CN.md           # Chinese version of this file
```

## Development Commands

All commands should be run from `crates/luxor/`:

```bash
# Navigate to the main crate
cd crates/luxor

# Build the library
cargo build

# Run all tests (138 tests: unit + integration + property + doc tests)
cargo test

# Run specific test suites
cargo test --test integration_tests    # Integration tests
cargo test --test property_tests       # Property-based tests with proptest
cargo test --lib                       # Unit tests only
cargo test --doc                       # Documentation tests only

# Run a specific test by name
cargo test test_color_system_compatibility

# Run tests with output
cargo test -- --nocapture

# Check code formatting
cargo fmt --check

# Format code
cargo fmt

# Run clippy linting (enforces warnings as errors)
cargo clippy -- -D warnings

# Build and open documentation
cargo doc --open

# Run benchmarks
cargo bench

# Run specific benchmark
cargo bench -- text_rendering
```

## Core Architecture

### Implemented Phase 1: Core Foundation

The library uses a trait-based architecture with two-phase rendering:

**Core Traits:**
- `Renderable` - Objects that can render to segments via `render(console, options) -> Vec<Segment>`
- `Measurable` - Objects that can calculate layout dimensions via `measure(console, options) -> Measurement`

**Rendering Pipeline:**
1. **Measurement Phase**: Calculate minimum/maximum width requirements
2. **Rendering Phase**: Generate styled text segments with ANSI codes
3. **Output Phase**: Write segments to terminal with proper escape sequences

**Key Types:**
- `Console` - Central rendering engine managing terminal state and options
- `Segment` - Fundamental rendering unit containing text + style + optional control codes
- `Style` - Styling attributes (colors, bold, italic, etc.) with composition support
- `Color` - Multi-format color system (Standard 16, 8-bit 256, TrueColor 24-bit)
- `Measurement` - Layout constraints with min/max width calculations

### Module Overview

**Core Modules:**
- `protocol.rs` - Defines `Renderable` and `Measurable` traits with trait object support
- `color.rs` - Complete color system with RGB/8-bit/Standard conversion algorithms  
- `style.rs` - Style composition, inheritance, and string parsing ("bold red on blue")
- `segment.rs` - Text segments with Unicode-aware width calculation and splitting
- `console.rs` - Terminal management, rendering pipeline, and `Text` struct
- `measure.rs` - Layout measurement system with constraint solving
- `ansi.rs` - ANSI escape sequence generation and text processing utilities
- `error.rs` - Structured error handling using `thiserror`

**Testing:**
- Comprehensive unit tests (62 tests) in each module
- Integration tests (12 tests) for end-to-end functionality  
- Property-based tests (14 tests) using `proptest` for invariant validation
- Documentation tests (50 tests) ensuring examples work correctly

## Implementation Status

**✅ Phase 1 Complete: Core Foundation**
- Trait-based architecture with `Renderable`/`Measurable` protocols
- Complete color system (Standard/8-bit/TrueColor) with conversion
- Style composition and inheritance system
- Unicode-aware text processing and measurement
- ANSI escape sequence generation
- Console rendering engine with terminal detection
- Comprehensive test coverage (138 tests passing)
- Zero clippy warnings with strict quality enforcement

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

## Development Roadmap - Comprehensive Rich Implementation

**⚠️ SCOPE UPDATE:** After analyzing the complete Python Rich library (54 modules), the original roadmap covered only ~30% of Rich's features. This expanded roadmap aims for full feature parity.

### ✅ Phase 1: Core Foundation (Completed - 15% of Rich features)
**Implemented:**
- Core `Renderable` and `Measurable` traits with trait object support
- Complete `Color` system with Standard/8-bit/TrueColor conversion algorithms
- `Style` composition system with inheritance and string parsing  
- `Segment` rendering units with Unicode-aware text processing
- `Console` rendering engine with terminal capability detection
- `Measurement` system for layout constraint solving
- ANSI escape sequence generation and text processing utilities
- Comprehensive error handling with structured types

### 🔄 Phase 2: Text & Markup System (4-5 weeks - Next Priority)
**Critical Foundation for all other components:**
- **Rich Markup Parser** - BBCode-like syntax `[bold red]text[/bold red]`
- **Text Spans** - Style ranges within text with proper composition
- **Word Wrapping** - Intelligent text flow with Unicode awareness
- **Text Alignment** - Left, center, right, justify with overflow handling
- **Highlighter System** - Regex-based text highlighting framework
- **Enhanced Text struct** - Full-featured text handling with markup support

### 📋 Phase 3: Layout Foundation (3-4 weeks)
**Essential for complex UI components:**
- **`Layout` class** - Flexible horizontal/vertical splits with constraints
- **`Padding`** - Space management around content with various padding modes
- **`Align`** - Content alignment within containers (Center, Left, Right)
- **`Constrain`** - Size constraints and content overflow handling
- **`Region`** - Rectangular screen regions with intersection/clipping
- **Ratio Resolution** - Proportional layout distribution algorithms

### 🎨 Phase 4: Core UI Components (4-5 weeks)
**Most commonly used Rich components:**
- **`Panel`** - Bordered containers with titles, subtitles, and various box styles
- **`Table`** - Feature-rich tables with auto-sizing, sorting, styling, borders
- **`Rule`** - Horizontal/vertical lines and separators with styling
- **`Bar`** - Progress bars and data visualization bars with gradients

### 🌳 Phase 5: Advanced Components (4-5 weeks)
**Complex interactive and display components:**
- **`Tree`** - Hierarchical tree display with guide lines and icons
- **`Columns`** - Multi-column layouts with equal/optimal width distribution
- **`Progress`** - Multiple progress bar system with live updates and ETA
- **`Status`** - Spinner animations with customizable indicators
- **`Live`** - Live updating displays with refresh control and threading

### 📝 Phase 6: Content Rendering (4-6 weeks)
**Rich content processing and display:**
- **`Markdown`** - Complete markdown rendering with code blocks, tables, links
- **`Pretty`** - Python object pretty printing with syntax highlighting
- **`JSON`** - Pretty JSON rendering with highlighting and validation
- **`Repr`** - Enhanced object representation with type information
- **`Syntax`** - Code syntax highlighting (subset - most common languages)
- **`Traceback`** - Beautiful error traceback rendering with context

### ⚡ Phase 7: Interactive & Dynamic Features (3-4 weeks)
**User interaction and dynamic content:**
- **`Prompt`** - Enhanced input prompts with validation and auto-completion
- **`Screen`** - Alternate screen buffer management for full-screen apps
- **Emoji Support** - Emoji name resolution (`:smiley:` → 😃) with fallbacks
- **`Inspect`** - Object inspection and analysis with interactive exploration
- **Rich Print** - Drop-in replacement for print() with automatic rich formatting
- **Input/Output** - File operations and stream handling

### 🔧 Phase 8: Integration & Platform Support (2-3 weeks)
**Export capabilities and platform compatibility:**
- **HTML/SVG Export** - Export rich content to web formats with styling
- **Theme System** - Customizable color themes and style presets
- **Jupyter Integration** - Rich display in notebooks with interactive features
- **Logging Integration** - Rich logging handler with structured output
- **Platform Compatibility** - Windows console support and terminal detection
- **Box Drawing** - Various box/border styles with Unicode support

## Updated Technical Challenges

### Extreme Complexity (10/10) - NEW
- **Rich Markup Parser** - Complex BBCode-like syntax with nested tags and validation
- **Layout Constraint Solver** - Multi-dimensional layout with flexible constraints
- **Live Updating System** - Thread-safe dynamic content with minimal flicker
- **Terminal State Management** - Complex terminal control and capability detection

### High Complexity (9/10)
- **Unicode width calculation** - Complex character width rules with emoji support
- **ANSI escape sequences** - Terminal compatibility and color handling
- **Dynamic styling** - Style inheritance and composition in Rust's ownership model
- **Markdown/Syntax Rendering** - Complex parsing with syntax highlighting

### Medium Complexity (6/10)
- **Component composition** - Using `Box<dyn Trait>` for renderable collections
- **Layout algorithms** - Constraint solving for flexible layouts
- **Measurement system** - Recursive width calculation
- **Content export** - HTML/SVG generation with proper formatting

### Lower Complexity (3/10)
- **Basic rendering** - String building and output
- **Color conversion** - RGB/HSL/terminal color mapping
- **Simple components** - Straightforward struct implementations

## Realistic Timeline

**Complete Rich Feature Parity:**
- **Conservative Estimate:** 26-35 weeks (6.5-8.5 months)
- **Aggressive Estimate:** 20-28 weeks (5-7 months)
- **Current Phase 1:** 3 weeks (completed)

**MVP Subset (Core Features Only):**
- **Phases 1-4 Only:** 12-16 weeks (3-4 months)
- **Essential Components:** Text markup, Layout, Panel, Table, Progress, Tree

## Performance Goals

Target 10-100x performance improvement over Python Rich:
- Zero-cost abstractions where possible
- Efficient string handling with `String`/`&str`
- Minimal allocations in hot paths
- SIMD optimizations for text processing where applicable

## Key Implementation Patterns

### Error Handling
- Use `Result<T, LuxorError>` consistently throughout the API
- Structured errors with `thiserror` for clear error messages
- Never panic in public APIs - return errors for invalid inputs

### Style Composition
```rust
// Styles compose via layering - later styles override earlier ones
let base = Style::new().bold().color(Color::red());
let overlay = Style::new().italic().color(Color::blue());
let combined = base.combine(overlay); // blue italic bold text
```

### Trait Objects for Extensibility
```rust
// Use trait objects to allow heterogeneous collections
let items: Vec<Box<dyn Renderable>> = vec![
    Box::new(Text::new("Hello")),
    Box::new(Panel::new("Content")),
];
```

### Two-Phase Rendering
```rust
// All renderables follow the measure -> render pattern
let measurement = item.measure(&console, &options)?;
let segments = item.render(&console, &options)?;
```

## Dependencies

**Core Runtime:**
- `crossterm` (0.27) - Cross-platform terminal manipulation
- `unicode-width` (0.1) - Unicode character width calculation
- `thiserror` (1.0) - Structured error handling

**Development/Testing:**
- `criterion` (0.5) - Performance benchmarking
- `proptest` (1.0) - Property-based testing for invariants

## Development Guidelines

### Code Quality Standards
All code must pass these checks before committing:

```bash
cd crates/luxor
cargo fmt --check          # Code formatting
cargo clippy -- -D warnings # Linting (warnings as errors)
cargo test                 # All tests must pass
cargo doc --no-deps        # Documentation builds without errors
```

### Testing Requirements
- **Unit tests**: Each module must have comprehensive tests
- **Integration tests**: End-to-end functionality in `tests/integration_tests.rs`
- **Property tests**: Invariant validation in `tests/property_tests.rs`  
- **Documentation tests**: All public API examples must work

### Style Guide
- Use `Result<T, LuxorError>` for fallible operations
- Prefer `&str` over `String` in function parameters
- All public APIs must be documented with examples
- Follow Rust naming conventions throughout
- Use trait objects (`Box<dyn Trait>`) for heterogeneous collections

## Python Rich Reference

The `rich/` directory contains the complete Python Rich source code for reference. Key files for understanding the architecture:

- `rich/console.py` - Core rendering engine (~2000+ lines)
- `rich/segment.py` - Fundamental rendering unit  
- `rich/protocol.py` - Renderable interface definition
- `rich/style.py` - Style system implementation
- `rich/text.py` - Rich text with markup support
- `rich/measure.py` - Width measurement system

This reference implementation helps ensure API compatibility and understand complex rendering behaviors.

## Critical Missing Components Analysis

Based on comprehensive analysis of the Python Rich library, here are the most critical missing components that need immediate attention:

### High Priority Missing Features

**1. Rich Markup Parser (BLOCKING ALL UI COMPONENTS)**
- Current: Only basic `Style` application
- Missing: BBCode-like markup parsing `[bold red]text[/bold red]`
- Impact: Required for Panel titles, Table cell formatting, all text rendering
- Examples in Rich: `console.print("[bold cyan]Hello[/bold cyan] World!")`

**2. Layout System (BLOCKING COMPLEX UIs)**
- Current: No layout management  
- Missing: `Layout`, `Padding`, `Align`, `Region` classes
- Impact: Cannot create Panel borders, Table layouts, multi-column displays
- Examples in Rich: Flexible terminal applications with split panes

**3. Text Processing Infrastructure (BLOCKING TEXT FEATURES)**
- Current: Basic `Text` struct
- Missing: Word wrapping, justification, overflow handling, text spans
- Impact: All text-heavy components will have poor rendering
- Examples in Rich: Table cell wrapping, justified text, text highlighting

**4. Core UI Components (USER-FACING FEATURES)**
- Current: None implemented
- Missing: `Panel`, `Table`, `Tree`, `Progress`, `Rule`
- Impact: No visible Rich-like output possible
- Examples in Rich: All the beautiful terminal UIs Rich is known for

### Medium Priority Missing Features

**5. Live Display System**
- Missing: `Live`, `Status`, dynamic updates
- Impact: No interactive progress bars or dynamic content

**6. Content Renderers**  
- Missing: `Markdown`, `Syntax`, `Pretty`, `JSON`
- Impact: Cannot display rich content like code or documents

**7. Advanced Features**
- Missing: Themes, export capabilities, Jupyter integration
- Impact: Limited customization and integration options

### Current Coverage Assessment

**What we HAVE:** ~15% of Rich's features
- Basic styling and color system
- Simple text rendering
- ANSI escape generation  
- Terminal capability detection

**What we LACK:** ~85% of Rich's features
- All markup parsing
- All layout management
- All UI components
- All content rendering
- All live/dynamic features
- All advanced integrations

### Recommended Immediate Next Steps

1. **Implement Rich Markup Parser** (Phase 2) - Unblocks everything else
2. **Build Layout Foundation** (Phase 3) - Enables complex UIs  
3. **Create Panel Component** (Phase 4) - First visible Rich-like output
4. **Add Table Component** (Phase 4) - Most requested feature
5. **Implement Progress Bars** (Phase 5) - Popular interactive feature

### Development Strategy Options

**Option A: Feature Parity (6-8 months)**
- Implement all 8 phases for complete Rich compatibility
- Best for applications requiring full Rich feature set

**Option B: Core Subset (3-4 months)**  
- Focus on Phases 1-4 only (markup, layout, core components)
- Sufficient for most terminal UI applications

**Option C: MVP (1-2 months)**
- Implement only markup parser and 2-3 core components
- Good for proof-of-concept and early adopters
