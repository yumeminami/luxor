//! Integration tests for the box_drawing module.
//!
//! These tests verify that the box drawing system works correctly with
//! border styles, configurations, and rendering.

use luxor::{
    box_drawing::{
        BorderSides, BorderStyle, BoxChars, BoxConfig, BoxSegment, BoxSegmentType, draw_box,
        horizontal_line, vertical_line,
    },
    style::Style,
};

#[test]
fn test_box_chars() {
    // Test single border characters
    let single = BoxChars::single();
    assert_eq!(single.top_left, '┌');
    assert_eq!(single.top_right, '┐');
    assert_eq!(single.bottom_left, '└');
    assert_eq!(single.bottom_right, '┘');
    assert_eq!(single.horizontal, '─');
    assert_eq!(single.vertical, '│');
    assert_eq!(single.top_tee, '┬');
    assert_eq!(single.bottom_tee, '┴');
    assert_eq!(single.left_tee, '├');
    assert_eq!(single.right_tee, '┤');
    assert_eq!(single.cross, '┼');

    // Test double border characters
    let double = BoxChars::double();
    assert_eq!(double.top_left, '╔');
    assert_eq!(double.top_right, '╗');
    assert_eq!(double.bottom_left, '╚');
    assert_eq!(double.bottom_right, '╝');
    assert_eq!(double.horizontal, '═');
    assert_eq!(double.vertical, '║');

    // Test rounded border characters
    let rounded = BoxChars::rounded();
    assert_eq!(rounded.top_left, '╭');
    assert_eq!(rounded.top_right, '╮');
    assert_eq!(rounded.bottom_left, '╰');
    assert_eq!(rounded.bottom_right, '╯');
    assert_eq!(rounded.horizontal, '─'); // Uses single line
    assert_eq!(rounded.vertical, '│');

    // Test thick border characters
    let thick = BoxChars::thick();
    assert_eq!(thick.top_left, '┏');
    assert_eq!(thick.top_right, '┓');
    assert_eq!(thick.bottom_left, '┗');
    assert_eq!(thick.bottom_right, '┛');
    assert_eq!(thick.horizontal, '━');
    assert_eq!(thick.vertical, '┃');

    // Test ASCII border characters
    let ascii = BoxChars::ascii();
    assert_eq!(ascii.top_left, '+');
    assert_eq!(ascii.top_right, '+');
    assert_eq!(ascii.bottom_left, '+');
    assert_eq!(ascii.bottom_right, '+');
    assert_eq!(ascii.horizontal, '-');
    assert_eq!(ascii.vertical, '|');
}

#[test]
fn test_border_style() {
    // Test visibility
    assert!(BorderStyle::Single.is_visible());
    assert!(BorderStyle::Double.is_visible());
    assert!(BorderStyle::Rounded.is_visible());
    assert!(BorderStyle::Thick.is_visible());
    assert!(BorderStyle::Ascii.is_visible());
    assert!(!BorderStyle::None.is_visible());

    // Test thickness
    assert_eq!(BorderStyle::Single.thickness(), 1);
    assert_eq!(BorderStyle::Double.thickness(), 1);
    assert_eq!(BorderStyle::Rounded.thickness(), 1);
    assert_eq!(BorderStyle::Thick.thickness(), 1);
    assert_eq!(BorderStyle::Ascii.thickness(), 1);
    assert_eq!(BorderStyle::None.thickness(), 0);

    // Test character retrieval
    assert!(BorderStyle::Single.chars().is_some());
    assert!(BorderStyle::Double.chars().is_some());
    assert!(BorderStyle::Rounded.chars().is_some());
    assert!(BorderStyle::Thick.chars().is_some());
    assert!(BorderStyle::Ascii.chars().is_some());
    assert!(BorderStyle::None.chars().is_none());
}

