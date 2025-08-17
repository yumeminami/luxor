//! Rich text implementation with style spans and markup support.

use crate::{
    console::{Console, ConsoleOptions},
    error::{LuxorError, Result},
    markup::Span,
    measure::Measurement,
    protocol::{Measurable, Renderable},
    segment::Segment,
    style::Style,
};
use std::ops::Range;
use unicode_width::UnicodeWidthStr;

/// A rich text object that supports styled spans within the text.
///
/// Text can contain multiple style spans that apply different formatting
/// to different portions of the text content.
#[derive(Debug, Clone)]
pub struct Text {
    /// The plain text content
    content: String,
    /// Base style applied to the entire text
    base_style: Style,
    /// Style spans that apply to portions of the text
    spans: Vec<Span>,
}

impl Text {
    /// Create new text with the given content.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use luxor::Text;
    ///
    /// let text = Text::new("Hello, world!");
    /// assert_eq!(text.plain(), "Hello, world!");
    /// ```
    pub fn new(content: &str) -> Self {
        Self {
            content: content.to_string(),
            base_style: Style::default(),
            spans: Vec::new(),
        }
    }

    /// Create text with an initial base style.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use luxor::{Text, Style, Color};
    ///
    /// let style = Style::new().color(Color::rgb(255, 0, 0));
    /// let text = Text::new("Hello").with_style(style);
    /// ```
    pub fn with_style(mut self, style: Style) -> Self {
        self.base_style = style;
        self
    }

    /// Set the base style for this text.
    pub fn set_style(&mut self, style: Style) {
        self.base_style = style;
    }

    /// Get the plain text content without any styling.
    pub fn plain(&self) -> &str {
        &self.content
    }

    /// Get the base style applied to the entire text.
    pub fn base_style(&self) -> &Style {
        &self.base_style
    }

    /// Get all style spans in this text.
    pub fn spans(&self) -> &[Span] {
        &self.spans
    }

    /// Get a mutable reference to the spans.
    pub fn spans_mut(&mut self) -> &mut Vec<Span> {
        &mut self.spans
    }

    /// Get the length of the text in characters.
    pub fn len(&self) -> usize {
        self.content.chars().count()
    }

    /// Check if the text is empty.
    pub fn is_empty(&self) -> bool {
        self.content.is_empty()
    }

    /// Get the display width of the text (considering Unicode width).
    pub fn width(&self) -> usize {
        self.content.width()
    }

    /// Apply a style to a range of characters.
    ///
    /// # Arguments
    ///
    /// * `range` - The character range to style (start..end)
    /// * `style` - The style to apply
    ///
    /// # Examples
    ///
    /// ```rust
    /// use luxor::{Text, Style, Color};
    ///
    /// let mut text = Text::new("Hello world");
    /// let style = Style::new().color(Color::rgb(255, 0, 0)).bold();
    /// text.stylize_range(0..5, style).unwrap(); // Make "Hello" red and bold
    /// ```
    pub fn stylize_range(&mut self, range: Range<usize>, style: Style) -> Result<()> {
        let start = range.start;
        let end = range.end;

        // Validate range
        if start > end {
            return Err(LuxorError::InvalidRange(format!(
                "Start index {} is greater than end index {}",
                start, end
            )));
        }

        if end > self.len() {
            return Err(LuxorError::InvalidRange(format!(
                "End index {} is out of bounds for text of length {}",
                end,
                self.len()
            )));
        }

        // Add the new span
        let span = Span::new(start, end, style);
        self.spans.push(span);

        // Sort spans by start position for consistent rendering
        self.spans.sort_by_key(|s| s.start);

        Ok(())
    }

    /// Apply a style to the entire text.
    ///
    /// This is equivalent to calling `stylize_range(0..text.len(), style)`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use luxor::{Text, Style, Color};
    ///
    /// let mut text = Text::new("Hello world");
    /// let style = Style::new().bold();
    /// text.stylize_all(style).unwrap();
    /// ```
    pub fn stylize_all(&mut self, style: Style) -> Result<()> {
        let len = self.len();
        self.stylize_range(0..len, style)
    }

    /// Append plain text to this text object.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use luxor::Text;
    ///
    /// let mut text = Text::new("Hello");
    /// text.append(" world");
    /// assert_eq!(text.plain(), "Hello world");
    /// ```
    pub fn append(&mut self, text: &str) {
        self.content.push_str(text);
        // No need to adjust spans since we're only adding at the end
    }

    /// Append another Text object to this one, preserving its spans.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use luxor::{Text, Style, Color};
    ///
    /// let mut text1 = Text::new("Hello ");
    /// let mut text2 = Text::new("world");
    /// text2.stylize_all(Style::new().color(Color::rgb(255, 0, 0))).unwrap();
    ///
    /// text1.append_text(text2);
    /// assert_eq!(text1.plain(), "Hello world");
    /// assert_eq!(text1.spans().len(), 1); // Should have the red span for "world"
    /// ```
    pub fn append_text(&mut self, other: Text) {
        let offset = self.len(); // Use character count, not byte count
        self.content.push_str(&other.content);

        // Adjust and add spans from the other text
        for span in other.spans {
            let adjusted_span =
                Span::new(span.start + offset, span.end + offset, span.style.clone());
            self.spans.push(adjusted_span);
        }

        // Re-sort spans
        self.spans.sort_by_key(|s| s.start);
    }

