//! Layout utilities for positioning and spacing content in the terminal.
//!
//! This module provides structures and utilities for managing layout in rich terminal applications:
//! - `Padding` for spacing around content
//! - `Align` for text and content alignment
//! - `Region` for rectangular areas and clipping
//! - `Justify` for text justification algorithms

use crate::{error::LuxorError, measure::Measurement};
use std::cmp::{max, min};

/// Represents padding (spacing) around content.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Padding {
    /// Top padding
    pub top: usize,
    /// Right padding  
    pub right: usize,
    /// Bottom padding
    pub bottom: usize,
    /// Left padding
    pub left: usize,
}

impl Padding {
    /// Create padding with the same value on all sides.
    pub fn uniform(padding: usize) -> Self {
        Self {
            top: padding,
            right: padding,
            bottom: padding,
            left: padding,
        }
    }

    /// Create padding with different horizontal and vertical values.
    pub fn symmetric(vertical: usize, horizontal: usize) -> Self {
        Self {
            top: vertical,
            right: horizontal,
            bottom: vertical,
            left: horizontal,
        }
    }

    /// Create padding with individual values for each side.
    pub fn new(top: usize, right: usize, bottom: usize, left: usize) -> Self {
        Self {
            top,
            right,
            bottom,
            left,
        }
    }

    /// Create zero padding.
    pub fn zero() -> Self {
        Self::uniform(0)
    }

    /// Get the total horizontal padding (left + right).
    pub fn horizontal(&self) -> usize {
        self.left + self.right
    }

    /// Get the total vertical padding (top + bottom).
    pub fn vertical(&self) -> usize {
        self.top + self.bottom
    }

    /// Get the total padding as (horizontal, vertical).
    pub fn total(&self) -> (usize, usize) {
        (self.horizontal(), self.vertical())
    }

    /// Apply padding to a region, reducing its usable area.
    pub fn apply_to_region(&self, region: Region) -> Region {
        let new_x = region.x + self.left;
        let new_y = region.y + self.top;
        let new_width = region.width.saturating_sub(self.horizontal());
        let new_height = region.height.saturating_sub(self.vertical());

        Region::new(new_x, new_y, new_width, new_height)
    }

    /// Calculate the outer region needed to fit content with this padding.
    pub fn expand_region(&self, content_region: Region) -> Region {
        let new_x = content_region.x.saturating_sub(self.left);
        let new_y = content_region.y.saturating_sub(self.top);
        let new_width = content_region.width + self.horizontal();
        let new_height = content_region.height + self.vertical();

        Region::new(new_x, new_y, new_width, new_height)
    }
}

impl Default for Padding {
    fn default() -> Self {
        Self::zero()
    }
}

/// Horizontal alignment options.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Align {
    /// Align to the left
    Left,
    /// Center alignment
    Center,
    /// Align to the right
    Right,
    /// Justify text (stretch to fill width)
    Justify,
}

impl Default for Align {
    fn default() -> Self {
        Self::Left
    }
}

impl Align {
    /// Calculate the starting position for content with this alignment.
    ///
    /// # Arguments
    /// * `content_width` - The width of the content to align
    /// * `available_width` - The total width available for alignment
    ///
    /// # Returns
    /// The offset from the left edge where content should start
    pub fn calculate_offset(&self, content_width: usize, available_width: usize) -> usize {
        if content_width >= available_width {
            return 0;
        }

        match self {
            Self::Left => 0,
            Self::Center => (available_width - content_width) / 2,
            Self::Right => available_width - content_width,
            Self::Justify => 0, // Justify doesn't change offset, it modifies content
        }
    }

    /// Apply alignment to a string by adding padding spaces.
    ///
    /// # Arguments  
    /// * `text` - The text to align
    /// * `width` - The target width
    ///
    /// # Returns
    /// The aligned text padded to the specified width
    pub fn apply_to_string(&self, text: &str, width: usize) -> String {
        let text_len = text.chars().count();

        if text_len >= width {
            return text.chars().take(width).collect();
        }

        let padding_needed = width - text_len;

        match self {
            Self::Left => format!("{}{}", text, " ".repeat(padding_needed)),
            Self::Right => format!("{}{}", " ".repeat(padding_needed), text),
            Self::Center => {
                let left_padding = padding_needed / 2;
                let right_padding = padding_needed - left_padding;
                format!(
                    "{}{}{}",
                    " ".repeat(left_padding),
                    text,
                    " ".repeat(right_padding)
                )
            }
            Self::Justify => {
                // For single words, fall back to left alignment
                if !text.contains(' ') {
                    return Self::Left.apply_to_string(text, width);
                }
                justify_text(text, width)
            }
        }
    }
}

