//! Rule component for drawing horizontal and vertical lines.
//!
//! Rules are used to create visual separators and dividers in terminal interfaces.
//! They support different orientations, styles, and alignment options.

use crate::{
    box_drawing::{BorderStyle, BoxChars},
    console::Console,
    console::ConsoleOptions,
    error::LuxorError,
    layout::{Align, VerticalAlign},
    measure::Measurement,
    protocol::{Measurable, Renderable},
    segment::Segment,
    style::Style,
    text::Text,
};

/// Type alias for rule rendering results
pub type RuleRenderResult = Result<Vec<Segment>, LuxorError>;

/// A rule component for drawing lines and separators.
#[derive(Debug, Clone)]
pub struct Rule {
    /// Optional title text to display with the rule
    title: Option<Text>,
    /// Style for the rule line
    style: Style,
    /// Border style for the line characters
    border_style: BorderStyle,
    /// Horizontal alignment of the rule
    align: Align,
    /// Vertical alignment of the rule (for horizontal rules)
    vertical_align: VerticalAlign,
    /// Character to use for the rule line (overrides border style if set)
    character: Option<char>,
    /// Orientation of the rule
    orientation: RuleOrientation,
}

/// Orientation of a rule line.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuleOrientation {
    /// Horizontal line (default)
    Horizontal,
    /// Vertical line
    Vertical,
}

impl Default for RuleOrientation {
    fn default() -> Self {
        Self::Horizontal
    }
}

impl Rule {
    /// Create a new horizontal rule.
    pub fn new() -> Self {
        Self {
            title: None,
            style: Style::default(),
            border_style: BorderStyle::Single,
            align: Align::Left,
            vertical_align: VerticalAlign::Middle,
            character: None,
            orientation: RuleOrientation::Horizontal,
        }
    }

    /// Create a new horizontal rule with a title.
    pub fn with_title(title: impl Into<Text>) -> Self {
        Self {
            title: Some(title.into()),
            ..Self::new()
        }
    }

    /// Create a new vertical rule.
    pub fn vertical() -> Self {
        Self {
            orientation: RuleOrientation::Vertical,
            ..Self::new()
        }
    }

    /// Create a new vertical rule with a title.
    pub fn vertical_with_title(title: impl Into<Text>) -> Self {
        Self {
            title: Some(title.into()),
            orientation: RuleOrientation::Vertical,
            ..Self::new()
        }
    }

    /// Set the style for the rule.
    pub fn with_style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    /// Set the border style for the rule line.
    pub fn with_border_style(mut self, border_style: BorderStyle) -> Self {
        self.border_style = border_style;
        self
    }

    /// Set the alignment of the rule.
    pub fn with_align(mut self, align: Align) -> Self {
        self.align = align;
        self
    }

    /// Set the vertical alignment of the rule.
    pub fn with_vertical_align(mut self, vertical_align: VerticalAlign) -> Self {
        self.vertical_align = vertical_align;
        self
    }

    /// Set a custom character for the rule line.
    pub fn with_character(mut self, character: char) -> Self {
        self.character = Some(character);
        self
    }

