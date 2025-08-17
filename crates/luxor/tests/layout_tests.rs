//! Integration tests for the layout module.
//!
//! These tests verify that the layout system works correctly with padding,
//! alignment, regions, and layout options.

use luxor::{
    layout::{Align, LayoutOptions, Padding, Region, VerticalAlign, layout_content},
    measure::Measurement,
};
use std::time::Duration;

#[test]
fn test_padding_operations() {
    // Test uniform padding
    let padding = Padding::uniform(2);
    assert_eq!(padding.top, 2);
    assert_eq!(padding.right, 2);
    assert_eq!(padding.bottom, 2);
    assert_eq!(padding.left, 2);
    assert_eq!(padding.horizontal(), 4);
    assert_eq!(padding.vertical(), 4);

    // Test symmetric padding
    let padding = Padding::symmetric(3, 5);
    assert_eq!(padding.top, 3);
    assert_eq!(padding.bottom, 3);
    assert_eq!(padding.left, 5);
    assert_eq!(padding.right, 5);
    assert_eq!(padding.horizontal(), 10);
    assert_eq!(padding.vertical(), 6);

    // Test custom padding
    let padding = Padding::new(1, 2, 3, 4);
    assert_eq!(padding.top, 1);
    assert_eq!(padding.right, 2);
    assert_eq!(padding.bottom, 3);
    assert_eq!(padding.left, 4);
    assert_eq!(padding.horizontal(), 6);
    assert_eq!(padding.vertical(), 4);

    // Test zero padding
    let padding = Padding::zero();
    assert_eq!(padding.horizontal(), 0);
    assert_eq!(padding.vertical(), 0);
}

#[test]
fn test_padding_region_operations() {
    let padding = Padding::uniform(2);
    let region = Region::new(10, 20, 30, 40);

    // Test apply to region
    let result = padding.apply_to_region(region);
    assert_eq!(result.x, 12);
    assert_eq!(result.y, 22);
    assert_eq!(result.width, 26);
    assert_eq!(result.height, 36);

    // Test expand region
    let content_region = Region::new(5, 10, 15, 20);
    let expanded = padding.expand_region(content_region);
    assert_eq!(expanded.x, 3);
    assert_eq!(expanded.y, 8);
    assert_eq!(expanded.width, 19);
    assert_eq!(expanded.height, 24);
}

#[test]
fn test_alignment_calculations() {
    // Test horizontal alignment offsets
    assert_eq!(Align::Left.calculate_offset(5, 10), 0);
    assert_eq!(Align::Center.calculate_offset(5, 10), 2);
    assert_eq!(Align::Right.calculate_offset(5, 10), 5);
    assert_eq!(Align::Justify.calculate_offset(5, 10), 0); // Justify doesn't change offset

    // Test with content larger than available space
    assert_eq!(Align::Center.calculate_offset(15, 10), 0);
    assert_eq!(Align::Right.calculate_offset(15, 10), 0);

    // Test vertical alignment offsets
    assert_eq!(VerticalAlign::Top.calculate_offset(3, 10), 0);
    assert_eq!(VerticalAlign::Middle.calculate_offset(3, 10), 3);
    assert_eq!(VerticalAlign::Bottom.calculate_offset(3, 10), 7);

    // Test with content larger than available space
    assert_eq!(VerticalAlign::Middle.calculate_offset(15, 10), 0);
}

#[test]
fn test_string_alignment() {
    // Test left alignment
    assert_eq!(Align::Left.apply_to_string("hello", 10), "hello     ");

    // Test right alignment
    assert_eq!(Align::Right.apply_to_string("hello", 10), "     hello");

    // Test center alignment
    assert_eq!(Align::Center.apply_to_string("hello", 10), "  hello   ");

    // Test justify alignment with multiple words
    let justified = Align::Justify.apply_to_string("hello world", 15);
    assert_eq!(justified, "hello     world");
    assert_eq!(justified.len(), 15);

    // Test justify with three words
    let justified = Align::Justify.apply_to_string("a b c", 9);
    assert_eq!(justified, "a   b   c");
    assert_eq!(justified.len(), 9);

    // Test justify with single word (falls back to left alignment)
    let justified = Align::Justify.apply_to_string("hello", 10);
    assert_eq!(justified, "hello     ");

    // Test text longer than target width
    assert_eq!(Align::Center.apply_to_string("verylongtext", 5), "veryl");
}