#[test]
fn test_border_sides() {
    // Test all sides
    let all = BorderSides::all();
    assert!(all.top);
    assert!(all.right);
    assert!(all.bottom);
    assert!(all.left);
    assert!(all.has_border());
    assert_eq!(all.horizontal_width(BorderStyle::Single), 2);
    assert_eq!(all.vertical_height(BorderStyle::Single), 2);

    // Test no sides
    let none = BorderSides::none();
    assert!(!none.top);
    assert!(!none.right);
    assert!(!none.bottom);
    assert!(!none.left);
    assert!(!none.has_border());
    assert_eq!(none.horizontal_width(BorderStyle::Single), 0);
    assert_eq!(none.vertical_height(BorderStyle::Single), 0);

    // Test horizontal only
    let horizontal = BorderSides::horizontal();
    assert!(horizontal.top);
    assert!(!horizontal.right);
    assert!(horizontal.bottom);
    assert!(!horizontal.left);
    assert!(horizontal.has_border());
    assert_eq!(horizontal.horizontal_width(BorderStyle::Single), 0);
    assert_eq!(horizontal.vertical_height(BorderStyle::Single), 2);

    // Test vertical only
    let vertical = BorderSides::vertical();
    assert!(!vertical.top);
    assert!(vertical.right);
    assert!(!vertical.bottom);
    assert!(vertical.left);
    assert!(vertical.has_border());
    assert_eq!(vertical.horizontal_width(BorderStyle::Single), 2);
    assert_eq!(vertical.vertical_height(BorderStyle::Single), 0);

    // Test custom sides
    let custom = BorderSides::new(true, false, true, false);
    assert!(custom.top);
    assert!(!custom.right);
    assert!(custom.bottom);
    assert!(!custom.left);
    assert!(custom.has_border());
}

#[test]
fn test_box_config() {
    // Test basic configuration
    let config = BoxConfig::new(BorderStyle::Single);
    assert_eq!(config.style, BorderStyle::Single);
    assert_eq!(config.sides, BorderSides::all());
    assert!(config.is_visible());
    assert_eq!(config.total_width(), 2);
    assert_eq!(config.total_height(), 2);

    // Test builder pattern
    let config = BoxConfig::new(BorderStyle::Double)
        .with_sides(BorderSides::horizontal())
        .with_border_style(Style::new().bold());

    assert_eq!(config.style, BorderStyle::Double);
    assert_eq!(config.sides, BorderSides::horizontal());
    assert_eq!(config.border_style.bold, Some(true));
    assert!(config.is_visible());
    assert_eq!(config.total_width(), 0); // No left/right borders
    assert_eq!(config.total_height(), 2); // Top/bottom borders

    // Test invisible configuration
    let config = BoxConfig::new(BorderStyle::None);
    assert!(!config.is_visible());
    assert_eq!(config.total_width(), 0);
    assert_eq!(config.total_height(), 0);

    // Test no sides configuration
    let config = BoxConfig::new(BorderStyle::Single).with_sides(BorderSides::none());
    assert!(!config.is_visible());
}

#[test]
fn test_draw_box_basic() {
    let config = BoxConfig::new(BorderStyle::Single);
    let segments = draw_box(5, 3, &config).unwrap();

    // Should have: top border, 3 left borders, 3 right borders, bottom border = 1+3+3+1 = 8 segments
    assert_eq!(segments.len(), 8);

    // Check top border
    let top = segments.iter().find(|s| s.y == 0).unwrap();
    assert_eq!(top.content, "┌─────┐"); // 1 corner + 5 chars + 1 corner = 7 chars total
    assert_eq!(top.x, 0);
    assert_eq!(top.segment_type, BoxSegmentType::TopBorder);

    // Check bottom border
    let bottom = segments.iter().find(|s| s.y == 4).unwrap();
    assert_eq!(bottom.content, "└─────┘");
    assert_eq!(bottom.x, 0);
    assert_eq!(bottom.segment_type, BoxSegmentType::BottomBorder);

    // Check side borders
    let left_borders: Vec<_> = segments
        .iter()
        .filter(|s| s.segment_type == BoxSegmentType::LeftBorder)
        .collect();
    assert_eq!(left_borders.len(), 3);

    for (i, border) in left_borders.iter().enumerate() {
        assert_eq!(border.content, "│");
        assert_eq!(border.x, 0);
        assert_eq!(border.y, i + 1);
    }

    let right_borders: Vec<_> = segments
        .iter()
        .filter(|s| s.segment_type == BoxSegmentType::RightBorder)
        .collect();
    assert_eq!(right_borders.len(), 3);

    for (i, border) in right_borders.iter().enumerate() {
        assert_eq!(border.content, "│");
        assert_eq!(border.x, 6); // 5 + 1
        assert_eq!(border.y, i + 1);
    }
}