    /// Set the orientation of the rule.
    pub fn with_orientation(mut self, orientation: RuleOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    /// Get the character to use for drawing the rule.
    fn get_rule_character(&self) -> char {
        if let Some(character) = self.character {
            return character;
        }

        let chars = self.border_style.chars().unwrap_or_else(BoxChars::single);
        match self.orientation {
            RuleOrientation::Horizontal => chars.horizontal,
            RuleOrientation::Vertical => chars.vertical,
        }
    }

    /// Render a horizontal rule.
    fn render_horizontal(&self, console: &Console, options: &ConsoleOptions) -> RuleRenderResult {
        let mut segments = Vec::new();
        let available_width = options.max_width.unwrap_or(console.width());

        if available_width == 0 {
            return Ok(segments);
        }

        let rule_char = self.get_rule_character();

        match &self.title {
            Some(title) => {
                // Render rule with title
                let title_segments = title.render(console, options)?;
                let title_width = title_segments
                    .iter()
                    .map(|s| s.text().chars().count())
                    .sum::<usize>();

                if title_width >= available_width {
                    // Title too wide, just show the title
                    return Ok(title_segments);
                }

                let remaining_width = available_width - title_width;
                let left_rule_width = match self.align {
                    Align::Left => 0,
                    Align::Center => remaining_width / 2,
                    Align::Right => remaining_width,
                    Align::Justify => remaining_width / 2, // Same as center for rules
                };
                let right_rule_width = remaining_width - left_rule_width;

                // Left rule segment
                if left_rule_width > 0 {
                    let left_rule = rule_char.to_string().repeat(left_rule_width);
                    segments.push(Segment::new(left_rule, self.style.clone()));
                }

                // Title segments
                segments.extend(title_segments);

                // Right rule segment
                if right_rule_width > 0 {
                    let right_rule = rule_char.to_string().repeat(right_rule_width);
                    segments.push(Segment::new(right_rule, self.style.clone()));
                }
            }
            None => {
                // Simple rule without title
                let rule_line = rule_char.to_string().repeat(available_width);
                segments.push(Segment::new(rule_line, self.style.clone()));
            }
        }

        Ok(segments)
    }

    /// Render a vertical rule.
    fn render_vertical(&self, console: &Console, options: &ConsoleOptions) -> RuleRenderResult {
        let mut segments = Vec::new();
        let available_height = 10; // Default height for vertical rules, console.height() not available

        if available_height == 0 {
            return Ok(segments);
        }

        let rule_char = self.get_rule_character();

        match &self.title {
            Some(title) => {
                // For vertical rules with titles, we need to place the title in the middle
                // and draw vertical lines above and below
                let title_segments = title.render(console, options)?;
                let title_height = 1; // Assume title takes one line

                if title_height >= available_height {
                    // Not enough space for vertical lines, just show title
                    return Ok(title_segments);
                }

                let remaining_height = available_height - title_height;
                let top_rule_height = match self.vertical_align {
                    VerticalAlign::Top => 0,
                    VerticalAlign::Middle => remaining_height / 2,
                    VerticalAlign::Bottom => remaining_height,
                };
                let bottom_rule_height = remaining_height - top_rule_height;

                // Top vertical segments
                for _ in 0..top_rule_height {
                    segments.push(Segment::new(rule_char.to_string(), self.style.clone()));
                    segments.push(Segment::control(
                        crate::segment::ControlCode::CarriageReturn,
                    ));
                }

                // Title
                segments.extend(title_segments);
                if bottom_rule_height > 0 {
                    segments.push(Segment::control(
                        crate::segment::ControlCode::CarriageReturn,
                    ));
                }

                // Bottom vertical segments
                for i in 0..bottom_rule_height {
                    segments.push(Segment::new(rule_char.to_string(), self.style.clone()));
                    if i < bottom_rule_height - 1 {
                        segments.push(Segment::control(
                            crate::segment::ControlCode::CarriageReturn,
                        ));
                    }
                }
            }
            None => {
                // Simple vertical rule without title
                for i in 0..available_height {
                    segments.push(Segment::new(rule_char.to_string(), self.style.clone()));
                    if i < available_height - 1 {
                        segments.push(Segment::control(
                            crate::segment::ControlCode::CarriageReturn,
                        ));
                    }
                }
            }
        }

        Ok(segments)
    }
}

impl Default for Rule {
    fn default() -> Self {
        Self::new()
    }
}

impl Renderable for Rule {
    fn render(&self, console: &Console, options: &ConsoleOptions) -> RuleRenderResult {
        match self.orientation {
            RuleOrientation::Horizontal => self.render_horizontal(console, options),
            RuleOrientation::Vertical => self.render_vertical(console, options),
        }
    }
}

impl Measurable for Rule {
    fn measure(
        &self,
        console: &Console,
        options: &ConsoleOptions,
    ) -> Result<Measurement, LuxorError> {
        match self.orientation {
            RuleOrientation::Horizontal => {
                let available_width = options.max_width.unwrap_or(console.width());

                let min_width = match &self.title {
                    Some(title) => {
                        let title_measurement = title.measure(console, options)?;
                        title_measurement.minimum()
                    }
                    None => 1, // At least one character for the rule
                };

                Ok(Measurement::new(min_width, available_width))
            }
            RuleOrientation::Vertical => {
                let _available_height = 10; // Default height for vertical rules, console.height() not available

                let _min_height = match &self.title {
                    Some(_) => 1, // At least one line for the title
                    None => 1,    // At least one line for the rule
                };

                // For vertical rules, width is always 1 character
                Ok(Measurement::new(1, 1))
            }
        }
    }
}

/// Create a horizontal rule that spans the full width.
pub fn horizontal_rule() -> Rule {
    Rule::new()
}

/// Create a horizontal rule with a title.
pub fn horizontal_rule_with_title(title: impl Into<Text>) -> Rule {
    Rule::with_title(title)
}

/// Create a vertical rule that spans the full height.
pub fn vertical_rule() -> Rule {
    Rule::vertical()
}

/// Create a vertical rule with a title.
pub fn vertical_rule_with_title(title: impl Into<Text>) -> Rule {
    Rule::vertical_with_title(title)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        color::Color,
        console::{Console, ConsoleOptions},
    };

    fn test_console() -> Console {
        Console::new()
    }

    #[test]
    fn test_rule_creation() {
        let rule = Rule::new();
        assert_eq!(rule.orientation, RuleOrientation::Horizontal);
        assert!(rule.title.is_none());
        assert_eq!(rule.border_style, BorderStyle::Single);
    }

    #[test]
    fn test_rule_with_title() {
        let rule = Rule::with_title("Test Title");
        assert!(rule.title.is_some());
        assert_eq!(rule.title.unwrap().plain(), "Test Title");
    }

