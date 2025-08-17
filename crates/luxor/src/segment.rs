//! Segment - the fundamental rendering unit for rich text.

use crate::{ColorSystem, Style, ansi};
use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

/// Control codes for terminal operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ControlCode {
    /// Ring the terminal bell.
    Bell,
    /// Carriage return.
    CarriageReturn,
    /// Move cursor to home position.
    Home,
    /// Clear screen.
    Clear,
    /// Show cursor.
    ShowCursor,
    /// Hide cursor.
    HideCursor,
    /// Enable alternative screen buffer.
    EnableAltScreen,
    /// Disable alternative screen buffer.
    DisableAltScreen,
    /// Move cursor up by specified lines.
    CursorUp(usize),
    /// Move cursor down by specified lines.
    CursorDown(usize),
    /// Move cursor forward by specified columns.
    CursorForward(usize),
    /// Move cursor backward by specified columns.
    CursorBackward(usize),
    /// Move cursor to specific column (1-indexed).
    CursorMoveToColumn(usize),
    /// Move cursor to specific position (row, col) - both 1-indexed.
    CursorMoveTo { row: usize, col: usize },
}

impl ControlCode {
    /// Generate the ANSI escape sequence for this control code.
    pub fn to_ansi(self) -> String {
        match self {
            ControlCode::Bell => "\x07".to_string(),
            ControlCode::CarriageReturn => "\r".to_string(),
            ControlCode::Home => ansi::codes::CURSOR_HOME.to_string(),
            ControlCode::Clear => ansi::codes::CLEAR_SCREEN.to_string(),
            ControlCode::ShowCursor => ansi::codes::CURSOR_SHOW.to_string(),
            ControlCode::HideCursor => ansi::codes::CURSOR_HIDE.to_string(),
            ControlCode::EnableAltScreen => ansi::codes::ALT_SCREEN_ENABLE.to_string(),
            ControlCode::DisableAltScreen => ansi::codes::ALT_SCREEN_DISABLE.to_string(),
            ControlCode::CursorUp(n) => ansi::cursor::up(n),
            ControlCode::CursorDown(n) => ansi::cursor::down(n),
            ControlCode::CursorForward(n) => ansi::cursor::right(n),
            ControlCode::CursorBackward(n) => ansi::cursor::left(n),
            ControlCode::CursorMoveToColumn(col) => ansi::cursor::column(col),
            ControlCode::CursorMoveTo { row, col } => ansi::cursor::position(row, col),
        }
    }
}

/// A segment represents a piece of text with associated styling.
///
/// Segments are the fundamental rendering units in Luxor. They contain text,
/// style information, and optional control codes for terminal operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Segment {
    /// The text content of this segment.
    text: String,
    /// The style to apply to this segment.
    style: Style,
    /// Optional control code for terminal operations.
    control: Option<ControlCode>,
}

impl Segment {
    /// Create a new text segment with the given content and style.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use luxor::{Segment, Style, Color};
    ///
    /// let segment = Segment::new("Hello".to_string(), Style::new().color(Color::rgb(255, 0, 0)));
    /// assert_eq!(segment.text(), "Hello");
    /// ```
    pub fn new(text: String, style: Style) -> Self {
        Self {
            text,
            style,
            control: None,
        }
    }

    /// Create a new control segment with a control code.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use luxor::{Segment, ControlCode};
    ///
    /// let segment = Segment::control(ControlCode::Clear);
    /// assert!(segment.is_control());
    /// ```
    pub fn control(control: ControlCode) -> Self {
        Self {
            text: String::new(),
            style: Style::default(),
            control: Some(control),
        }
    }

    /// Create a segment with both text and a control code.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use luxor::{Segment, Style, ControlCode};
    ///
    /// let segment = Segment::with_control(
    ///     "Text".to_string(),
    ///     Style::new(),
    ///     ControlCode::Bell
    /// );
    /// ```
    pub fn with_control(text: String, style: Style, control: ControlCode) -> Self {
        Self {
            text,
            style,
            control: Some(control),
        }
    }

    /// Get the text content of this segment.
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Get the style of this segment.
    pub fn style(&self) -> &Style {
        &self.style
    }

    /// Get the control code of this segment, if any.
    pub fn get_control(&self) -> Option<ControlCode> {
        self.control
    }

    /// Check if this segment contains only a control code (no text).
    pub fn is_control(&self) -> bool {
        self.control.is_some() && self.text.is_empty()
    }

    /// Check if this segment contains only text (no control code).
    pub fn is_text(&self) -> bool {
        self.control.is_none() && !self.text.is_empty()
    }

    /// Check if this segment is empty (no text and no control code).
    pub fn is_empty(&self) -> bool {
        self.text.is_empty() && self.control.is_none()
    }

    /// Get the display width of this segment.
    ///
    /// This calculates the number of terminal columns this segment will occupy,
    /// taking into account Unicode character widths. Control codes have zero width.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use luxor::{Segment, Style};
    ///
    /// let segment = Segment::new("Hello".to_string(), Style::new());
    /// assert_eq!(segment.cell_length(), 5);
    /// ```
    pub fn cell_length(&self) -> usize {
        if self.is_control() {
            0
        } else {
            self.text.width()
        }
    }

