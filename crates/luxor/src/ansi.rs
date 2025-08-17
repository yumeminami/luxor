//! ANSI escape sequence generation for terminal control.

use crate::{Color, ColorSystem, Style};

/// ANSI escape sequence builder for styling text.
pub struct AnsiBuilder {
    sequences: Vec<String>,
}

impl AnsiBuilder {
    /// Create a new ANSI builder.
    pub fn new() -> Self {
        Self {
            sequences: Vec::new(),
        }
    }

    /// Add a style to the builder.
    ///
    /// This will generate the appropriate ANSI escape sequences for the given style
    /// and color system capability.
    pub fn add_style(&mut self, style: &Style, color_system: ColorSystem) {
        // Foreground color
        if let Some(color) = style.color {
            self.add_color(color, false, color_system);
        }

        // Background color
        if let Some(color) = style.background {
            self.add_color(color, true, color_system);
        }

        // Text attributes
        if let Some(true) = style.bold {
            self.sequences.push("1".to_string());
        }
        if let Some(true) = style.dim {
            self.sequences.push("2".to_string());
        }
        if let Some(true) = style.italic {
            self.sequences.push("3".to_string());
        }
        if let Some(true) = style.underline {
            self.sequences.push("4".to_string());
        }
        if let Some(true) = style.blink {
            self.sequences.push("5".to_string());
        }
        if let Some(true) = style.reverse {
            self.sequences.push("7".to_string());
        }
        if let Some(true) = style.hidden {
            self.sequences.push("8".to_string());
        }
        if let Some(true) = style.strikethrough {
            self.sequences.push("9".to_string());
        }

        // Turn off attributes explicitly set to false
        if let Some(false) = style.bold {
            self.sequences.push("22".to_string()); // Normal intensity
        }
        if let Some(false) = style.dim {
            self.sequences.push("22".to_string()); // Normal intensity
        }
        if let Some(false) = style.italic {
            self.sequences.push("23".to_string());
        }
        if let Some(false) = style.underline {
            self.sequences.push("24".to_string());
        }
        if let Some(false) = style.blink {
            self.sequences.push("25".to_string());
        }
        if let Some(false) = style.reverse {
            self.sequences.push("27".to_string());
        }
        if let Some(false) = style.hidden {
            self.sequences.push("28".to_string());
        }
        if let Some(false) = style.strikethrough {
            self.sequences.push("29".to_string());
        }
    }

    /// Add a color escape sequence.
    fn add_color(&mut self, color: Color, is_background: bool, color_system: ColorSystem) {
        let downgraded = color.downgrade(color_system);

        match downgraded {
            Color::Default => {
                if is_background {
                    self.sequences.push("49".to_string()); // Default background
                } else {
                    self.sequences.push("39".to_string()); // Default foreground
                }
            }
            Color::Standard(std_color) => {
                let code = if is_background {
                    std_color.ansi_bg_code()
                } else {
                    std_color.ansi_fg_code()
                };
                self.sequences.push(code.to_string());
            }
            Color::EightBit(index) => {
                if is_background {
                    self.sequences.push(format!("48;5;{}", index));
                } else {
                    self.sequences.push(format!("38;5;{}", index));
                }
            }
            Color::TrueColor { r, g, b } => {
                if is_background {
                    self.sequences.push(format!("48;2;{};{};{}", r, g, b));
                } else {
                    self.sequences.push(format!("38;2;{};{};{}", r, g, b));
                }
            }
        }
    }

    /// Build the final ANSI escape sequence.
    ///
    /// Returns an empty string if no sequences were added.
    pub fn build(self) -> String {
        if self.sequences.is_empty() {
            String::new()
        } else {
            format!("\x1b[{}m", self.sequences.join(";"))
        }
    }

    /// Check if the builder is empty.
    pub fn is_empty(&self) -> bool {
        self.sequences.is_empty()
    }
}

impl Default for AnsiBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Generate ANSI escape sequence for a style.
///
/// This is a convenience function that creates an AnsiBuilder, adds the style,
/// and builds the escape sequence.
///
/// # Examples
///
/// ```rust
/// use luxor::{ansi::style_to_ansi, Style, Color, ColorSystem};
///
/// let style = Style::new().bold().color(Color::rgb(255, 0, 0));
/// let ansi = style_to_ansi(&style, ColorSystem::TrueColor);
/// println!("{}Bold red text\x1b[0m", ansi);
/// ```
pub fn style_to_ansi(style: &Style, color_system: ColorSystem) -> String {
    let mut builder = AnsiBuilder::new();
    builder.add_style(style, color_system);
    builder.build()
}

/// Reset all ANSI formatting.
pub const RESET: &str = "\x1b[0m";

/// Common ANSI escape sequences.
pub mod codes {
    /// Reset all formatting.
    pub const RESET: &str = "\x1b[0m";

    /// Clear screen.
    pub const CLEAR_SCREEN: &str = "\x1b[2J";

    /// Move cursor to home position (0,0).
    pub const CURSOR_HOME: &str = "\x1b[H";

    /// Hide cursor.
    pub const CURSOR_HIDE: &str = "\x1b[?25l";

    /// Show cursor.
    pub const CURSOR_SHOW: &str = "\x1b[?25h";

    /// Enable alternative screen buffer.
    pub const ALT_SCREEN_ENABLE: &str = "\x1b[?1049h";

    /// Disable alternative screen buffer.
    pub const ALT_SCREEN_DISABLE: &str = "\x1b[?1049l";
}

/// Generate cursor movement escape sequences.
pub mod cursor {
    /// Move cursor up by `n` lines.
    pub fn up(n: usize) -> String {
        format!("\x1b[{}A", n)
    }

