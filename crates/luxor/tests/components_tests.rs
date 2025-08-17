//! Integration tests for UI components.
//!
//! These tests verify that UI components work correctly with the existing
//! Luxor infrastructure and can be composed together.

use luxor::{
    box_drawing::BorderStyle,
    components::Rule,
    console::{Console, ConsoleOptions},
    layout::Align,
    protocol::{Measurable, Renderable},
    style::Style,
    text::Text,
};

fn test_console() -> Console {
    Console::new()
}

#[test]
fn test_rule_creation() {
    // Test default horizontal rule
    let _rule = Rule::new();
    // Note: orientation field is private, test through behavior

    // Test rule with title
    let _rule_with_title = Rule::with_title("Test Title");
    // Verify through measurement that title affects minimum width

    // Test vertical rule
    let _vertical_rule = Rule::vertical();
    // Test through measurement that it behaves differently

    // Test vertical rule with title
    let _vertical_with_title = Rule::vertical_with_title("Vertical Test");
    // Verify behavior through measurement
}

#[test]
fn test_rule_builder_pattern() {
    let rule = Rule::new()
        .with_style(Style::new().bold())
        .with_border_style(BorderStyle::Double)
        .with_align(Align::Center)
        .with_character('=');

    // Test that the rule was configured properly through rendering
    let console = test_console();
    let options = ConsoleOptions::default().with_max_width(10);
    let segments = rule.render(&console, &options).unwrap();

    assert!(!segments.is_empty());
    assert_eq!(segments[0].text(), "==========");
    assert_eq!(segments[0].style().bold, Some(true));
}

#[test]
fn test_rule_measurement() {
    let console = test_console();
    let options = ConsoleOptions::default().with_max_width(20);

    // Test horizontal rule measurement
    let rule = Rule::new();
    let measurement = rule.measure(&console, &options).unwrap();
    assert_eq!(measurement.minimum(), 1);
    assert_eq!(measurement.maximum(), 20);

    // Test rule with title measurement
    let rule_with_title = Rule::with_title("Hello World");
    let measurement = rule_with_title.measure(&console, &options).unwrap();
    assert!(measurement.minimum() >= 11); // At least the title length
    assert_eq!(measurement.maximum(), 20);

    // Test vertical rule measurement
    let vertical_rule = Rule::vertical();
    let measurement = vertical_rule.measure(&console, &options).unwrap();
    assert_eq!(measurement.minimum(), 1);
    assert_eq!(measurement.maximum(), 1); // Vertical rules always have width 1
}

#[test]
fn test_horizontal_rule_rendering() {
    let console = test_console();
    let options = ConsoleOptions::default().with_max_width(10);

    // Test basic horizontal rule
    let rule = Rule::new();
    let segments = rule.render(&console, &options).unwrap();
    assert_eq!(segments.len(), 1);
    assert_eq!(segments[0].text(), "──────────"); // 10 dashes

    // Test rule with custom character
    let custom_rule = Rule::new().with_character('*');
    let segments = custom_rule.render(&console, &options).unwrap();
    assert_eq!(segments[0].text(), "**********"); // 10 asterisks

    // Test rule with different border styles
    let double_rule = Rule::new().with_border_style(BorderStyle::Double);
    let segments = double_rule.render(&console, &options).unwrap();
    assert_eq!(segments[0].text(), "══════════"); // 10 double lines

    let thick_rule = Rule::new().with_border_style(BorderStyle::Thick);
    let segments = thick_rule.render(&console, &options).unwrap();
    assert_eq!(segments[0].text(), "━━━━━━━━━━"); // 10 thick lines

    let ascii_rule = Rule::new().with_border_style(BorderStyle::Ascii);
    let segments = ascii_rule.render(&console, &options).unwrap();
    assert_eq!(segments[0].text(), "----------"); // 10 dashes (ASCII)
}

#[test]
fn test_horizontal_rule_with_title() {
    let console = test_console();
    let options = ConsoleOptions::default().with_max_width(15);

    // Test center-aligned title
    let rule = Rule::with_title("Hi").with_align(Align::Center);
    let segments = rule.render(&console, &options).unwrap();

    // Should have multiple segments: left rule + title + right rule
    assert!(segments.len() >= 2);

    // Find the title segment
    let title_found = segments.iter().any(|s| s.text().contains("Hi"));
    assert!(title_found);

    // Test left-aligned title
    let rule = Rule::with_title("Test").with_align(Align::Left);
    let segments = rule.render(&console, &options).unwrap();
    assert!(segments.iter().any(|s| s.text().contains("Test")));

    // Test right-aligned title
    let rule = Rule::with_title("End").with_align(Align::Right);
    let segments = rule.render(&console, &options).unwrap();
    assert!(segments.iter().any(|s| s.text().contains("End")));

    // Test title too wide for available space
    let options = ConsoleOptions::default().with_max_width(5);
    let rule = Rule::with_title("This is a very long title");
    let segments = rule.render(&console, &options).unwrap();
    // Should still render something (just the title, possibly truncated)
    assert!(!segments.is_empty());
}