    /// Split this segment at the given character position.
    ///
    /// Returns a tuple of (left_segment, right_segment). If the position is
    /// out of bounds, returns the original segment and an empty segment.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use luxor::{Segment, Style};
    ///
    /// let segment = Segment::new("Hello World".to_string(), Style::new());
    /// let (left, right) = segment.split_at_char(5);
    /// assert_eq!(left.text(), "Hello");
    /// assert_eq!(right.text(), " World");
    /// ```
    pub fn split_at_char(self, pos: usize) -> (Self, Self) {
        if pos >= self.text.len() {
            return (self, Segment::new(String::new(), Style::default()));
        }

        // Handle control codes - they stay with the left segment
        if self.is_control() {
            return (self, Segment::new(String::new(), Style::default()));
        }

        // Find the byte position for the character position
        let mut byte_pos = 0;
        let mut char_count = 0;

        for (byte_idx, _) in self.text.char_indices() {
            if char_count == pos {
                byte_pos = byte_idx;
                break;
            }
            char_count += 1;
        }

        if char_count == pos {
            let (left_text, right_text) = self.text.split_at(byte_pos);
            let left = Self {
                text: left_text.to_string(),
                style: self.style.clone(),
                control: self.control,
            };
            let right = Self {
                text: right_text.to_string(),
                style: self.style,
                control: None,
            };
            (left, right)
        } else {
            (self, Segment::new(String::new(), Style::default()))
        }
    }

    /// Split this segment to fit within the given display width.
    ///
    /// Returns a tuple of (left_segment, right_segment) where the left segment
    /// has a display width <= max_width. Uses Unicode-aware width calculation.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use luxor::{Segment, Style};
    ///
    /// let segment = Segment::new("Hello World".to_string(), Style::new());
    /// let (left, right) = segment.split_at_width(5);
    /// assert_eq!(left.text(), "Hello");
    /// assert_eq!(right.text(), " World");
    /// ```
    pub fn split_at_width(self, max_width: usize) -> (Self, Self) {
        if self.is_control() {
            return (self, Segment::new(String::new(), Style::default()));
        }

        let mut current_width = 0;
        let mut split_pos = 0;

        for (char_idx, ch) in self.text.char_indices() {
            let char_width = ch.width().unwrap_or(0);
            if current_width + char_width > max_width {
                break;
            }
            current_width += char_width;
            split_pos = char_idx + ch.len_utf8();
        }

        if split_pos == 0 {
            // Can't fit any characters
            (Segment::new(String::new(), self.style.clone()), self)
        } else if split_pos >= self.text.len() {
            // Entire segment fits
            (self, Segment::new(String::new(), Style::default()))
        } else {
            let (left_text, right_text) = self.text.split_at(split_pos);
            let left = Self {
                text: left_text.to_string(),
                style: self.style.clone(),
                control: self.control,
            };
            let right = Self {
                text: right_text.to_string(),
                style: self.style,
                control: None,
            };
            (left, right)
        }
    }

    /// Apply a style to this segment by combining it with the existing style.
    ///
    /// The provided style will override any conflicting attributes in the
    /// segment's current style.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use luxor::{Segment, Style, Color};
    ///
    /// let mut segment = Segment::new("Hello".to_string(), Style::new().bold());
    /// segment.apply_style(Style::new().color(Color::rgb(255, 0, 0)));
    /// // Segment now has both bold and red color
    /// ```
    pub fn apply_style(&mut self, style: Style) {
        self.style = self.style.clone().combine(style);
    }

    /// Render this segment to a string with ANSI escape sequences.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use luxor::{Segment, Style, Color, ColorSystem};
    ///
    /// let segment = Segment::new(
    ///     "Hello".to_string(),
    ///     Style::new().bold().color(Color::rgb(255, 0, 0))
    /// );
    /// let output = segment.render(ColorSystem::TrueColor);
    /// // Output includes ANSI codes for bold red text
    /// ```
    pub fn render(&self, color_system: ColorSystem) -> String {
        let mut output = String::new();

        // Add control code if present
        if let Some(control) = self.control {
            output.push_str(&control.to_ansi());
        }

        // Add styled text if present
        if !self.text.is_empty() {
            let style_ansi = ansi::style_to_ansi(&self.style, color_system);
            if !style_ansi.is_empty() {
                output.push_str(&style_ansi);
                output.push_str(&self.text);
                output.push_str(ansi::RESET);
            } else {
                output.push_str(&self.text);
            }
        }

        output
    }

    /// Get the plain text content without any styling or control codes.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use luxor::{Segment, Style, ControlCode};
    ///
    /// let segment = Segment::with_control(
    ///     "Hello".to_string(),
    ///     Style::new().bold(),
    ///     ControlCode::Bell
    /// );
    /// assert_eq!(segment.plain_text(), "Hello");
    /// ```
    pub fn plain_text(&self) -> &str {
        &self.text
    }
}

/// A collection of segments that can be efficiently joined and manipulated.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Segments(Vec<Segment>);