    /// Move cursor down by `n` lines.
    pub fn down(n: usize) -> String {
        format!("\x1b[{}B", n)
    }

    /// Move cursor right by `n` columns.
    pub fn right(n: usize) -> String {
        format!("\x1b[{}C", n)
    }

    /// Move cursor left by `n` columns.
    pub fn left(n: usize) -> String {
        format!("\x1b[{}D", n)
    }

    /// Move cursor to specific position (row, column). 1-indexed.
    pub fn position(row: usize, col: usize) -> String {
        format!("\x1b[{};{}H", row, col)
    }

    /// Move cursor to column. 1-indexed.
    pub fn column(col: usize) -> String {
        format!("\x1b[{}G", col)
    }
}

/// Strip ANSI escape sequences from a string.
///
/// This function removes all ANSI escape sequences from the input string,
/// leaving only the plain text content.
///
/// # Examples
///
/// ```rust
/// use luxor::ansi::strip_ansi;
///
/// let styled = "\x1b[1;31mHello\x1b[0m World";
/// let plain = strip_ansi(styled);
/// assert_eq!(plain, "Hello World");
/// ```
pub fn strip_ansi(text: &str) -> String {
    let mut result = String::new();
    let mut chars = text.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '\x1b' {
            // Start of escape sequence
            if chars.peek() == Some(&'[') {
                chars.next(); // Consume '['

                // Skip until we find the end of the sequence
                for ch in chars.by_ref() {
                    if ch.is_ascii_alphabetic() {
                        break; // End of sequence - found terminator
                    }
                    // If we encounter another escape or end of string without terminator,
                    // this was an incomplete sequence
                    if ch == '\x1b' {
                        // Put back the escape character for next iteration
                        result.push(ch);
                        break;
                    }
                }

                // If we didn't find a proper terminator, this was likely an incomplete
                // or malformed sequence. In that case, we've already consumed it.
                // For incomplete sequences like "\x1b[", we just remove them entirely.
            } else {
                // Not an ANSI sequence, keep the character
                result.push(ch);
            }
        } else {
            result.push(ch);
        }
    }

    result
}

/// Calculate the display width of text, ignoring ANSI escape sequences.
///
/// This function strips ANSI sequences and then calculates the Unicode display width.
///
/// # Examples
///
/// ```rust
/// use luxor::ansi::text_width;
///
/// let styled = "\x1b[1;31mHello\x1b[0m";
/// assert_eq!(text_width(styled), 5);
/// ```
pub fn text_width(text: &str) -> usize {
    use unicode_width::UnicodeWidthStr;
    strip_ansi(text).width()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Color, ColorSystem, StandardColor, Style};

    #[test]
    fn test_ansi_builder_empty() {
        let builder = AnsiBuilder::new();
        assert!(builder.is_empty());
        assert_eq!(builder.build(), "");
    }

    #[test]
    fn test_ansi_builder_bold() {
        let mut builder = AnsiBuilder::new();
        let style = Style::new().bold();
        builder.add_style(&style, ColorSystem::Standard);
        assert_eq!(builder.build(), "\x1b[1m");
    }

    #[test]
    fn test_ansi_builder_color() {
        let mut builder = AnsiBuilder::new();
        let style = Style::new().color(Color::Standard(StandardColor::Red));
        builder.add_style(&style, ColorSystem::Standard);
        assert_eq!(builder.build(), "\x1b[31m");
    }

    #[test]
    fn test_ansi_builder_true_color() {
        let mut builder = AnsiBuilder::new();
        let style = Style::new().color(Color::rgb(255, 128, 64));
        builder.add_style(&style, ColorSystem::TrueColor);
        assert_eq!(builder.build(), "\x1b[38;2;255;128;64m");
    }

    #[test]
    fn test_ansi_builder_complex() {
        let mut builder = AnsiBuilder::new();
        let style = Style::new()
            .bold()
            .italic()
            .color(Color::Standard(StandardColor::Red))
            .background(Color::Standard(StandardColor::Blue));
        builder.add_style(&style, ColorSystem::Standard);

        let result = builder.build();
        assert!(result.contains("31")); // Red foreground
        assert!(result.contains("44")); // Blue background
        assert!(result.contains("1")); // Bold
        assert!(result.contains("3")); // Italic
    }

    #[test]
    fn test_style_to_ansi() {
        let style = Style::new().bold().color(Color::rgb(255, 0, 0));
        let ansi = style_to_ansi(&style, ColorSystem::TrueColor);
        assert!(ansi.contains("38;2;255;0;0")); // True color red
        assert!(ansi.contains("1")); // Bold
    }

    #[test]
    fn test_cursor_functions() {
        assert_eq!(cursor::up(5), "\x1b[5A");
        assert_eq!(cursor::down(3), "\x1b[3B");
        assert_eq!(cursor::position(10, 20), "\x1b[10;20H");
    }

    #[test]
    fn test_strip_ansi() {
        let styled = "\x1b[1;31mHello\x1b[0m World";
        let plain = strip_ansi(styled);
        assert_eq!(plain, "Hello World");
    }

    #[test]
    fn test_strip_ansi_complex() {
        let styled = "\x1b[38;2;255;0;0mRed\x1b[39m \x1b[1mBold\x1b[22m";
        let plain = strip_ansi(styled);
        assert_eq!(plain, "Red Bold");
    }

    #[test]
    fn test_text_width() {
        let styled = "\x1b[1;31mHello\x1b[0m";
        assert_eq!(text_width(styled), 5);

        let plain = "Hello";
        assert_eq!(text_width(plain), 5);
    }

    #[test]
    fn test_strip_ansi_no_escape() {
        let plain = "Hello World";
        assert_eq!(strip_ansi(plain), plain);
    }
}
