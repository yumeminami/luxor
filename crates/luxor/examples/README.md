# Luxor Examples

This directory contains various demonstrations of Luxor features, inspired by the Python Rich library examples. To run them, make sure Luxor is built, then run the example you want with `cargo run --example <name>` from the `crates/luxor` directory.

## Available Examples

### 1. `showcase`

**Command:** `cargo run --example showcase`

A comprehensive demonstration of all current Luxor capabilities including:

- Basic text styling with fluent API
- Rich markup parsing with BBCode syntax
- Style ranges for character-level styling
- Color system demonstrations
- Performance testing

### 2. `rainbow`

**Command:** `cargo run --example rainbow`

Creates rainbow-colored text using different approaches:

- Character-by-character coloring with predefined colors
- Smooth gradient transitions from red to blue
- HSV hue cycling through the full color spectrum
- HSV to RGB color conversion demonstration

### 3. `align`

**Command:** `cargo run --example align`

Demonstrates text alignment techniques:

- Manual alignment with padding using Rust's format specifiers
- Styled text with different alignments
- Centered titles and section headers
- Decorative borders and formatting

### 4. `highlighter`

**Command:** `cargo run --example highlighter`

Shows how to create text highlighters for automatic styling:

- Email address highlighting with regex-like pattern matching
- URL highlighting for http/https links
- Rust keyword highlighting with color coding
- Number highlighting including scientific notation
- Unicode-aware character position handling

### 5. `overflow`

**Command:** `cargo run --example overflow`

Demonstrates text overflow handling when text exceeds available width:

- Crop overflow (cut off at boundary)
- Ellipsis overflow (show "..." at end)
- Wrap overflow (break into multiple lines)
- Styled text with various overflow methods

### 6. `styles`

**Command:** `cargo run --example styles`

A comprehensive showcase of styling capabilities:

- Basic text attributes (bold, italic, underline, etc.)
- Color variations (standard colors and RGB gradients)
- Style combinations and layering
- Background colors with foreground text
- Style composition and inheritance
- Advanced color palette demonstrations

### 7. `markup`

**Command:** `cargo run --example markup`

Demonstrates the rich markup parsing system:

- Basic markup tags (`[bold]`, `[italic]`, etc.)
- Nested markup combinations
- Color markup with named colors
- Complex markup with multiple attributes
- Error handling for malformed markup
- Edge cases and validation

## Running All Examples

To run all examples in sequence, you can use:

```bash
cd crates/luxor
for example in showcase rainbow align highlighter overflow styles markup; do
    echo "=== Running $example example ==="
    cargo run --example $example
    echo
done
```

## Example Features by Luxor Capability

### Current Luxor Features Demonstrated:

- ✅ **Rich Markup Parsing**: `markup`, `showcase`
- ✅ **Style Composition**: `styles`, `showcase`
- ✅ **Color Systems**: `rainbow`, `styles`, `showcase`
- ✅ **Text Processing**: `highlighter`, `overflow`
- ✅ **Unicode Support**: All examples
- ✅ **Error Handling**: `markup`, `showcase`

### Planned Features (Not Yet Implemented):

- ⏳ **Layout System**: Panel borders, table layouts
- ⏳ **Text Wrapping**: Intelligent word wrapping
- ⏳ **Progress Bars**: Animated progress indicators
- ⏳ **Live Updates**: Dynamic content refresh
- ⏳ **Tables**: Structured data display
- ⏳ **Trees**: Hierarchical data visualization

## Implementation Notes

These examples are designed to:

1. **Mirror Rich's API patterns** while using Rust idioms
2. **Demonstrate real-world usage** of Luxor features
3. **Serve as API documentation** through working code
4. **Test edge cases** and error conditions
5. **Show performance characteristics** where relevant

Each example includes:

- Clear documentation comments
- Step-by-step demonstrations
- Error handling examples
- Performance considerations
- Unicode-aware text processing

## Rich Compatibility

While these examples are inspired by Python Rich, they use Rust-idiomatic patterns:

- `Result<T, Error>` instead of exceptions
- Ownership and borrowing for memory safety
- Zero-cost abstractions for performance
- Strong typing with comprehensive trait system

The goal is to provide Rich's user-friendly experience with Rust's performance and safety guarantees.
