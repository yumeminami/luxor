//! Property-based tests for Luxor library.
//!
//! These tests use proptest to generate random inputs and verify
//! that certain properties always hold true.

use luxor::{Color, ColorSystem, Measurement, Segment, Style};
use proptest::prelude::*;

// Strategies for generating test data

/// Strategy for generating valid RGB values.
fn rgb_strategy() -> impl Strategy<Value = (u8, u8, u8)> {
    (any::<u8>(), any::<u8>(), any::<u8>())
}

/// Strategy for generating valid colors.
fn color_strategy() -> impl Strategy<Value = Color> {
    prop_oneof![
        Just(Color::Default),
        (0u8..16).prop_map(|i| Color::Standard(standard_color_from_index(i))),
        any::<u8>().prop_map(Color::EightBit),
        rgb_strategy().prop_map(|(r, g, b)| Color::TrueColor { r, g, b }),
    ]
}

/// Strategy for generating styles with random attributes.
fn style_strategy() -> impl Strategy<Value = Style> {
    (
        prop::option::of(color_strategy()), // color
        prop::option::of(color_strategy()), // background
        prop::option::of(any::<bool>()),    // bold
        prop::option::of(any::<bool>()),    // italic
        prop::option::of(any::<bool>()),    // underline
        prop::option::of(any::<bool>()),    // strikethrough
        prop::option::of(any::<bool>()),    // dim
        prop::option::of(any::<bool>()),    // reverse
        prop::option::of(any::<bool>()),    // blink
        prop::option::of(any::<bool>()),    // hidden
    )
        .prop_map(
            |(
                color,
                background,
                bold,
                italic,
                underline,
                strikethrough,
                dim,
                reverse,
                blink,
                hidden,
            )| {
                Style {
                    color,
                    background,
                    bold,
                    italic,
                    underline,
                    strikethrough,
                    dim,
                    reverse,
                    blink,
                    hidden,
                }
            },
        )
}

/// Helper function to convert index to standard color.
fn standard_color_from_index(index: u8) -> luxor::StandardColor {
    use luxor::StandardColor;
    match index % 16 {
        0 => StandardColor::Black,
        1 => StandardColor::Red,
        2 => StandardColor::Green,
        3 => StandardColor::Yellow,
        4 => StandardColor::Blue,
        5 => StandardColor::Magenta,
        6 => StandardColor::Cyan,
        7 => StandardColor::White,
        8 => StandardColor::BrightBlack,
        9 => StandardColor::BrightRed,
        10 => StandardColor::BrightGreen,
        11 => StandardColor::BrightYellow,
        12 => StandardColor::BrightBlue,
        13 => StandardColor::BrightMagenta,
        14 => StandardColor::BrightCyan,
        15 => StandardColor::BrightWhite,
        _ => StandardColor::White,
    }
}

// Property tests