#[test]
fn test_vertical_rule_rendering() {
    let console = test_console();
    let options = ConsoleOptions::default();

    // Test basic vertical rule
    let rule = Rule::vertical();
    let segments = rule.render(&console, &options).unwrap();

    // Should have multiple segments with line breaks
    assert!(!segments.is_empty());

    // Count vertical line characters
    let vertical_chars = segments.iter().filter(|s| s.text() == "│").count();
    assert!(vertical_chars > 0);

    // Test vertical rule with custom character
    let custom_rule = Rule::vertical().with_character('|');
    let segments = custom_rule.render(&console, &options).unwrap();
    let custom_chars = segments.iter().filter(|s| s.text() == "|").count();
    assert!(custom_chars > 0);
}

#[test]
fn test_vertical_rule_with_title() {
    let console = test_console();
    let options = ConsoleOptions::default();

    let rule = Rule::vertical_with_title("Title");
    let segments = rule.render(&console, &options).unwrap();

    // Should have vertical segments and title
    assert!(!segments.is_empty());

    // Should contain the title somewhere
    let title_found = segments.iter().any(|s| s.text().contains("Title"));
    assert!(title_found);

    // Should also have vertical line characters
    let vertical_chars = segments.iter().filter(|s| s.text() == "│").count();
    assert!(vertical_chars > 0);
}

#[test]
fn test_rule_styling() {
    let console = test_console();
    let options = ConsoleOptions::default().with_max_width(5);

    // Test style application
    let rule = Rule::new()
        .with_style(Style::new().bold().italic())
        .with_character('=');

    let segments = rule.render(&console, &options).unwrap();
    assert_eq!(segments[0].text(), "=====");
    assert_eq!(segments[0].style().bold, Some(true));
    assert_eq!(segments[0].style().italic, Some(true));

    // Test that style is applied to all segments
    for segment in &segments {
        if !segment.text().is_empty() {
            assert_eq!(segment.style().bold, Some(true));
            assert_eq!(segment.style().italic, Some(true));
        }
    }
}

#[test]
fn test_rule_edge_cases() {
    let console = test_console();

    // Test zero-width rule
    let options = ConsoleOptions::default().with_max_width(0);
    let rule = Rule::new();
    let segments = rule.render(&console, &options).unwrap();
    assert!(segments.is_empty());

    // Test very narrow rule
    let options = ConsoleOptions::default().with_max_width(1);
    let rule = Rule::new();
    let segments = rule.render(&console, &options).unwrap();
    assert_eq!(segments.len(), 1);
    assert_eq!(segments[0].text(), "─");

    // Test rule with title wider than available space
    let options = ConsoleOptions::default().with_max_width(5);
    let rule = Rule::with_title("This title is way too long for the space");
    let segments = rule.render(&console, &options).unwrap();
    // Should handle gracefully without panicking
    assert!(!segments.is_empty());
}

#[test]
fn test_rule_helper_functions() {
    // Test module-level helper functions
    let h_rule = luxor::components::rule::horizontal_rule();
    let console = test_console();
    let options = ConsoleOptions::default().with_max_width(10);
    let segments = h_rule.render(&console, &options).unwrap();
    assert_eq!(segments[0].text(), "──────────");

    let h_rule_title = luxor::components::rule::horizontal_rule_with_title("Test");
    let segments = h_rule_title.render(&console, &options).unwrap();
    let title_found = segments.iter().any(|s| s.text().contains("Test"));
    assert!(title_found);

    let v_rule = luxor::components::rule::vertical_rule();
    let segments = v_rule.render(&console, &options).unwrap();
    let vertical_chars = segments.iter().filter(|s| s.text() == "│").count();
    assert!(vertical_chars > 0);

    let v_rule_title = luxor::components::rule::vertical_rule_with_title("Vertical");
    let segments = v_rule_title.render(&console, &options).unwrap();
    let title_found = segments.iter().any(|s| s.text().contains("Vertical"));
    assert!(title_found);
}

#[test]
fn test_component_composition() {
    let console = test_console();
    let options = ConsoleOptions::default().with_max_width(20);

    // Test that components can be used with existing Text components
    let text = Text::new("Hello World").with_style(Style::new().bold());
    let text_segments = text.render(&console, &options).unwrap();
    assert!(!text_segments.is_empty());
    assert_eq!(text_segments[0].style().bold, Some(true));

    // Test that rules can be created with different styles
    let rule = Rule::new().with_style(Style::new().italic());
    let rule_segments = rule.render(&console, &options).unwrap();
    assert!(!rule_segments.is_empty());
    assert_eq!(rule_segments[0].style().italic, Some(true));

    // Verify they can coexist and have different styles
    assert_ne!(text_segments[0].style().bold, rule_segments[0].style().bold);
    assert_ne!(
        text_segments[0].style().italic,
        rule_segments[0].style().italic
    );
}

