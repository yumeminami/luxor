//! Unicode box drawing utilities for creating borders and frames.
//!
//! This module provides utilities for drawing boxes and borders using Unicode
//! box drawing characters. It supports various border styles and handles
//! corner connections properly.

use crate::{error::LuxorError, segment::Segment, style::Style};

/// Type alias for box drawing results
pub type BoxDrawingResult = Result<Vec<BoxSegment>, LuxorError>;

/// Unicode box drawing characters for different border styles.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BoxChars {
    /// Top-left corner character
    pub top_left: char,
    /// Top-right corner character  
    pub top_right: char,
    /// Bottom-left corner character
    pub bottom_left: char,
    /// Bottom-right corner character
    pub bottom_right: char,
    /// Horizontal line character
    pub horizontal: char,
    /// Vertical line character
    pub vertical: char,
    /// Top tee (for intersections)
    pub top_tee: char,
    /// Bottom tee (for intersections)
    pub bottom_tee: char,
    /// Left tee (for intersections)
    pub left_tee: char,
    /// Right tee (for intersections)
    pub right_tee: char,
    /// Cross (for intersections)
    pub cross: char,
}

impl BoxChars {
    /// Get box characters for single-line borders.
    pub const fn single() -> Self {
        Self {
            top_left: '┌',
            top_right: '┐',
            bottom_left: '└',
            bottom_right: '┘',
            horizontal: '─',
            vertical: '│',
            top_tee: '┬',
            bottom_tee: '┴',
            left_tee: '├',
            right_tee: '┤',
            cross: '┼',
        }
    }

    /// Get box characters for double-line borders.
    pub const fn double() -> Self {
        Self {
            top_left: '╔',
            top_right: '╗',
            bottom_left: '╚',
            bottom_right: '╝',
            horizontal: '═',
            vertical: '║',
            top_tee: '╦',
            bottom_tee: '╩',
            left_tee: '╠',
            right_tee: '╣',
            cross: '╬',
        }
    }

    /// Get box characters for rounded corners.
    pub const fn rounded() -> Self {
        Self {
            top_left: '╭',
            top_right: '╮',
            bottom_left: '╰',
            bottom_right: '╯',
            horizontal: '─',
            vertical: '│',
            top_tee: '┬',
            bottom_tee: '┴',
            left_tee: '├',
            right_tee: '┤',
            cross: '┼',
        }
    }

    /// Get box characters for thick borders.
    pub const fn thick() -> Self {
        Self {
            top_left: '┏',
            top_right: '┓',
            bottom_left: '┗',
            bottom_right: '┛',
            horizontal: '━',
            vertical: '┃',
            top_tee: '┳',
            bottom_tee: '┻',
            left_tee: '┣',
            right_tee: '┫',
            cross: '╋',
        }
    }

    /// Get ASCII fallback characters for compatibility.
    pub const fn ascii() -> Self {
        Self {
            top_left: '+',
            top_right: '+',
            bottom_left: '+',
            bottom_right: '+',
            horizontal: '-',
            vertical: '|',
            top_tee: '+',
            bottom_tee: '+',
            left_tee: '+',
            right_tee: '+',
            cross: '+',
        }
    }
}

/// Border styles available for drawing boxes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BorderStyle {
    /// No border
    None,
    /// Single-line border (┌─┐)
    Single,
    /// Double-line border (╔═╗)
    Double,
    /// Rounded corners border (╭─╮)
    Rounded,
    /// Thick border (┏━┓)
    Thick,
    /// ASCII compatible border (+--+)
    Ascii,
}

impl BorderStyle {
    /// Get the box characters for this border style.
    pub fn chars(&self) -> Option<BoxChars> {
        match self {
            Self::None => None,
            Self::Single => Some(BoxChars::single()),
            Self::Double => Some(BoxChars::double()),
            Self::Rounded => Some(BoxChars::rounded()),
            Self::Thick => Some(BoxChars::thick()),
            Self::Ascii => Some(BoxChars::ascii()),
        }
    }

    /// Check if this border style is visible.
    pub fn is_visible(&self) -> bool {
        !matches!(self, Self::None)
    }

    /// Get the border thickness (width/height) for layout calculations.
    pub fn thickness(&self) -> usize {
        match self {
            Self::None => 0,
            _ => 1,
        }
    }
}

impl Default for BorderStyle {
    fn default() -> Self {
        Self::Single
    }
}