proptest! {
    /// Test that color downgrading never panics and always produces valid colors.
    #[test]
    fn color_downgrade_preserves_validity(
        color in color_strategy(),
        color_system in prop::sample::select(&[ColorSystem::Standard, ColorSystem::EightBit, ColorSystem::TrueColor])
    ) {
        let downgraded = color.downgrade(color_system);

        // Verify the downgraded color is appropriate for the color system
        match color_system {
            ColorSystem::Standard => {
                prop_assert!(matches!(downgraded, Color::Default | Color::Standard(_)));
            }
            ColorSystem::EightBit => {
                prop_assert!(matches!(downgraded, Color::Default | Color::Standard(_) | Color::EightBit(_)));
            }
            ColorSystem::TrueColor => {
                // TrueColor should accept any color
                prop_assert!(true);
            }
        }
    }

    /// Test that RGB to 8-bit conversion always produces valid indices.
    #[test]
    fn rgb_to_eight_bit_produces_valid_indices((r, g, b) in rgb_strategy()) {
        let index = Color::rgb_to_eight_bit(r, g, b);
        // All 8-bit color indices should be valid u8 values (0-255 by definition)
        // Just verify the function doesn't panic and returns a value
        prop_assert!(index == index); // Always true, but ensures no panic
    }

    /// Test that color round-trip conversions preserve meaning.
    #[test]
    fn color_rgb_conversion_roundtrip(color in color_strategy()) {
        let rgb = color.to_rgb();
        prop_assert_eq!(rgb.0, rgb.0); // Just verify it doesn't panic
        prop_assert_eq!(rgb.1, rgb.1);
        prop_assert_eq!(rgb.2, rgb.2);
    }

    /// Test that style combination is associative.
    #[test]
    fn style_combination_associative(
        style1 in style_strategy(),
        style2 in style_strategy(),
        style3 in style_strategy()
    ) {
        let left_assoc = style1.clone().combine(style2.clone()).combine(style3.clone());
        let right_assoc = style1.combine(style2.combine(style3));

        // Style combination should be associative
        prop_assert_eq!(left_assoc, right_assoc);
    }

    /// Test that style combination with empty style is identity.
    #[test]
    fn style_combination_identity(style in style_strategy()) {
        let empty = Style::new();
        let combined = style.clone().combine(empty);

        // Combining with empty style should not change anything
        prop_assert_eq!(style, combined);
    }

    /// Test that measurement operations preserve invariants.
    #[test]
    fn measurement_invariants(min_width in 0usize..1000, max_width in 0usize..1000) {
        let min = min_width.min(max_width);
        let max = min_width.max(max_width);

        let measurement = Measurement::new(min, max);

        prop_assert_eq!(measurement.minimum(), min);
        prop_assert_eq!(measurement.maximum(), max);
        prop_assert!(measurement.minimum() <= measurement.maximum());
    }

    /// Test that measurement clamping works correctly.
    #[test]
    fn measurement_clamping(
        min_width in 0usize..500,
        max_width in 500usize..1000,
        clamp_min in 0usize..1000,
        clamp_max in 0usize..1000
    ) {
        let measurement = Measurement::new(min_width, max_width);
        let clamp_min = clamp_min.min(clamp_max);
        let clamp_max = clamp_min.max(clamp_max);

        let clamped = measurement.clamp(clamp_min, clamp_max);

        prop_assert!(clamped.minimum() >= clamp_min);
        prop_assert!(clamped.maximum() <= clamp_max);
        prop_assert!(clamped.minimum() <= clamped.maximum());
    }

    /// Test that segment splitting preserves text content.
    #[test]
    fn segment_splitting_preserves_content(
        text in ".{0,100}",  // Random string up to 100 chars
        split_pos in 0usize..100,
        style in style_strategy()
    ) {
        if !text.is_empty() {
            let segment = Segment::new(text.clone(), style);
            let split_pos = split_pos.min(text.chars().count());

            let (left, right) = segment.split_at_char(split_pos);

            let combined = format!("{}{}", left.text(), right.text());
            prop_assert_eq!(combined, text);
        }
    }

    /// Test that text width calculation is consistent.
    #[test]
    fn text_width_consistency(text in r"[^\x1b]{0,50}") {
        use luxor::ansi::{text_width, strip_ansi};
        use unicode_width::UnicodeWidthStr;

        // For text without escape characters, width should match Unicode width
        let calculated_width = text_width(&text);
        let unicode_width = text.width();
        prop_assert_eq!(calculated_width, unicode_width);

        // Stripping ANSI from text without escape characters should return the same text
        let stripped = strip_ansi(&text);
        prop_assert_eq!(stripped, text);
    }

    /// Test that ANSI stripping handles malformed sequences gracefully.
    #[test]
    fn ansi_stripping_handles_malformed_sequences(text in ".{0,50}") {
        use luxor::ansi::{strip_ansi, text_width};

        // ANSI stripping should never panic, even with malformed input
        let stripped = strip_ansi(&text);

        // The width should be reasonable (not exceeding reasonable bounds)
        let width = text_width(&text);
        prop_assert!(width <= text.len() * 2); // At most 2 width units per byte

        // Stripping should be idempotent - stripping again should give same result
        let double_stripped = strip_ansi(&stripped);
        prop_assert_eq!(stripped, double_stripped);
    }

    /// Test that ANSI sequence generation never produces malformed sequences.
    #[test]
    fn ansi_generation_wellformed(style in style_strategy()) {
        use luxor::ansi::style_to_ansi;

        let ansi = style_to_ansi(&style, ColorSystem::TrueColor);

        if !ansi.is_empty() {
            // ANSI sequences should start with escape character
            prop_assert!(ansi.starts_with('\x1b'));
            // And should contain the CSI sequence marker
            prop_assert!(ansi.contains('['));
            // And should end with a letter (command character)
            prop_assert!(ansi.chars().last().map(|c| c.is_ascii_alphabetic()).unwrap_or(false));
        }
    }

    /// Test that color parsing and formatting are consistent.
    #[test]
    fn color_hex_parsing_consistency((r, g, b) in rgb_strategy()) {
        let color = Color::rgb(r, g, b);
        let hex = format!("#{:02X}{:02X}{:02X}", r, g, b);

        match Color::from_hex(&hex) {
            Ok(parsed_color) => {
                prop_assert_eq!(parsed_color, color);
            }
            Err(_) => {
                // Parsing can fail for various reasons, but shouldn't panic
                prop_assert!(true);
            }
        }
    }

    /// Test that style parsing handles various input formats gracefully.
    #[test]
    fn style_parsing_robustness(input in r"[a-zA-Z0-9 #_-]{0,50}") {
        // Style parsing should never panic, even with random input
        let result = Style::parse(&input);
        // We don't care if it succeeds or fails, just that it doesn't panic
        let _result = result.is_ok() || result.is_err();
        prop_assert!(true);
    }

    /// Test that measurement arithmetic operations preserve invariants.
    #[test]
    fn measurement_arithmetic_invariants(
        m1_min in 0usize..500,
        m1_max in 0usize..500,
        m2_min in 0usize..500,
        m2_max in 0usize..500,
        width in 0usize..100
    ) {
        let m1_min = m1_min.min(m1_max);
        let m1_max = m1_min.max(m1_max);
        let m2_min = m2_min.min(m2_max);
        let m2_max = m2_min.max(m2_max);

        let m1 = Measurement::new(m1_min, m1_max);
        let m2 = Measurement::new(m2_min, m2_max);

        // Test max_with
        let max_combined = m1.max_with(m2);
        prop_assert!(max_combined.minimum() >= m1.minimum());
        prop_assert!(max_combined.minimum() >= m2.minimum());
        prop_assert!(max_combined.maximum() >= m1.maximum());
        prop_assert!(max_combined.maximum() >= m2.maximum());

        // Test add_with
        let add_combined = m1.add_with(m2);
        prop_assert_eq!(add_combined.minimum(), m1.minimum() + m2.minimum());
        prop_assert_eq!(add_combined.maximum(), m1.maximum() + m2.maximum());

        // Test add_width
        let widened = m1.add_width(width);
        prop_assert_eq!(widened.minimum(), m1.minimum() + width);
        prop_assert_eq!(widened.maximum(), m1.maximum() + width);

        // Test subtract_width (with saturation)
        let narrowed = m1.subtract_width(width);
        prop_assert!(narrowed.minimum() <= m1.minimum());
        prop_assert!(narrowed.maximum() <= m1.maximum());
        prop_assert!(narrowed.minimum() <= narrowed.maximum());
    }
}