impl Segments {
    /// Create a new empty segments collection.
    pub fn new() -> Self {
        Self(Vec::new())
    }

    /// Create segments from a vector of segments.
    pub fn from_vec(segments: Vec<Segment>) -> Self {
        Self(segments)
    }

    /// Add a segment to the collection.
    pub fn push(&mut self, segment: Segment) {
        self.0.push(segment);
    }

    /// Get the segments as a slice.
    pub fn as_slice(&self) -> &[Segment] {
        &self.0
    }

    /// Get a mutable slice of segments.
    pub fn as_mut_slice(&mut self) -> &mut [Segment] {
        &mut self.0
    }

    /// Convert to a vector of segments.
    pub fn into_vec(self) -> Vec<Segment> {
        self.0
    }

    /// Get the total display width of all segments.
    pub fn cell_length(&self) -> usize {
        self.0.iter().map(|s| s.cell_length()).sum()
    }

    /// Check if the segments collection is empty.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Get the number of segments.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Render all segments to a string with ANSI escape sequences.
    pub fn render(&self, color_system: ColorSystem) -> String {
        self.0.iter().map(|s| s.render(color_system)).collect()
    }

    /// Get the plain text content of all segments combined.
    pub fn plain_text(&self) -> String {
        self.0.iter().map(|s| s.plain_text()).collect()
    }
}

impl Default for Segments {
    fn default() -> Self {
        Self::new()
    }
}

impl FromIterator<Segment> for Segments {
    fn from_iter<T: IntoIterator<Item = Segment>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl IntoIterator for Segments {
    type Item = Segment;
    type IntoIter = std::vec::IntoIter<Segment>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Color, Style};

    #[test]
    fn test_segment_new() {
        let style = Style::new().bold();
        let segment = Segment::new("Hello".to_string(), style.clone());

        assert_eq!(segment.text(), "Hello");
        assert_eq!(segment.style(), &style);
        assert_eq!(segment.get_control(), None);
        assert!(segment.is_text());
        assert!(!segment.is_control());
        assert!(!segment.is_empty());
    }

    #[test]
    fn test_segment_control() {
        let segment = Segment::control(ControlCode::Clear);

        assert_eq!(segment.text(), "");
        assert_eq!(segment.get_control(), Some(ControlCode::Clear));
        assert!(!segment.is_text());
        assert!(segment.is_control());
    }

    #[test]
    fn test_segment_cell_length() {
        let segment = Segment::new("Hello".to_string(), Style::new());
        assert_eq!(segment.cell_length(), 5);

        let control_segment = Segment::control(ControlCode::Clear);
        assert_eq!(control_segment.cell_length(), 0);
    }

    #[test]
    fn test_segment_split_at_char() {
        let segment = Segment::new("Hello World".to_string(), Style::new().bold());
        let (left, right) = segment.split_at_char(5);

        assert_eq!(left.text(), "Hello");
        assert_eq!(right.text(), " World");
        assert_eq!(left.style().bold, Some(true));
        assert_eq!(right.style().bold, Some(true));
    }

    #[test]
    fn test_segment_split_at_width() {
        let segment = Segment::new("Hello World".to_string(), Style::new());
        let (left, right) = segment.split_at_width(5);

        assert_eq!(left.text(), "Hello");
        assert_eq!(right.text(), " World");
        assert_eq!(left.cell_length(), 5);
    }

    #[test]
    fn test_segment_apply_style() {
        let mut segment = Segment::new("Hello".to_string(), Style::new().bold());
        segment.apply_style(Style::new().color(Color::rgb(255, 0, 0)));

        assert_eq!(segment.style().bold, Some(true));
        assert_eq!(segment.style().color, Some(Color::rgb(255, 0, 0)));
    }

    #[test]
    fn test_segment_render() {
        let segment = Segment::new(
            "Hello".to_string(),
            Style::new().bold().color(Color::rgb(255, 0, 0)),
        );
        let output = segment.render(ColorSystem::TrueColor);

        assert!(output.contains("Hello"));
        assert!(output.contains("\x1b["));
        assert!(output.contains("\x1b[0m"));
    }

    #[test]
    fn test_control_code_to_ansi() {
        assert_eq!(ControlCode::Bell.to_ansi(), "\x07");
        assert_eq!(ControlCode::Clear.to_ansi(), "\x1b[2J");
        assert_eq!(ControlCode::CursorUp(3).to_ansi(), "\x1b[3A");
    }

    #[test]
    fn test_segments_collection() {
        let mut segments = Segments::new();
        segments.push(Segment::new("Hello".to_string(), Style::new()));
        segments.push(Segment::new(" World".to_string(), Style::new().bold()));

        assert_eq!(segments.len(), 2);
        assert_eq!(segments.cell_length(), 11);
        assert_eq!(segments.plain_text(), "Hello World");
    }

    #[test]
    fn test_segments_from_iter() {
        let vec = vec![
            Segment::new("Hello".to_string(), Style::new()),
            Segment::new(" World".to_string(), Style::new()),
        ];
        let segments: Segments = vec.into_iter().collect();

        assert_eq!(segments.len(), 2);
        assert_eq!(segments.plain_text(), "Hello World");
    }
}