    /// Create text from markup string.
    ///
    /// This is a convenience method that uses the markup parser to create
    /// a Text instance with appropriate style spans.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use luxor::Text;
    ///
    /// let text = Text::from_markup("[bold]Hello[/bold] [red]world[/red]").unwrap();
    /// assert_eq!(text.plain(), "Hello world");
    /// assert_eq!(text.spans().len(), 2);
    /// ```
    pub fn from_markup(markup: &str) -> Result<Self> {
        crate::markup::render(markup, None)
    }

    /// Create text from markup with a base style.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use luxor::{Text, Style, Color};
    ///
    /// let base_style = Style::new().color(Color::rgb(0, 0, 255));
    /// let text = Text::from_markup_with_style("[bold]Hello[/bold]", base_style).unwrap();
    /// ```
    pub fn from_markup_with_style(markup: &str, base_style: Style) -> Result<Self> {
        crate::markup::render(markup, Some(base_style))
    }

    /// Get the style at a specific character position.
    ///
    /// This combines the base style with any applicable span styles.
    pub fn style_at(&self, position: usize) -> Style {
        let mut combined_style = self.base_style.clone();

        // Apply all spans that contain this position
        for span in &self.spans {
            if position >= span.start && position < span.end {
                combined_style = combined_style.combine(span.style.clone());
            }
        }

        combined_style
    }

    /// Split the text into segments for rendering.
    ///
    /// This method analyzes all the style spans and creates segments
    /// where each segment has consistent styling.
    pub fn to_segments(&self) -> Vec<Segment> {
        if self.content.is_empty() {
            return vec![Segment::new(String::new(), self.base_style.clone())];
        }

        if self.spans.is_empty() {
            // No spans, just return a single segment with base style
            return vec![Segment::new(self.content.clone(), self.base_style.clone())];
        }

        let mut segments = Vec::new();
        let mut events = Vec::new();

        // Create events for span starts and ends
        for span in &self.spans {
            events.push((span.start, true, span)); // Start event
            events.push((span.end, false, span)); // End event
        }

        // Sort events by position, with end events before start events at the same position
        events.sort_by_key(|(pos, is_start, _)| (*pos, *is_start));

        let mut current_position = 0;
        let mut active_spans: Vec<&Span> = Vec::new();

        for (position, is_start, span) in events {
            // Create segment for text before this event
            if position > current_position {
                let text_slice = self.get_char_slice(current_position, position);
                let style = self.compute_style_for_spans(&active_spans);
                if !text_slice.is_empty() {
                    segments.push(Segment::new(text_slice, style));
                }
                current_position = position;
            }

            // Update active spans
            if is_start {
                active_spans.push(span);
            } else {
                active_spans.retain(|s| s != &span);
            }
        }

        // Handle remaining text after last event
        if current_position < self.len() {
            let text_slice = self.get_char_slice(current_position, self.len());
            let style = self.compute_style_for_spans(&active_spans);
            if !text_slice.is_empty() {
                segments.push(Segment::new(text_slice, style));
            }
        }

        segments
    }

    /// Get a slice of the text by character positions.
    fn get_char_slice(&self, start: usize, end: usize) -> String {
        self.content.chars().skip(start).take(end - start).collect()
    }

    /// Compute the combined style for a set of active spans.
    fn compute_style_for_spans(&self, spans: &[&Span]) -> Style {
        let mut style = self.base_style.clone();
        for span in spans {
            style = style.combine(span.style.clone());
        }
        style
    }
}

impl Renderable for Text {
    fn render(&self, _console: &Console, _options: &ConsoleOptions) -> Result<Vec<Segment>> {
        Ok(self.to_segments())
    }
}

impl Measurable for Text {
    fn measure(&self, _console: &Console, _options: &ConsoleOptions) -> Result<Measurement> {
        let width = self.width();
        Ok(Measurement::fixed(width))
    }
}

impl std::fmt::Display for Text {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.content)
    }
}

impl From<&str> for Text {
    fn from(content: &str) -> Self {
        Text::new(content)
    }
}

