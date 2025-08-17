//! Integration tests for Luxor library.
//!
//! These tests verify that all components work together correctly
//! and test the public API as it would be used by consumers.

use luxor::{
    Color, ColorSystem, Console, ConsoleOptions, Measurable, Renderable, Result, Segment, Style,
    Text,
};

/// Test basic text rendering through the full pipeline.
#[test]
fn test_end_to_end_text_rendering() -> Result<()> {
    let console = Console::new();
    let options = ConsoleOptions::new().with_color(true);

    // Create styled text
    let text =
        Text::new("Hello, World!").with_style(Style::new().bold().color(Color::rgb(255, 0, 0)));

    // Render through the full pipeline
    let segments = text.render(&console, &options)?;

    // Verify we got segments
    assert_eq!(segments.len(), 1);
    assert_eq!(segments[0].text(), "Hello, World!");
    assert_eq!(segments[0].style().bold, Some(true));
    assert_eq!(segments[0].style().color, Some(Color::rgb(255, 0, 0)));

    Ok(())
}

/// Test color downgrading through different color systems.
#[test]
fn test_color_system_compatibility() -> Result<()> {
    let console = Console::new();
    let true_color = Color::rgb(128, 64, 192);

    // Test rendering with different color systems
    for color_system in [
        ColorSystem::Standard,
        ColorSystem::EightBit,
        ColorSystem::TrueColor,
    ] {
        let options = ConsoleOptions::new().with_color_system(color_system);
        let text = Text::new("Test").with_style(Style::new().color(true_color));

        let segments = text.render(&console, &options)?;
        assert_eq!(segments.len(), 1);
        assert!(segments[0].style().color.is_some());

        // The color should still be the original color in the segment
        // (downgrading happens during ANSI generation, not in the segment itself)
        assert_eq!(segments[0].style().color.unwrap(), true_color);

        // Test that the downgrading logic itself works correctly
        let downgraded = true_color.downgrade(color_system);
        match color_system {
            ColorSystem::Standard => {
                assert!(matches!(downgraded, Color::Standard(_)));
            }
            ColorSystem::EightBit => {
                assert!(matches!(
                    downgraded,
                    Color::Standard(_) | Color::EightBit(_) | Color::TrueColor { .. }
                ));
            }
            ColorSystem::TrueColor => {
                assert_eq!(downgraded, true_color);
            }
        }
    }

    Ok(())
}

/// Test complex style composition and inheritance.
#[test]
fn test_complex_style_composition() -> Result<()> {
    let console = Console::new();
    let options = ConsoleOptions::new();

    // Create base style
    let base_style = Style::new()
        .bold()
        .color(Color::rgb(255, 0, 0))
        .background(Color::rgb(0, 0, 255));

    // Create overlay style
    let overlay_style = Style::new()
        .italic()
        .color(Color::rgb(0, 255, 0)) // Should override base color
        .underline();

    // Combine styles
    let combined_style = base_style.combine(overlay_style);

    // Create text with combined style
    let text = Text::new("Styled Text").with_style(combined_style);
    let segments = text.render(&console, &options)?;

    assert_eq!(segments.len(), 1);
    let style = segments[0].style();

    // Verify all attributes are correctly combined
    assert_eq!(style.bold, Some(true)); // From base
    assert_eq!(style.italic, Some(true)); // From overlay
    assert_eq!(style.underline, Some(true)); // From overlay
    assert_eq!(style.color, Some(Color::rgb(0, 255, 0))); // Overlay overrides base
    assert_eq!(style.background, Some(Color::rgb(0, 0, 255))); // From base

    Ok(())
}

/// Test text measurement with various Unicode characters.
#[test]
fn test_unicode_text_measurement() -> Result<()> {
    let console = Console::new();
    let options = ConsoleOptions::new();

    // Test different types of text
    let test_cases = [
        ("ASCII", "Hello", 5),
        ("Emoji", "👋🌍", 4),            // Each emoji is 2 width units
        ("Chinese", "你好", 4),          // Each Chinese character is 2 width units
        ("Mixed", "Hi 👋", 5),           // 2 + 1 + 2 = 5
        ("Zero Width", "a\u{200B}b", 2), // Zero-width space should not count
    ];

    for (name, text, expected_width) in test_cases {
        let text_obj = Text::new(text);
        let measurement = text_obj.measure(&console, &options)?;

        assert_eq!(
            measurement.minimum(),
            expected_width,
            "Failed for {}: expected {}, got {}",
            name,
            expected_width,
            measurement.minimum()
        );
        assert_eq!(measurement.maximum(), expected_width);
        assert!(measurement.is_fixed());
    }

    Ok(())
}

/// Test segment splitting with complex Unicode and styles.
#[test]
fn test_segment_splitting_edge_cases() -> Result<()> {
    // Test splitting at grapheme boundaries
    let style = Style::new().bold();
    let segment = Segment::new("👨‍👩‍👧‍👦 Family".to_string(), style.clone());

    // Split at width that should respect grapheme boundaries
    let (left, right) = segment.split_at_width(8);

    // The family emoji should not be split
    assert!(left.text().len() < "👨‍👩‍👧‍👦".len() || left.text() == "👨‍👩‍👧‍👦");
    assert_eq!(left.style(), &style);
    assert_eq!(right.style(), &style);

    Ok(())
}

/// Test error handling throughout the system.
#[test]
fn test_error_handling() {
    // Test invalid color parsing
    assert!(Color::from_hex("invalid").is_err());
    assert!(Color::from_hex("#GG0000").is_err());
    assert!(Color::from_hex("#FF").is_err()); // Wrong length

    // Test invalid style parsing
    assert!(Style::parse("bold on").is_err()); // Missing color after 'on'
    assert!(Style::parse("unknown_attribute").is_err());
}