/// Vertical alignment options.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerticalAlign {
    /// Align to the top
    Top,
    /// Center vertically
    Middle,
    /// Align to the bottom  
    Bottom,
}

impl Default for VerticalAlign {
    fn default() -> Self {
        Self::Top
    }
}

impl VerticalAlign {
    /// Calculate the starting row for content with this vertical alignment.
    ///
    /// # Arguments
    /// * `content_height` - The height of the content to align
    /// * `available_height` - The total height available for alignment
    ///
    /// # Returns
    /// The offset from the top where content should start
    pub fn calculate_offset(&self, content_height: usize, available_height: usize) -> usize {
        if content_height >= available_height {
            return 0;
        }

        match self {
            Self::Top => 0,
            Self::Middle => (available_height - content_height) / 2,
            Self::Bottom => available_height - content_height,
        }
    }
}

/// Represents a rectangular region in the terminal.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Region {
    /// X coordinate (column) of the top-left corner
    pub x: usize,
    /// Y coordinate (row) of the top-left corner  
    pub y: usize,
    /// Width of the region
    pub width: usize,
    /// Height of the region
    pub height: usize,
}

impl Region {
    /// Create a new region.
    pub fn new(x: usize, y: usize, width: usize, height: usize) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    /// Create a region starting at origin (0, 0).
    pub fn from_size(width: usize, height: usize) -> Self {
        Self::new(0, 0, width, height)
    }

    /// Check if this region is empty (zero width or height).
    pub fn is_empty(&self) -> bool {
        self.width == 0 || self.height == 0
    }

    /// Get the area of this region.
    pub fn area(&self) -> usize {
        self.width * self.height
    }

    /// Get the right edge x-coordinate (exclusive).
    pub fn right(&self) -> usize {
        self.x + self.width
    }

    /// Get the bottom edge y-coordinate (exclusive).
    pub fn bottom(&self) -> usize {
        self.y + self.height
    }

    /// Check if a point is contained within this region.
    pub fn contains(&self, x: usize, y: usize) -> bool {
        x >= self.x && x < self.right() && y >= self.y && y < self.bottom()
    }

    /// Check if this region intersects with another region.
    pub fn intersects(&self, other: &Region) -> bool {
        self.x < other.right()
            && self.right() > other.x
            && self.y < other.bottom()
            && self.bottom() > other.y
    }

    /// Calculate the intersection of this region with another region.
    /// Returns None if the regions don't intersect.
    pub fn intersection(&self, other: &Region) -> Option<Region> {
        if !self.intersects(other) {
            return None;
        }

        let x = max(self.x, other.x);
        let y = max(self.y, other.y);
        let right = min(self.right(), other.right());
        let bottom = min(self.bottom(), other.bottom());

        Some(Region::new(x, y, right - x, bottom - y))
    }

    /// Calculate the union of this region with another region.
    pub fn union(&self, other: &Region) -> Region {
        let x = min(self.x, other.x);
        let y = min(self.y, other.y);
        let right = max(self.right(), other.right());
        let bottom = max(self.bottom(), other.bottom());

        Region::new(x, y, right - x, bottom - y)
    }

    /// Move this region by the given offset.
    pub fn translate(&self, dx: isize, dy: isize) -> Region {
        let new_x = if dx < 0 {
            self.x.saturating_sub((-dx) as usize)
        } else {
            self.x + (dx as usize)
        };

        let new_y = if dy < 0 {
            self.y.saturating_sub((-dy) as usize)
        } else {
            self.y + (dy as usize)
        };

        Region::new(new_x, new_y, self.width, self.height)
    }

    /// Expand this region by the given amounts in each direction.
    pub fn expand(&self, top: usize, right: usize, bottom: usize, left: usize) -> Region {
        let new_x = self.x.saturating_sub(left);
        let new_y = self.y.saturating_sub(top);
        let new_width = self.width + left + right;
        let new_height = self.height + top + bottom;

        Region::new(new_x, new_y, new_width, new_height)
    }