/// Represents which sides of a box should have borders.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BorderSides {
    /// Draw top border
    pub top: bool,
    /// Draw right border
    pub right: bool,
    /// Draw bottom border
    pub bottom: bool,
    /// Draw left border
    pub left: bool,
}

impl BorderSides {
    /// Create border sides with all sides enabled.
    pub const fn all() -> Self {
        Self {
            top: true,
            right: true,
            bottom: true,
            left: true,
        }
    }

    /// Create border sides with no sides enabled.
    pub const fn none() -> Self {
        Self {
            top: false,
            right: false,
            bottom: false,
            left: false,
        }
    }

    /// Create border sides with only horizontal borders (top and bottom).
    pub const fn horizontal() -> Self {
        Self {
            top: true,
            right: false,
            bottom: true,
            left: false,
        }
    }

    /// Create border sides with only vertical borders (left and right).
    pub const fn vertical() -> Self {
        Self {
            top: false,
            right: true,
            bottom: false,
            left: true,
        }
    }

    /// Create border sides with individual side settings.
    pub const fn new(top: bool, right: bool, bottom: bool, left: bool) -> Self {
        Self {
            top,
            right,
            bottom,
            left,
        }
    }

    /// Check if any side has a border.
    pub fn has_border(&self) -> bool {
        self.top || self.right || self.bottom || self.left
    }

    /// Get the total horizontal border width (left + right).
    pub fn horizontal_width(&self, style: BorderStyle) -> usize {
        let thickness = style.thickness();
        let left_width = if self.left { thickness } else { 0 };
        let right_width = if self.right { thickness } else { 0 };
        left_width + right_width
    }

    /// Get the total vertical border height (top + bottom).
    pub fn vertical_height(&self, style: BorderStyle) -> usize {
        let thickness = style.thickness();
        let top_height = if self.top { thickness } else { 0 };
        let bottom_height = if self.bottom { thickness } else { 0 };
        top_height + bottom_height
    }
}

impl Default for BorderSides {
    fn default() -> Self {
        Self::all()
    }
}

/// Configuration for drawing a box border.
#[derive(Debug, Clone)]
pub struct BoxConfig {
    /// Border style
    pub style: BorderStyle,
    /// Which sides to draw
    pub sides: BorderSides,
    /// Style for the border (color, etc.)
    pub border_style: Style,
}

impl BoxConfig {
    /// Create a new box configuration.
    pub fn new(style: BorderStyle) -> Self {
        Self {
            style,
            sides: BorderSides::all(),
            border_style: Style::default(),
        }
    }

    /// Set which sides to draw.
    pub fn with_sides(mut self, sides: BorderSides) -> Self {
        self.sides = sides;
        self
    }

    /// Set the border style (color, etc.).
    pub fn with_border_style(mut self, style: Style) -> Self {
        self.border_style = style;
        self
    }

    /// Check if this configuration will draw any border.
    pub fn is_visible(&self) -> bool {
        self.style.is_visible() && self.sides.has_border()
    }

    /// Get the total border width needed for layout.
    pub fn total_width(&self) -> usize {
        self.sides.horizontal_width(self.style)
    }

    /// Get the total border height needed for layout.
    pub fn total_height(&self) -> usize {
        self.sides.vertical_height(self.style)
    }
}

impl Default for BoxConfig {
    fn default() -> Self {
        Self::new(BorderStyle::Single)
    }
}