#[test]
fn test_draw_box_different_styles() {
    // Test double border
    let config = BoxConfig::new(BorderStyle::Double);
    let segments = draw_box(3, 2, &config).unwrap();

    let top = segments.iter().find(|s| s.y == 0).unwrap();
    assert_eq!(top.content, "╔═══╗");

    let bottom = segments.iter().find(|s| s.y == 3).unwrap();
    assert_eq!(bottom.content, "╚═══╝");

    // Test rounded corners
    let config = BoxConfig::new(BorderStyle::Rounded);
    let segments = draw_box(3, 2, &config).unwrap();

    let top = segments.iter().find(|s| s.y == 0).unwrap();
    assert_eq!(top.content, "╭───╮");

    let bottom = segments.iter().find(|s| s.y == 3).unwrap();
    assert_eq!(bottom.content, "╰───╯");

    // Test thick border
    let config = BoxConfig::new(BorderStyle::Thick);
    let segments = draw_box(3, 2, &config).unwrap();

    let top = segments.iter().find(|s| s.y == 0).unwrap();
    assert_eq!(top.content, "┏━━━┓");

    // Test ASCII fallback
    let config = BoxConfig::new(BorderStyle::Ascii);
    let segments = draw_box(3, 2, &config).unwrap();

    let top = segments.iter().find(|s| s.y == 0).unwrap();
    assert_eq!(top.content, "+---+");

    let bottom = segments.iter().find(|s| s.y == 3).unwrap();
    assert_eq!(bottom.content, "+---+");
}

#[test]
fn test_draw_box_partial_sides() {
    // Test horizontal only
    let config = BoxConfig::new(BorderStyle::Single).with_sides(BorderSides::horizontal());
    let segments = draw_box(5, 3, &config).unwrap();

    // Should have only top and bottom borders
    assert_eq!(segments.len(), 2);

    let top = segments.iter().find(|s| s.y == 0).unwrap();
    assert_eq!(top.content, "─────");
    assert_eq!(top.segment_type, BoxSegmentType::TopBorder);

    let bottom = segments.iter().find(|s| s.y == 4).unwrap(); // height + 1
    assert_eq!(bottom.content, "─────");
    assert_eq!(bottom.segment_type, BoxSegmentType::BottomBorder);

    // Test vertical only
    let config = BoxConfig::new(BorderStyle::Single).with_sides(BorderSides::vertical());
    let segments = draw_box(5, 3, &config).unwrap();

    // Should have only left and right borders
    assert_eq!(segments.len(), 6); // 3 left + 3 right

    let left_borders: Vec<_> = segments
        .iter()
        .filter(|s| s.segment_type == BoxSegmentType::LeftBorder)
        .collect();
    assert_eq!(left_borders.len(), 3);

    let right_borders: Vec<_> = segments
        .iter()
        .filter(|s| s.segment_type == BoxSegmentType::RightBorder)
        .collect();
    assert_eq!(right_borders.len(), 3);

    // Test no sides
    let config = BoxConfig::new(BorderStyle::Single).with_sides(BorderSides::none());
    let segments = draw_box(5, 3, &config).unwrap();
    assert!(segments.is_empty());
}

#[test]
fn test_horizontal_line() {
    // Test basic horizontal line
    let line = horizontal_line(10, BorderStyle::Single, Style::new()).unwrap();
    assert_eq!(line.content, "──────────");
    assert_eq!(line.content.chars().count(), 10);
    assert_eq!(line.x, 0);
    assert_eq!(line.y, 0);
    assert_eq!(line.segment_type, BoxSegmentType::TopBorder);

    // Test different border styles
    let line = horizontal_line(5, BorderStyle::Double, Style::new()).unwrap();
    assert_eq!(line.content, "═════");

    let line = horizontal_line(3, BorderStyle::Thick, Style::new()).unwrap();
    assert_eq!(line.content, "━━━");

    let line = horizontal_line(4, BorderStyle::Ascii, Style::new()).unwrap();
    assert_eq!(line.content, "----");

    // Test with styling
    let line = horizontal_line(3, BorderStyle::Single, Style::new().bold()).unwrap();
    assert_eq!(line.content, "───");
    assert_eq!(line.style.bold, Some(true));
}

#[test]
fn test_vertical_line() {
    // Test basic vertical line
    let lines = vertical_line(5, BorderStyle::Single, Style::new()).unwrap();
    assert_eq!(lines.len(), 5);

    for (i, line) in lines.iter().enumerate() {
        assert_eq!(line.content, "│");
        assert_eq!(line.x, 0);
        assert_eq!(line.y, i);
        assert_eq!(line.segment_type, BoxSegmentType::LeftBorder);
    }

    // Test different border styles
    let lines = vertical_line(3, BorderStyle::Double, Style::new()).unwrap();
    for line in &lines {
        assert_eq!(line.content, "║");
    }

    let lines = vertical_line(3, BorderStyle::Thick, Style::new()).unwrap();
    for line in &lines {
        assert_eq!(line.content, "┃");
    }

    let lines = vertical_line(3, BorderStyle::Ascii, Style::new()).unwrap();
    for line in &lines {
        assert_eq!(line.content, "|");
    }
}