    /// Shrink this region by the given amounts in each direction.
    pub fn shrink(&self, top: usize, right: usize, bottom: usize, left: usize) -> Region {
        let new_x = self.x + left;
        let new_y = self.y + top;
        let new_width = self.width.saturating_sub(left + right);
        let new_height = self.height.saturating_sub(top + bottom);

        Region::new(new_x, new_y, new_width, new_height)
    }

    /// Create a sub-region within this region.
    pub fn sub_region(
        &self,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
    ) -> Result<Region, LuxorError> {
        if x + width > self.width || y + height > self.height {
            return Err(LuxorError::LayoutError(format!(
                "Sub-region {}x{} at ({}, {}) exceeds parent region {}x{}",
                width, height, x, y, self.width, self.height
            )));
        }

        Ok(Region::new(self.x + x, self.y + y, width, height))
    }
}

impl Default for Region {
    fn default() -> Self {
        Self::new(0, 0, 0, 0)
    }
}

/// Justify text to fill the given width by adding spaces between words.
fn justify_text(text: &str, width: usize) -> String {
    let words: Vec<&str> = text.split_whitespace().collect();

    if words.len() <= 1 {
        // Single word or empty - fall back to left alignment
        return format!("{}{}", text, " ".repeat(width.saturating_sub(text.len())));
    }

    let total_word_length: usize = words.iter().map(|w| w.len()).sum();
    let total_spaces_needed = width.saturating_sub(total_word_length);
    let gaps = words.len() - 1;

    if gaps == 0 || total_spaces_needed == 0 {
        return text.to_string();
    }

    let spaces_per_gap = total_spaces_needed / gaps;
    let extra_spaces = total_spaces_needed % gaps;

    let mut result = String::new();
    for (i, word) in words.iter().enumerate() {
        result.push_str(word);

        if i < words.len() - 1 {
            // Add normal spaces plus one extra space for the first few gaps
            let spaces_for_this_gap = spaces_per_gap + if i < extra_spaces { 1 } else { 0 };
            result.push_str(&" ".repeat(spaces_for_this_gap));
        }
    }

    result
}

/// Layout options for arranging content.
#[derive(Debug, Clone)]
pub struct LayoutOptions {
    /// Horizontal alignment
    pub align: Align,
    /// Vertical alignment
    pub vertical_align: VerticalAlign,
    /// Padding around content
    pub padding: Padding,
    /// Whether to clip content that exceeds the region
    pub clip: bool,
}

impl LayoutOptions {
    /// Create new layout options with default values.
    pub fn new() -> Self {
        Self {
            align: Align::Left,
            vertical_align: VerticalAlign::Top,
            padding: Padding::zero(),
            clip: true,
        }
    }

    /// Set horizontal alignment.
    pub fn with_align(mut self, align: Align) -> Self {
        self.align = align;
        self
    }

    /// Set vertical alignment.
    pub fn with_vertical_align(mut self, align: VerticalAlign) -> Self {
        self.vertical_align = align;
        self
    }

    /// Set padding.
    pub fn with_padding(mut self, padding: Padding) -> Self {
        self.padding = padding;
        self
    }

    /// Set clipping behavior.
    pub fn with_clip(mut self, clip: bool) -> Self {
        self.clip = clip;
        self
    }
}

impl Default for LayoutOptions {
    fn default() -> Self {
        Self::new()
    }
}