/// Draw a box border with the given dimensions and configuration.
///
/// Returns a vector of segments representing the border lines.
/// The segments include position information for proper placement.
///
/// # Arguments
/// * `width` - Inner width of the box (content area)
/// * `height` - Inner height of the box (content area)
/// * `config` - Box configuration (style, sides, colors)
///
/// # Returns
/// Vector of segments that make up the border, with position information
pub fn draw_box(width: usize, height: usize, config: &BoxConfig) -> BoxDrawingResult {
    if !config.is_visible() {
        return Ok(Vec::new());
    }

    let chars = config
        .style
        .chars()
        .ok_or_else(|| LuxorError::rendering("Border style has no characters"))?;

    let mut segments = Vec::new();

    // Calculate total dimensions including borders
    let total_width = width + config.total_width();
    let total_height = height + config.total_height();

    if total_width < 2 || total_height < 2 {
        return Ok(segments); // Too small to draw a box
    }

    // Draw top border
    if config.sides.top {
        let mut top_line = String::new();

        // Top-left corner
        if config.sides.left {
            top_line.push(chars.top_left);
        }

        // Top horizontal line
        if width > 0 {
            top_line.push_str(&chars.horizontal.to_string().repeat(width));
        }

        // Top-right corner
        if config.sides.right {
            top_line.push(chars.top_right);
        }

        segments.push(BoxSegment {
            content: top_line,
            x: 0,
            y: 0,
            style: config.border_style.clone(),
            segment_type: BoxSegmentType::TopBorder,
        });
    }

    // Draw side borders for each row
    for row in 0..height {
        let y = if config.sides.top { row + 1 } else { row };

        // Left border
        if config.sides.left {
            segments.push(BoxSegment {
                content: chars.vertical.to_string(),
                x: 0,
                y,
                style: config.border_style.clone(),
                segment_type: BoxSegmentType::LeftBorder,
            });
        }

        // Right border
        if config.sides.right {
            let x = if config.sides.left { width + 1 } else { width };
            segments.push(BoxSegment {
                content: chars.vertical.to_string(),
                x,
                y,
                style: config.border_style.clone(),
                segment_type: BoxSegmentType::RightBorder,
            });
        }
    }

    // Draw bottom border
    if config.sides.bottom {
        let mut bottom_line = String::new();
        let y = if config.sides.top { height + 1 } else { height };

        // Bottom-left corner
        if config.sides.left {
            bottom_line.push(chars.bottom_left);
        }

        // Bottom horizontal line
        if width > 0 {
            bottom_line.push_str(&chars.horizontal.to_string().repeat(width));
        }

        // Bottom-right corner
        if config.sides.right {
            bottom_line.push(chars.bottom_right);
        }

        segments.push(BoxSegment {
            content: bottom_line,
            x: 0,
            y,
            style: config.border_style.clone(),
            segment_type: BoxSegmentType::BottomBorder,
        });
    }

    Ok(segments)
}

/// A segment that represents part of a box border.
#[derive(Debug, Clone)]
pub struct BoxSegment {
    /// The text content of this segment
    pub content: String,
    /// X position (column) where this segment should be placed
    pub x: usize,
    /// Y position (row) where this segment should be placed
    pub y: usize,
    /// Style to apply to this segment
    pub style: Style,
    /// Type of border segment
    pub segment_type: BoxSegmentType,
}

impl BoxSegment {
    /// Convert this box segment to a regular rendering segment.
    pub fn to_segment(&self) -> Segment {
        Segment::new(self.content.clone(), self.style.clone())
    }
}

/// Type of box border segment.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BoxSegmentType {
    /// Top border line
    TopBorder,
    /// Bottom border line
    BottomBorder,
    /// Left border character
    LeftBorder,
    /// Right border character
    RightBorder,
}

/// Create a simple horizontal line with the given style.
///
/// # Arguments
/// * `width` - Width of the line
/// * `style` - Border style to use
/// * `line_style` - Style for the line (color, etc.)
///
/// # Returns
/// A box segment representing the horizontal line
pub fn horizontal_line(
    width: usize,
    style: BorderStyle,
    line_style: Style,
) -> Result<BoxSegment, LuxorError> {
    if !style.is_visible() || width == 0 {
        return Err(LuxorError::rendering("Cannot create empty line"));
    }

    let chars = style
        .chars()
        .ok_or_else(|| LuxorError::rendering("Border style has no characters"))?;

    Ok(BoxSegment {
        content: chars.horizontal.to_string().repeat(width),
        x: 0,
        y: 0,
        style: line_style,
        segment_type: BoxSegmentType::TopBorder,
    })
}