/// Test console size detection and caching.
#[test]
fn test_console_size_management() {
    let mut console = Console::new();

    // Get initial size
    let (width1, height1) = console.size();
    assert!(width1 > 0);
    assert!(height1 > 0);

    // Update size cache
    let _ = console.update_size(); // May fail in test environment, that's ok

    // Size should still be reasonable
    let (width2, height2) = console.size();
    assert!(width2 > 0);
    assert!(height2 > 0);
}

/// Test console options and their effects.
#[test]
fn test_console_options_effects() -> Result<()> {
    let console = Console::new();

    // Test with color disabled
    let no_color_options = ConsoleOptions::new().with_color(false);
    assert_eq!(no_color_options.get_color_system(), ColorSystem::Standard);

    // Test with specific width
    let fixed_width_options = ConsoleOptions::new().with_max_width(80);
    assert_eq!(fixed_width_options.get_max_width(), 80);

    // Test rendering with disabled color
    let text = Text::new("Colored").with_style(Style::new().color(Color::rgb(255, 0, 0)));
    let segments = text.render(&console, &no_color_options)?;

    // Should still render but color system should be limited
    assert_eq!(segments.len(), 1);
    assert_eq!(segments[0].text(), "Colored");

    Ok(())
}

/// Test ANSI escape sequence generation and stripping.
#[test]
fn test_ansi_processing() {
    use luxor::ansi::{strip_ansi, text_width};

    // Test complex ANSI sequences
    let complex_ansi = "\x1b[1;31;42mBold Red on Green\x1b[0m Normal \x1b[3mItalic\x1b[23m";
    let stripped = strip_ansi(complex_ansi);
    assert_eq!(stripped, "Bold Red on Green Normal Italic");

    // Test width calculation ignoring ANSI
    let width = text_width(complex_ansi);
    assert_eq!(width, "Bold Red on Green Normal Italic".len());

    // Test with nested escapes (shouldn't occur in practice but should be handled)
    let nested = "\x1b[1m\x1b[31mNested\x1b[0m\x1b[0m";
    assert_eq!(strip_ansi(nested), "Nested");
}

/// Test performance characteristics are reasonable.
#[test]
fn test_performance_characteristics() -> Result<()> {
    let console = Console::new();
    let options = ConsoleOptions::new();

    // Test that large text can be processed efficiently
    let large_text = "Lorem ipsum ".repeat(1000);
    let text = Text::new(&large_text);

    // This should complete quickly
    let start = std::time::Instant::now();
    let _segments = text.render(&console, &options)?;
    let duration = start.elapsed();

    // Should complete in well under a second for 11,000 characters
    assert!(
        duration.as_millis() < 100,
        "Rendering took too long: {:?}",
        duration
    );

    // Test measurement performance
    let start = std::time::Instant::now();
    let _measurement = text.measure(&console, &options)?;
    let duration = start.elapsed();

    assert!(
        duration.as_millis() < 50,
        "Measurement took too long: {:?}",
        duration
    );

    Ok(())
}

/// Test thread safety by using components across threads.
#[test]
fn test_thread_safety() {
    use std::sync::Arc;
    use std::thread;

    let console = Arc::new(Console::new());
    let style = Arc::new(Style::new().bold().color(Color::rgb(255, 0, 0)));

    let handles: Vec<_> = (0..4)
        .map(|i| {
            let console = Arc::clone(&console);
            let style = Arc::clone(&style);
            thread::spawn(move || {
                let text = Text::new(&format!("Thread {}", i)).with_style((*style).clone());
                let options = ConsoleOptions::new();
                text.render(&*console, &options).unwrap()
            })
        })
        .collect();

    // Wait for all threads to complete
    for handle in handles {
        let segments = handle.join().unwrap();
        assert_eq!(segments.len(), 1);
        assert!(segments[0].text().starts_with("Thread "));
    }
}

/// Test that the library handles edge cases gracefully.
#[test]
fn test_edge_cases() -> Result<()> {
    let console = Console::new();
    let options = ConsoleOptions::new();

    // Test empty text
    let empty_text = Text::new("");
    let segments = empty_text.render(&console, &options)?;
    assert_eq!(segments.len(), 1);
    assert_eq!(segments[0].text(), "");

    // Test very long lines
    let long_line = "a".repeat(10000);
    let long_text = Text::new(&long_line);
    let segments = long_text.render(&console, &options)?;
    assert_eq!(segments.len(), 1);
    assert_eq!(segments[0].text().len(), 10000);

    // Test all possible style combinations
    let all_styles = Style::new()
        .bold()
        .italic()
        .underline()
        .strikethrough()
        .dim()
        .reverse()
        .blink()
        .hidden()
        .color(Color::rgb(255, 128, 64))
        .background(Color::rgb(64, 128, 255));

    let styled_text = Text::new("All styles").with_style(all_styles);
    let segments = styled_text.render(&console, &options)?;
    assert_eq!(segments.len(), 1);

    // Verify all style attributes are preserved
    let style = segments[0].style();
    assert_eq!(style.bold, Some(true));
    assert_eq!(style.italic, Some(true));
    assert_eq!(style.underline, Some(true));
    assert_eq!(style.strikethrough, Some(true));
    assert_eq!(style.dim, Some(true));
    assert_eq!(style.reverse, Some(true));
    assert_eq!(style.blink, Some(true));
    assert_eq!(style.hidden, Some(true));

    Ok(())
}