/// Calculate the position and size for content within a region using layout options.
pub fn layout_content(
    content_measurement: &Measurement,
    region: Region,
    options: &LayoutOptions,
) -> Result<Region, LuxorError> {
    if region.is_empty() {
        return Ok(Region::from_size(0, 0));
    }

    // Apply padding to get the content area
    let content_area = options.padding.apply_to_region(region);

    if content_area.is_empty() {
        return Ok(content_area);
    }

    // Determine content size
    let content_width = if options.clip {
        min(content_measurement.minimum(), content_area.width)
    } else {
        content_measurement.minimum()
    };

    let content_height = 1; // For now, assume single line height

    // Calculate alignment offsets
    let x_offset = options
        .align
        .calculate_offset(content_width, content_area.width);
    let y_offset = options
        .vertical_align
        .calculate_offset(content_height, content_area.height);

    // Create the final content region
    let final_x = content_area.x + x_offset;
    let final_y = content_area.y + y_offset;
    let final_width = if options.clip {
        min(content_width, content_area.width - x_offset)
    } else {
        content_width
    };
    let final_height = if options.clip {
        min(content_height, content_area.height - y_offset)
    } else {
        content_height
    };

    Ok(Region::new(final_x, final_y, final_width, final_height))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_padding_creation() {
        let padding = Padding::uniform(2);
        assert_eq!(padding.top, 2);
        assert_eq!(padding.right, 2);
        assert_eq!(padding.bottom, 2);
        assert_eq!(padding.left, 2);
        assert_eq!(padding.horizontal(), 4);
        assert_eq!(padding.vertical(), 4);
    }

    #[test]
    fn test_padding_symmetric() {
        let padding = Padding::symmetric(3, 5);
        assert_eq!(padding.top, 3);
        assert_eq!(padding.bottom, 3);
        assert_eq!(padding.left, 5);
        assert_eq!(padding.right, 5);
        assert_eq!(padding.horizontal(), 10);
        assert_eq!(padding.vertical(), 6);
    }

    #[test]
    fn test_padding_apply_to_region() {
        let padding = Padding::uniform(2);
        let region = Region::new(10, 20, 30, 40);
        let result = padding.apply_to_region(region);

        assert_eq!(result.x, 12);
        assert_eq!(result.y, 22);
        assert_eq!(result.width, 26);
        assert_eq!(result.height, 36);
    }

    #[test]
    fn test_align_calculate_offset() {
        assert_eq!(Align::Left.calculate_offset(5, 10), 0);
        assert_eq!(Align::Center.calculate_offset(5, 10), 2);
        assert_eq!(Align::Right.calculate_offset(5, 10), 5);

        // Content larger than available space
        assert_eq!(Align::Center.calculate_offset(15, 10), 0);
    }

    #[test]
    fn test_align_apply_to_string() {
        assert_eq!(Align::Left.apply_to_string("hello", 10), "hello     ");
        assert_eq!(Align::Right.apply_to_string("hello", 10), "     hello");
        assert_eq!(Align::Center.apply_to_string("hello", 10), "  hello   ");
    }

    #[test]
    fn test_justify_text() {
        let result = justify_text("hello world", 15);
        assert_eq!(result, "hello     world");
        assert_eq!(result.len(), 15);

        let result = justify_text("a b c", 9);
        assert_eq!(result, "a   b   c");
        assert_eq!(result.len(), 9);
    }

    #[test]
    fn test_region_operations() {
        let r1 = Region::new(0, 0, 10, 10);
        let r2 = Region::new(5, 5, 10, 10);

        assert!(r1.intersects(&r2));

        let intersection = r1.intersection(&r2).unwrap();
        assert_eq!(intersection, Region::new(5, 5, 5, 5));

        let union = r1.union(&r2);
        assert_eq!(union, Region::new(0, 0, 15, 15));
    }

    #[test]
    fn test_region_contains() {
        let region = Region::new(10, 20, 30, 40);

        assert!(region.contains(10, 20)); // Top-left corner
        assert!(region.contains(25, 35)); // Inside
        assert!(region.contains(39, 59)); // Bottom-right corner (exclusive)
        assert!(!region.contains(40, 60)); // Outside
        assert!(!region.contains(5, 15)); // Outside
    }

    #[test]
    fn test_region_translate() {
        let region = Region::new(10, 20, 30, 40);
        let translated = region.translate(5, -3);

        assert_eq!(translated.x, 15);
        assert_eq!(translated.y, 17);
        assert_eq!(translated.width, 30);
        assert_eq!(translated.height, 40);
    }

    #[test]
    fn test_region_sub_region() {
        let parent = Region::new(10, 20, 100, 100);
        let sub = parent.sub_region(5, 10, 20, 30).unwrap();

        assert_eq!(sub.x, 15);
        assert_eq!(sub.y, 30);
        assert_eq!(sub.width, 20);
        assert_eq!(sub.height, 30);

        // Test error case
        assert!(parent.sub_region(95, 95, 20, 20).is_err());
    }

    #[test]
    fn test_vertical_align_calculate_offset() {
        assert_eq!(VerticalAlign::Top.calculate_offset(3, 10), 0);
        assert_eq!(VerticalAlign::Middle.calculate_offset(3, 10), 3);
        assert_eq!(VerticalAlign::Bottom.calculate_offset(3, 10), 7);

        // Content larger than available space
        assert_eq!(VerticalAlign::Middle.calculate_offset(15, 10), 0);
    }

    #[test]
    fn test_layout_content() {
        let measurement = Measurement::new(20, 50);
        let region = Region::new(0, 0, 100, 50);
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
    }
}