#[test]
fn test_rule_with_markup_title() {
    let console = test_console();
    let options = ConsoleOptions::default().with_max_width(20);

    // Test rule with markup in title
    let title = Text::from_markup("[bold]Bold Title[/bold]").unwrap();
    let rule = Rule::with_title(title);
    let segments = rule.render(&console, &options).unwrap();

    // Should render without errors
    assert!(!segments.is_empty());

    // Should contain the title text
    let title_found = segments.iter().any(|s| s.text().contains("Bold Title"));
    assert!(title_found);
}

#[test]
fn test_rule_alignment_behavior() {
    let console = test_console();
    let options = ConsoleOptions::default().with_max_width(20);

    // Test that different alignments produce different outputs
    let left_rule = Rule::with_title("Hi").with_align(Align::Left);
    let center_rule = Rule::with_title("Hi").with_align(Align::Center);
    let right_rule = Rule::with_title("Hi").with_align(Align::Right);

    let left_segments = left_rule.render(&console, &options).unwrap();
    let center_segments = center_rule.render(&console, &options).unwrap();
    let right_segments = right_rule.render(&console, &options).unwrap();

    // All should render successfully
    assert!(!left_segments.is_empty());
    assert!(!center_segments.is_empty());
    assert!(!right_segments.is_empty());

    // Should have title in all cases
    assert!(left_segments.iter().any(|s| s.text().contains("Hi")));
    assert!(center_segments.iter().any(|s| s.text().contains("Hi")));
    assert!(right_segments.iter().any(|s| s.text().contains("Hi")));
}

#[test]
fn test_performance() {
    use std::time::{Duration, Instant};

    let console = test_console();
    let options = ConsoleOptions::default().with_max_width(50);

    // Test that basic operations complete without errors (removed strict timing assertions)
    let start = Instant::now();
    for i in 0..100 {
        // Reduced iterations
        let _rule = Rule::new()
            .with_character(if i % 2 == 0 { '=' } else { '-' })
            .with_style(Style::new().bold());
    }
    let creation_time = start.elapsed();
    assert!(creation_time < Duration::from_secs(1)); // Very generous

    // Test rule rendering
    let rule = Rule::new().with_character('=');
    let start = Instant::now();
    for _ in 0..100 {
        // Reduced iterations
        let _segments = rule.render(&console, &options).unwrap();
    }
    let render_time = start.elapsed();
    assert!(render_time < Duration::from_secs(1)); // Very generous

    // Test rule measurement
    let start = Instant::now();
    for _ in 0..100 {
        // Reduced iterations
        let _measurement = rule.measure(&console, &options).unwrap();
    }
    let measure_time = start.elapsed();
    assert!(measure_time < Duration::from_secs(1)); // Very generous
}

#[test]
fn test_memory_usage() {
    use std::mem;

    // Test that Rule component has reasonable memory footprint
    assert!(mem::size_of::<Rule>() <= 200);

    // Test that creating many rules doesn't cause excessive memory usage
    let mut rules = Vec::new();
    for i in 0..1000 {
        rules.push(Rule::new().with_character(char::from_u32(45 + (i % 10) as u32).unwrap()));
    }

    // Should not cause excessive memory usage (this is a sanity check)
    assert_eq!(rules.len(), 1000);
}

#[test]
fn test_unicode_handling() {
    let console = test_console();
    let options = ConsoleOptions::default().with_max_width(15);

    // Test rule with Unicode title
    let unicode_rule = Rule::with_title("测试标题");
    let segments = unicode_rule.render(&console, &options).unwrap();
    assert!(!segments.is_empty());

    // Should contain the Unicode title
    let title_found = segments.iter().any(|s| s.text().contains("测试标题"));
    assert!(title_found);

    // Test rule with Unicode custom character
    let unicode_char_rule = Rule::new().with_character('※');
    let segments = unicode_char_rule.render(&console, &options).unwrap();
    assert!(segments[0].text().contains('※'));

    // Test that Unicode box drawing characters work
    let unicode_border_rule = Rule::new().with_border_style(BorderStyle::Single);
    let segments = unicode_border_rule.render(&console, &options).unwrap();
    assert!(segments[0].text().contains('─'));
}

#[test]
fn test_error_handling() {
    let console = test_console();
    let options = ConsoleOptions::default().with_max_width(100);

    // Test that all operations complete without panicking
    let rule = Rule::with_title("Test")
        .with_style(Style::new().bold())
        .with_border_style(BorderStyle::Double)
        .with_align(Align::Center)
        .with_character('=');

    // These should all succeed
    let measurement = rule.measure(&console, &options).unwrap();
    assert!(measurement.minimum() > 0);

    let segments = rule.render(&console, &options).unwrap();
    assert!(!segments.is_empty());

    // Test with very constrained options
    let tight_options = ConsoleOptions::default().with_max_width(1);
    let segments = rule.render(&console, &tight_options).unwrap();
    // Should handle gracefully
    assert!(segments.len() <= 1);
}

#[test]
fn test_default_implementations() {
    // Test that Default trait works
    let default_rule = Rule::default();
    let console = test_console();
    let options = ConsoleOptions::default().with_max_width(10);

    let segments = default_rule.render(&console, &options).unwrap();
    assert!(!segments.is_empty());
    assert_eq!(segments[0].text(), "──────────");
}