/// Create a simple vertical line with the given style.
///
/// # Arguments
/// * `height` - Height of the line
/// * `style` - Border style to use
/// * `line_style` - Style for the line (color, etc.)
///
/// # Returns
/// Vector of box segments representing the vertical line
pub fn vertical_line(height: usize, style: BorderStyle, line_style: Style) -> BoxDrawingResult {
    if !style.is_visible() || height == 0 {
        return Err(LuxorError::rendering("Cannot create empty line"));
    }

    let chars = style
        .chars()
        .ok_or_else(|| LuxorError::rendering("Border style has no characters"))?;

    let mut segments = Vec::new();
    for y in 0..height {
        segments.push(BoxSegment {
            content: chars.vertical.to_string(),
            x: 0,
            y,
            style: line_style.clone(),
            segment_type: BoxSegmentType::LeftBorder,
        });
    }

    Ok(segments)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::Color;

    #[test]
    fn test_box_chars() {
        let single = BoxChars::single();
        assert_eq!(single.top_left, '┌');
        assert_eq!(single.horizontal, '─');
        assert_eq!(single.vertical, '│');

        let double = BoxChars::double();
        assert_eq!(double.top_left, '╔');
        assert_eq!(double.horizontal, '═');
        assert_eq!(double.vertical, '║');
    }

    #[test]
    fn test_border_style() {
        assert!(BorderStyle::Single.is_visible());
        assert!(!BorderStyle::None.is_visible());
        assert_eq!(BorderStyle::Single.thickness(), 1);
        assert_eq!(BorderStyle::None.thickness(), 0);
    }

    #[test]
    fn test_border_sides() {
        let all = BorderSides::all();
        assert!(all.has_border());
        assert_eq!(all.horizontal_width(BorderStyle::Single), 2);
        assert_eq!(all.vertical_height(BorderStyle::Single), 2);

        let none = BorderSides::none();
        assert!(!none.has_border());
        assert_eq!(none.horizontal_width(BorderStyle::Single), 0);
        assert_eq!(none.vertical_height(BorderStyle::Single), 0);

        let horizontal = BorderSides::horizontal();
        assert!(horizontal.has_border());
        assert_eq!(horizontal.horizontal_width(BorderStyle::Single), 0);
        assert_eq!(horizontal.vertical_height(BorderStyle::Single), 2);
    }

    #[test]
    fn test_box_config() {
        let config = BoxConfig::new(BorderStyle::Single)
            .with_sides(BorderSides::all())
            .with_border_style(Style::new().color(Color::rgb(255, 0, 0)));

        assert!(config.is_visible());
        assert_eq!(config.total_width(), 2);
        assert_eq!(config.total_height(), 2);
    }

    #[test]
    fn test_draw_simple_box() {
        let config = BoxConfig::new(BorderStyle::Single);
        let segments = draw_box(5, 3, &config).unwrap();

        // Should have: top border, 3 left borders, 3 right borders, bottom border = 8 segments
        assert_eq!(segments.len(), 8);

        // Check top border
        let top = segments.iter().find(|s| s.y == 0).unwrap();
        assert_eq!(top.content, "┌─────┐");
        assert_eq!(top.x, 0);

        // Check bottom border
        let bottom = segments.iter().find(|s| s.y == 4).unwrap();
        assert_eq!(bottom.content, "└─────┘");
        assert_eq!(bottom.x, 0);

        // Check side borders
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
    }

    #[test]
    fn test_draw_box_partial_sides() {
        let config = BoxConfig::new(BorderStyle::Single).with_sides(BorderSides::horizontal());
        let segments = draw_box(5, 3, &config).unwrap();

        // Should have only top and bottom borders
        assert_eq!(segments.len(), 2);

        let top = segments.iter().find(|s| s.y == 0).unwrap();
        assert_eq!(top.content, "─────");

        let bottom = segments.iter().find(|s| s.y == 4).unwrap();
        assert_eq!(bottom.content, "─────");
    }

    #[test]
    fn test_horizontal_line() {
        let line = horizontal_line(
            10,
            BorderStyle::Double,
            Style::new().color(Color::rgb(0, 0, 255)),
        )
        .unwrap();

        assert_eq!(line.content, "══════════");
        assert_eq!(line.content.chars().count(), 10);
        assert_eq!(line.x, 0);
        assert_eq!(line.y, 0);
    }

    #[test]
    fn test_vertical_line() {
        let lines = vertical_line(5, BorderStyle::Thick, Style::new()).unwrap();

        assert_eq!(lines.len(), 5);
        for (i, line) in lines.iter().enumerate() {
            assert_eq!(line.content, "┃");
            assert_eq!(line.x, 0);
            assert_eq!(line.y, i);
        }
    }

    #[test]
    fn test_no_border() {
        let config = BoxConfig::new(BorderStyle::None);
        let segments = draw_box(5, 3, &config).unwrap();
        assert!(segments.is_empty());
    }

    #[test]
    fn test_too_small_box() {
        let config = BoxConfig::new(BorderStyle::Single);
        let segments = draw_box(0, 0, &config).unwrap();
        assert!(!segments.is_empty()); // Should still draw minimal borders
    }

    #[test]
    fn test_ascii_fallback() {
        let config = BoxConfig::new(BorderStyle::Ascii);
        let segments = draw_box(3, 2, &config).unwrap();

        let top = segments.iter().find(|s| s.y == 0).unwrap();
        assert_eq!(top.content, "+---+");

        let bottom = segments.iter().find(|s| s.y == 3).unwrap();
        assert_eq!(bottom.content, "+---+");
    }
}