    #[test]
    fn test_vertical_rule() {
        let rule = Rule::vertical();
        assert_eq!(rule.orientation, RuleOrientation::Vertical);
    }

    #[test]
    fn test_rule_character() {
        let rule = Rule::new();
        assert_eq!(rule.get_rule_character(), '─'); // Single border horizontal

        let rule = Rule::new().with_border_style(BorderStyle::Double);
        assert_eq!(rule.get_rule_character(), '═'); // Double border horizontal

        let rule = Rule::new().with_character('*');
        assert_eq!(rule.get_rule_character(), '*'); // Custom character
    }

    #[test]
    fn test_horizontal_rule_render() {
        let console = test_console();
        let options = ConsoleOptions::new().with_max_width(10);

        let rule = Rule::new();
        let segments = rule.render(&console, &options).unwrap();

        assert_eq!(segments.len(), 1);
        assert_eq!(segments[0].text(), "──────────"); // 10 dashes
    }

    #[test]
    fn test_horizontal_rule_with_title_render() {
        let console = test_console();
        let options = ConsoleOptions::new().with_max_width(15);

        let rule = Rule::with_title("Test").with_align(Align::Center);
        let segments = rule.render(&console, &options).unwrap();

        // Should have: left rule + title + right rule
        assert!(segments.len() >= 2);

        // Find title segment
        let title_found = segments.iter().any(|s| s.text().contains("Test"));
        assert!(title_found);
    }

    #[test]
    fn test_vertical_rule_render() {
        let console = test_console();
        let options = ConsoleOptions::default();

        let rule = Rule::vertical();
        let segments = rule.render(&console, &options).unwrap();

        // Should have 5 vertical characters with line breaks
        let vertical_chars = segments.iter().filter(|s| s.text() == "│").count();
        assert!(vertical_chars > 0); // Should have some vertical characters
    }

    #[test]
    fn test_rule_measurement() {
        let console = test_console();
        let options = ConsoleOptions::new().with_max_width(20);

        let rule = Rule::new();
        let measurement = rule.measure(&console, &options).unwrap();

        assert_eq!(measurement.minimum(), 1);
        assert_eq!(measurement.maximum(), 20);
    }

    #[test]
    fn test_rule_with_title_measurement() {
        let console = test_console();
        let options = ConsoleOptions::new().with_max_width(20);

        let rule = Rule::with_title("Hello World");
        let measurement = rule.measure(&console, &options).unwrap();

        // Minimum should be at least the title length
        assert!(measurement.minimum() >= 11); // "Hello World" length
    }

    #[test]
    fn test_vertical_rule_measurement() {
        let console = test_console();
        let options = ConsoleOptions::default();

        let rule = Rule::vertical();
        let measurement = rule.measure(&console, &options).unwrap();

        // Vertical rules always have width 1
        assert_eq!(measurement.minimum(), 1);
        assert_eq!(measurement.maximum(), 1);
    }

    #[test]
    fn test_rule_styling() {
        let console = test_console();
        let options = ConsoleOptions::new().with_max_width(5);

        let rule = Rule::new()
            .with_style(Style::new().color(Color::rgb(255, 0, 0)).bold())
            .with_character('=');

        let segments = rule.render(&console, &options).unwrap();

        assert_eq!(segments[0].text(), "=====");
        assert_eq!(segments[0].style().color, Some(Color::rgb(255, 0, 0)));
        assert_eq!(segments[0].style().bold, Some(true));
    }

    #[test]
    fn test_rule_helper_functions() {
        let h_rule = horizontal_rule();
        assert_eq!(h_rule.orientation, RuleOrientation::Horizontal);

        let h_rule_title = horizontal_rule_with_title("Test");
        assert!(h_rule_title.title.is_some());

        let v_rule = vertical_rule();
        assert_eq!(v_rule.orientation, RuleOrientation::Vertical);

        let v_rule_title = vertical_rule_with_title("Test");
        assert!(v_rule_title.title.is_some());
        assert_eq!(v_rule_title.orientation, RuleOrientation::Vertical);
    }

    #[test]
    fn test_rule_alignment() {
        let console = test_console();
        let options = ConsoleOptions::new().with_max_width(20);

        // Test different alignments with title
        let rule = Rule::with_title("Hi").with_align(Align::Center);
        let segments = rule.render(&console, &options).unwrap();

        // Should have rule parts on both sides of title
        assert!(segments.len() >= 2);
    }

    #[test]
    fn test_rule_edge_cases() {
        let console = test_console();

        // Zero width
        let options = ConsoleOptions::new().with_max_width(0);
        let rule = Rule::new();
        let segments = rule.render(&console, &options).unwrap();
        assert!(segments.is_empty());

        // Title wider than available space
        let options = ConsoleOptions::new().with_max_width(5);
        let rule = Rule::with_title("This is a very long title");
        let segments = rule.render(&console, &options).unwrap();
        // Should still render something (just the title, truncated by the title renderer)
        assert!(!segments.is_empty());
    }
}