impl From<String> for Text {
    fn from(content: String) -> Self {
        Text::new(&content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Style, color::Color};

    #[test]
    fn test_text_creation() {
        let text = Text::new("Hello world");
        assert_eq!(text.plain(), "Hello world");
        assert_eq!(text.len(), 11);
        assert_eq!(text.width(), 11);
        assert!(text.spans().is_empty());
    }

    #[test]
    fn test_text_with_style() {
        let style = Style::new().bold();
        let text = Text::new("Hello").with_style(style.clone());
        assert_eq!(text.base_style(), &style);
    }

    #[test]
    fn test_stylize_range() {
        let mut text = Text::new("Hello world");
        let style = Style::new().color(Color::rgb(255, 0, 0));

        text.stylize_range(0..5, style.clone()).unwrap();
        assert_eq!(text.spans().len(), 1);

        let span = &text.spans()[0];
        assert_eq!(span.start, 0);
        assert_eq!(span.end, 5);
        assert_eq!(span.style.color, Some(Color::rgb(255, 0, 0)));
    }

    #[test]
    fn test_stylize_range_invalid() {
        let mut text = Text::new("Hello");
        let style = Style::new().bold();

        // End > length
        assert!(text.stylize_range(0..10, style.clone()).is_err());

        // Start > end
        assert!(text.stylize_range(5..2, style.clone()).is_err());
    }

    #[test]
    fn test_stylize_all() {
        let mut text = Text::new("Hello world");
        let style = Style::new().italic();

        text.stylize_all(style.clone()).unwrap();
        assert_eq!(text.spans().len(), 1);

        let span = &text.spans()[0];
        assert_eq!(span.start, 0);
        assert_eq!(span.end, 11);
        assert_eq!(span.style.italic, Some(true));
    }

    #[test]
    fn test_append() {
        let mut text = Text::new("Hello");
        text.append(" world");
        assert_eq!(text.plain(), "Hello world");
        assert_eq!(text.len(), 11);
    }

    #[test]
    fn test_append_text() {
        let mut text1 = Text::new("Hello ");
        let mut text2 = Text::new("world");
        text2
            .stylize_all(Style::new().color(Color::rgb(255, 0, 0)))
            .unwrap();

        text1.append_text(text2);
        assert_eq!(text1.plain(), "Hello world");
        assert_eq!(text1.spans().len(), 1);

        let span = &text1.spans()[0];
        assert_eq!(span.start, 6); // "world" starts at position 6
        assert_eq!(span.end, 11);
        assert_eq!(span.style.color, Some(Color::rgb(255, 0, 0)));
    }

    #[test]
    fn test_style_at() {
        let mut text = Text::new("Hello world");
        let base_style = Style::new().color(Color::rgb(0, 0, 255));
        text.set_style(base_style.clone());

        // Add a bold span for "Hello"
        text.stylize_range(0..5, Style::new().bold()).unwrap();

        // Check style at different positions
        let style_at_0 = text.style_at(0);
        assert_eq!(style_at_0.color, Some(Color::rgb(0, 0, 255)));
        assert_eq!(style_at_0.bold, Some(true));

        let style_at_7 = text.style_at(7); // In "world"
        assert_eq!(style_at_7.color, Some(Color::rgb(0, 0, 255)));
        assert_eq!(style_at_7.bold, None); // No bold in this part
    }

    #[test]
    fn test_to_segments_no_spans() {
        let text = Text::new("Hello world");
        let segments = text.to_segments();

        assert_eq!(segments.len(), 1);
        assert_eq!(segments[0].text(), "Hello world");
    }

    #[test]
    fn test_to_segments_with_spans() {
        let mut text = Text::new("Hello world");
        text.stylize_range(0..5, Style::new().bold()).unwrap();
        text.stylize_range(6..11, Style::new().color(Color::rgb(255, 0, 0)))
            .unwrap();

        let segments = text.to_segments();
        assert_eq!(segments.len(), 3); // "Hello", " ", "world"

        assert_eq!(segments[0].text(), "Hello");
        assert_eq!(segments[0].style().bold, Some(true));

        assert_eq!(segments[1].text(), " ");

        assert_eq!(segments[2].text(), "world");
        assert_eq!(segments[2].style().color, Some(Color::rgb(255, 0, 0)));
    }

    #[test]
    fn test_overlapping_spans() {
        let mut text = Text::new("Hello world");
        text.stylize_range(0..8, Style::new().bold()).unwrap(); // "Hello wo"
        text.stylize_range(6..11, Style::new().color(Color::rgb(255, 0, 0)))
            .unwrap(); // "world"

        let segments = text.to_segments();

        // Should have segments for different style combinations
        assert!(segments.len() >= 3);

        // Find the overlapping segment (should be both bold and red)
        let overlapping_segment = segments.iter().find(|s| {
            s.style().bold == Some(true) && s.style().color == Some(Color::rgb(255, 0, 0))
        });
        assert!(overlapping_segment.is_some());
    }

    #[test]
    fn test_from_markup() {
        let text = Text::from_markup("[bold]Hello[/bold] world").unwrap();
        assert_eq!(text.plain(), "Hello world");
        assert_eq!(text.spans().len(), 1);

        let span = &text.spans()[0];
        assert_eq!(span.start, 0);
        assert_eq!(span.end, 5);
        assert_eq!(span.style.bold, Some(true));
    }

    #[test]
    fn test_display_trait() {
        let text = Text::new("Hello world");
        assert_eq!(format!("{}", text), "Hello world");
    }

    #[test]
    fn test_from_str() {
        let text: Text = "Hello world".into();
        assert_eq!(text.plain(), "Hello world");
    }

    #[test]
    fn test_from_string() {
        let text: Text = String::from("Hello world").into();
        assert_eq!(text.plain(), "Hello world");
    }
}