#[test]
fn test_region_operations() {
    let r1 = Region::new(0, 0, 10, 10);
    let r2 = Region::new(5, 5, 10, 10);

    // Test intersection
    assert!(r1.intersects(&r2));
    let intersection = r1.intersection(&r2).unwrap();
    assert_eq!(intersection, Region::new(5, 5, 5, 5));

    // Test union
    let union = r1.union(&r2);
    assert_eq!(union, Region::new(0, 0, 15, 15));

    // Test non-intersecting regions
    let r3 = Region::new(20, 20, 5, 5);
    assert!(!r1.intersects(&r3));
    assert!(r1.intersection(&r3).is_none());
}

#[test]
fn test_region_properties() {
    let region = Region::new(10, 20, 30, 40);

    // Test basic properties
    assert_eq!(region.x, 10);
    assert_eq!(region.y, 20);
    assert_eq!(region.width, 30);
    assert_eq!(region.height, 40);
    assert_eq!(region.right(), 40);
    assert_eq!(region.bottom(), 60);
    assert_eq!(region.area(), 1200);
    assert!(!region.is_empty());

    // Test contains
    assert!(region.contains(10, 20)); // Top-left corner
    assert!(region.contains(25, 35)); // Inside
    assert!(region.contains(39, 59)); // Bottom-right corner (exclusive)
    assert!(!region.contains(40, 60)); // Outside
    assert!(!region.contains(5, 15)); // Outside

    // Test empty region
    let empty = Region::new(0, 0, 0, 0);
    assert!(empty.is_empty());
    assert_eq!(empty.area(), 0);
}

#[test]
fn test_region_transformations() {
    let region = Region::new(10, 20, 30, 40);

    // Test translation
    let translated = region.translate(5, -3);
    assert_eq!(translated.x, 15);
    assert_eq!(translated.y, 17);
    assert_eq!(translated.width, 30);
    assert_eq!(translated.height, 40);

    // Test translation with negative values
    let translated = region.translate(-5, -25);
    assert_eq!(translated.x, 5);
    assert_eq!(translated.y, 0); // Saturating subtraction

    // Test expansion
    let expanded = region.expand(1, 2, 3, 4);
    assert_eq!(expanded.x, 6); // 10 - 4
    assert_eq!(expanded.y, 19); // 20 - 1
    assert_eq!(expanded.width, 36); // 30 + 4 + 2
    assert_eq!(expanded.height, 44); // 40 + 1 + 3

    // Test shrinking
    let shrunk = region.shrink(1, 2, 3, 4);
    assert_eq!(shrunk.x, 14); // 10 + 4
    assert_eq!(shrunk.y, 21); // 20 + 1
    assert_eq!(shrunk.width, 24); // 30 - 4 - 2
    assert_eq!(shrunk.height, 36); // 40 - 1 - 3
}

#[test]
fn test_sub_regions() {
    let parent = Region::new(10, 20, 100, 100);

    // Test valid sub-region
    let sub = parent.sub_region(5, 10, 20, 30).unwrap();
    assert_eq!(sub.x, 15);
    assert_eq!(sub.y, 30);
    assert_eq!(sub.width, 20);
    assert_eq!(sub.height, 30);

    // Test sub-region at edge
    let sub = parent.sub_region(0, 0, 100, 100).unwrap();
    assert_eq!(sub, parent);

    // Test invalid sub-region (too large)
    assert!(parent.sub_region(95, 95, 20, 20).is_err());

    // Test invalid sub-region (outside bounds)
    assert!(parent.sub_region(101, 0, 5, 5).is_err());
}

#[test]
fn test_layout_options() {
    // Test default options
    let options = LayoutOptions::new();
    assert_eq!(options.align, Align::Left);
    assert_eq!(options.vertical_align, VerticalAlign::Top);
    assert_eq!(options.padding, Padding::zero());
    assert!(options.clip);

    // Test builder pattern
    let options = LayoutOptions::new()
        .with_align(Align::Center)
        .with_vertical_align(VerticalAlign::Middle)
        .with_padding(Padding::uniform(5))
        .with_clip(false);

    assert_eq!(options.align, Align::Center);
    assert_eq!(options.vertical_align, VerticalAlign::Middle);
    assert_eq!(options.padding, Padding::uniform(5));
    assert!(!options.clip);
}