#[test]
fn test_box_segment() {
    let segment = BoxSegment {
        content: "test".to_string(),
        x: 5,
        y: 10,
        style: Style::new().bold(),
        segment_type: BoxSegmentType::TopBorder,
    };

    // Test conversion to regular segment
    let regular_segment = segment.to_segment();
    assert_eq!(regular_segment.text(), "test");
    assert_eq!(regular_segment.style().bold, Some(true));
}

#[test]
fn test_error_cases() {
    // Test horizontal line with zero width
    let result = horizontal_line(0, BorderStyle::Single, Style::new());
    assert!(result.is_err());

    // Test horizontal line with invisible style
    let result = horizontal_line(5, BorderStyle::None, Style::new());
    assert!(result.is_err());

    // Test vertical line with zero height
    let result = vertical_line(0, BorderStyle::Single, Style::new());
    assert!(result.is_err());

    // Test vertical line with invisible style
    let result = vertical_line(5, BorderStyle::None, Style::new());
    assert!(result.is_err());

    // Test draw_box with invisible config
    let config = BoxConfig::new(BorderStyle::None);
    let segments = draw_box(5, 3, &config).unwrap();
    assert!(segments.is_empty());

    // Test draw_box with very small dimensions
    let config = BoxConfig::new(BorderStyle::Single);
    let segments = draw_box(0, 0, &config).unwrap();
    // A 0x0 box should still draw minimal borders (total 2x2)
    assert!(!segments.is_empty());
}

#[test]
fn test_edge_cases() {
    // Test very small box
    let config = BoxConfig::new(BorderStyle::Single);
    let segments = draw_box(1, 1, &config).unwrap();

    // Should still render borders (minimum meaningful box)
    assert!(!segments.is_empty());
    let top = segments.iter().find(|s| s.y == 0).unwrap();
    assert_eq!(top.content, "┌─┐");

    // Test box with only width or height - should still draw some borders
    let segments = draw_box(5, 0, &config).unwrap();
    assert!(!segments.is_empty()); // Should draw horizontal borders

    let segments = draw_box(0, 5, &config).unwrap();
    assert!(!segments.is_empty()); // Should draw vertical borders
}

#[test]
fn test_performance() {
    use std::time::{Duration, Instant};

    // Test that box drawing operations are reasonably fast
    let start = Instant::now();

    for i in 1..=100 {
        let config = BoxConfig::new(BorderStyle::Single);
        let _segments = draw_box(i % 20 + 1, i % 15 + 1, &config).unwrap();
    }

    let elapsed = start.elapsed();
    assert!(elapsed < Duration::from_millis(100));

    // Test line creation performance
    let start = Instant::now();

    for i in 1..=1000 {
        let _line = horizontal_line(i % 50 + 1, BorderStyle::Single, Style::new()).unwrap();
    }

    let elapsed = start.elapsed();
    assert!(elapsed < Duration::from_millis(50));
}

#[test]
fn test_memory_usage() {
    use std::mem;

    // Test that structures have reasonable memory footprints
    assert!(mem::size_of::<BorderStyle>() <= 8);
    assert!(mem::size_of::<BorderSides>() <= 8);
    assert!(mem::size_of::<BoxChars>() <= 64);
    assert!(mem::size_of::<BoxConfig>() <= 128);
    assert!(mem::size_of::<BoxSegment>() <= 128);
    assert!(mem::size_of::<BoxSegmentType>() <= 4);
}

#[test]
fn test_unicode_compatibility() {
    // Test that all Unicode box drawing characters are properly rendered
    let config = BoxConfig::new(BorderStyle::Single);
    let segments = draw_box(3, 2, &config).unwrap();

    // Verify that Unicode characters are present in the output
    let top_border = &segments[0];
    assert!(top_border.content.contains('┌'));
    assert!(top_border.content.contains('─'));
    assert!(top_border.content.contains('┐'));

    let left_borders: Vec<_> = segments
        .iter()
        .filter(|s| s.segment_type == BoxSegmentType::LeftBorder)
        .collect();

    for border in left_borders {
        assert!(border.content.contains('│'));
    }

    // Test that all border styles produce valid Unicode output
    for style in [
        BorderStyle::Single,
        BorderStyle::Double,
        BorderStyle::Rounded,
        BorderStyle::Thick,
    ] {
        let config = BoxConfig::new(style);
        let segments = draw_box(3, 2, &config).unwrap();
        assert!(!segments.is_empty());

        // Verify all segments have non-empty content
        for segment in segments {
            assert!(!segment.content.is_empty());
            assert!(segment.content.chars().all(|c| !c.is_control()));
        }
    }
}