#[test]
fn test_layout_content() {
    let measurement = Measurement::new(20, 50);
    let region = Region::new(0, 0, 100, 50);

    // Test with center alignment and padding
    let options = LayoutOptions::new()
        .with_align(Align::Center)
        .with_padding(Padding::uniform(5));

    let result = layout_content(&measurement, region, &options).unwrap();

    // With padding, content area is 90x40
    // Content width is 20, so center offset is (90-20)/2 = 35
    // Plus padding left offset: 5 + 35 = 40
    assert_eq!(result.x, 40);
    assert_eq!(result.y, 5);
    assert_eq!(result.width, 20);
    assert_eq!(result.height, 1);

    // Test with clipping disabled
    let options = LayoutOptions::new().with_clip(false);

    let result = layout_content(&measurement, region, &options).unwrap();
    assert_eq!(result.width, 20); // Should use minimum width even without clipping
}

#[test]
fn test_edge_cases() {
    // Test zero-width regions
    let zero_region = Region::new(0, 0, 0, 0);
    assert!(zero_region.is_empty());
    assert_eq!(zero_region.area(), 0);

    // Test layout with zero region
    let measurement = Measurement::new(10, 20);
    let options = LayoutOptions::new();
    let result = layout_content(&measurement, zero_region, &options).unwrap();
    assert_eq!(result.width, 0);
    assert_eq!(result.height, 0);

    // Test padding larger than region
    let padding = Padding::uniform(100);
    let small_region = Region::new(0, 0, 5, 5);
    let result = padding.apply_to_region(small_region);
    assert_eq!(result.width, 0); // Should saturate to 0
    assert_eq!(result.height, 0);

    // Test alignment with zero width
    assert_eq!(Align::Center.apply_to_string("test", 0), "");
    assert_eq!(Align::Right.apply_to_string("test", 0), "");

    // Test region translation edge cases
    let region = Region::new(5, 5, 10, 10);
    let translated = region.translate(-10, -10);
    assert_eq!(translated.x, 0); // Should saturate
    assert_eq!(translated.y, 0);
}

#[test]
fn test_performance_characteristics() {
    // Test that operations are fast
    let start = std::time::Instant::now();

    // Test many region operations
    for i in 0..1000 {
        let r1 = Region::new(i, i, 10, 10);
        let r2 = Region::new(i + 5, i + 5, 10, 10);
        let _intersection = r1.intersection(&r2);
        let _union = r1.union(&r2);
    }

    let region_time = start.elapsed();
    assert!(region_time < Duration::from_millis(10));

    // Test many padding operations
    let start = std::time::Instant::now();
    for i in 0..1000 {
        let padding = Padding::uniform(i % 10);
        let region = Region::new(0, 0, 100, 100);
        let _result = padding.apply_to_region(region);
    }

    let padding_time = start.elapsed();
    assert!(padding_time < Duration::from_millis(5));

    // Test many string alignments
    let start = std::time::Instant::now();
    for i in 0..1000 {
        let text = format!("text{}", i);
        let _aligned = Align::Center.apply_to_string(&text, 20);
    }

    let align_time = start.elapsed();
    assert!(align_time < Duration::from_millis(50));
}

#[test]
fn test_unicode_handling() {
    // Test alignment with Unicode text
    let unicode_text = "你好世界";
    let aligned = Align::Center.apply_to_string(unicode_text, 10);
    assert_eq!(aligned.chars().count(), 10);

    // Test that Unicode characters are handled correctly in justify
    let unicode_justify = Align::Justify.apply_to_string("测试 文本", 10);
    assert_eq!(unicode_justify.chars().count(), 5); // Only 5 chars: 测试[space]文本, no padding added

    // Test empty Unicode strings
    let empty_unicode = "";
    let aligned = Align::Center.apply_to_string(empty_unicode, 5);
    assert_eq!(aligned, "     ");
}

#[test]
fn test_memory_usage() {
    use std::mem;

    // Test that structures have reasonable memory footprints
    assert!(mem::size_of::<Padding>() <= 32);
    assert!(mem::size_of::<Region>() <= 32);
    assert!(mem::size_of::<Align>() <= 4);
    assert!(mem::size_of::<VerticalAlign>() <= 4);
    assert!(mem::size_of::<LayoutOptions>() <= 64);
}
