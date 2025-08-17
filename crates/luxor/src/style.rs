//! Style system for text formatting and appearance.

use crate::{Color, LuxorError, Result};
use std::fmt;

/// Text style attributes.
///
/// A style defines the appearance of text including colors, font attributes,
/// and other formatting options. Styles can be composed and inherited.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Style {
    /// Foreground color.
    pub color: Option<Color>,
    /// Background color.
    pub background: Option<Color>,
    /// Bold text.
    pub bold: Option<bool>,
    /// Italic text.
    pub italic: Option<bool>,
    /// Underlined text.
    pub underline: Option<bool>,
    /// Strikethrough text.
    pub strikethrough: Option<bool>,
    /// Dim/faint text.
    pub dim: Option<bool>,
    /// Reverse/inverse colors.
    pub reverse: Option<bool>,
    /// Blinking text.
    pub blink: Option<bool>,
    /// Hidden/invisible text.
    pub hidden: Option<bool>,
}

impl Style {
    /// Create a new empty style.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use luxor::Style;
    ///
    /// let style = Style::new();
    /// assert!(style.is_empty());
    /// ```
    pub fn new() -> Self {
        Self {
            color: None,
            background: None,
            bold: None,
            italic: None,
            underline: None,
            strikethrough: None,
            dim: None,
            reverse: None,
            blink: None,
            hidden: None,
        }
    }

    /// Set the foreground color.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use luxor::{Style, Color};
    ///
    /// let style = Style::new().color(Color::rgb(255, 0, 0));
    /// ```
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    /// Set the background color.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use luxor::{Style, Color};
    ///
    /// let style = Style::new().background(Color::rgb(0, 255, 0));
    /// ```
    pub fn background(mut self, color: Color) -> Self {
        self.background = Some(color);
        self
    }

    /// Set bold formatting.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use luxor::Style;
    ///
    /// let style = Style::new().bold();
    /// let style_not_bold = Style::new().bold_off();
    /// ```
    pub fn bold(mut self) -> Self {
        self.bold = Some(true);
        self
    }

    /// Explicitly turn off bold formatting.
    pub fn bold_off(mut self) -> Self {
        self.bold = Some(false);
        self
    }

    /// Set italic formatting.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use luxor::Style;
    ///
    /// let style = Style::new().italic();
    /// ```
    pub fn italic(mut self) -> Self {
        self.italic = Some(true);
        self
    }

    /// Explicitly turn off italic formatting.
    pub fn italic_off(mut self) -> Self {
        self.italic = Some(false);
        self
    }

    /// Set underline formatting.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use luxor::Style;
    ///
    /// let style = Style::new().underline();
    /// ```
    pub fn underline(mut self) -> Self {
        self.underline = Some(true);
        self
    }

    /// Explicitly turn off underline formatting.
    pub fn underline_off(mut self) -> Self {
        self.underline = Some(false);
        self
    }

    /// Set strikethrough formatting.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use luxor::Style;
    ///
    /// let style = Style::new().strikethrough();
    /// ```
    pub fn strikethrough(mut self) -> Self {
        self.strikethrough = Some(true);
        self
    }

    /// Explicitly turn off strikethrough formatting.
    pub fn strikethrough_off(mut self) -> Self {
        self.strikethrough = Some(false);
        self
    }

    /// Set dim/faint formatting.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use luxor::Style;
    ///
    /// let style = Style::new().dim();
    /// ```
    pub fn dim(mut self) -> Self {
        self.dim = Some(true);
        self
    }

    /// Explicitly turn off dim formatting.
    pub fn dim_off(mut self) -> Self {
        self.dim = Some(false);
        self
    }

    /// Set reverse/inverse formatting.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use luxor::Style;
    ///
    /// let style = Style::new().reverse();
    /// ```
    pub fn reverse(mut self) -> Self {
        self.reverse = Some(true);
        self
    }

    /// Explicitly turn off reverse formatting.
    pub fn reverse_off(mut self) -> Self {
        self.reverse = Some(false);
        self
    }

    /// Set blinking formatting.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use luxor::Style;
    ///
    /// let style = Style::new().blink();
    /// ```
    pub fn blink(mut self) -> Self {
        self.blink = Some(true);
        self
    }

    /// Explicitly turn off blinking.
    pub fn blink_off(mut self) -> Self {
        self.blink = Some(false);
        self
    }

    /// Set hidden/invisible formatting.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use luxor::Style;
    ///
    /// let style = Style::new().hidden();
    /// ```
    pub fn hidden(mut self) -> Self {
        self.hidden = Some(true);
        self
    }

    /// Explicitly turn off hidden formatting.
    pub fn hidden_off(mut self) -> Self {
        self.hidden = Some(false);
        self
    }

    /// Check if this style has no attributes set.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use luxor::{Style, Color};
    ///
    /// assert!(Style::new().is_empty());
    /// assert!(!Style::new().color(Color::rgb(255, 0, 0)).is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.color.is_none()
            && self.background.is_none()
            && self.bold.is_none()
            && self.italic.is_none()
            && self.underline.is_none()
            && self.strikethrough.is_none()
            && self.dim.is_none()
            && self.reverse.is_none()
            && self.blink.is_none()
            && self.hidden.is_none()
    }

    /// Combine this style with another style.
    ///
    /// Attributes from the other style will override attributes in this style
    /// if they are set (Some). This allows for style inheritance and composition.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use luxor::{Style, Color};
    ///
    /// let base = Style::new().color(Color::rgb(255, 0, 0)).bold();
    /// let overlay = Style::new().color(Color::rgb(0, 255, 0)).italic();
    /// let combined = base.combine(overlay);
    ///
    /// // Result has green color (from overlay), bold (from base), and italic (from overlay)
    /// ```
    pub fn combine(self, other: Self) -> Self {
        Self {
            color: other.color.or(self.color),
            background: other.background.or(self.background),
            bold: other.bold.or(self.bold),
            italic: other.italic.or(self.italic),
            underline: other.underline.or(self.underline),
            strikethrough: other.strikethrough.or(self.strikethrough),
            dim: other.dim.or(self.dim),
            reverse: other.reverse.or(self.reverse),
            blink: other.blink.or(self.blink),
            hidden: other.hidden.or(self.hidden),
        }
    }

    /// Parse a style from a string representation.
    ///
    /// Supports various formats:
    /// - Color names: "red", "green", "blue"
    /// - Hex colors: "#FF0000", "#F00"
    /// - Style attributes: "bold", "italic", "underline"
    /// - Combined: "bold red on blue", "italic #FF0000"
    ///
    /// # Examples
    ///
    /// ```rust
    /// use luxor::Style;
    ///
    /// let style1 = Style::parse("bold red").unwrap();
    /// let style2 = Style::parse("italic #FF0000 on #0000FF").unwrap();
    /// ```
    pub fn parse(style_str: &str) -> Result<Self> {
        let mut style = Style::new();
        let mut tokens = style_str.split_whitespace().peekable();

        while let Some(token) = tokens.next() {
            match token.to_lowercase().as_str() {
                "bold" => style.bold = Some(true),
                "italic" => style.italic = Some(true),
                "underline" => style.underline = Some(true),
                "strikethrough" => style.strikethrough = Some(true),
                "dim" => style.dim = Some(true),
                "reverse" => style.reverse = Some(true),
                "blink" => style.blink = Some(true),
                "hidden" => style.hidden = Some(true),
                "on" => {
                    // Next token should be background color
                    if let Some(bg_token) = tokens.next() {
                        style.background = Some(parse_color_token(bg_token)?);
                    } else {
                        return Err(LuxorError::style("Expected color after 'on'"));
                    }
                }
                color_token => {
                    // Try to parse as color
                    if let Ok(color) = parse_color_token(color_token) {
                        if style.color.is_none() {
                            style.color = Some(color);
                        }
                    } else {
                        return Err(LuxorError::style(format!(
                            "Unknown style token: {}",
                            color_token
                        )));
                    }
                }
            }
        }

        Ok(style)
    }
}

impl Default for Style {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Style {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut parts = Vec::new();

        if let Some(true) = self.bold {
            parts.push("bold".to_string());
        }
        if let Some(true) = self.italic {
            parts.push("italic".to_string());
        }
        if let Some(true) = self.underline {
            parts.push("underline".to_string());
        }
        if let Some(true) = self.strikethrough {
            parts.push("strikethrough".to_string());
        }
        if let Some(true) = self.dim {
            parts.push("dim".to_string());
        }
        if let Some(true) = self.reverse {
            parts.push("reverse".to_string());
        }
        if let Some(true) = self.blink {
            parts.push("blink".to_string());
        }
        if let Some(true) = self.hidden {
            parts.push("hidden".to_string());
        }

        if let Some(color) = self.color {
            parts.push(format!("color:{:?}", color));
        }

        if let Some(background) = self.background {
            parts.push(format!("bg:{:?}", background));
        }

        if parts.is_empty() {
            write!(f, "Style::new()")
        } else {
            write!(f, "Style({})", parts.join(" "))
        }
    }
}

/// Parse a color token from a string.
fn parse_color_token(token: &str) -> Result<Color> {
    match token.to_lowercase().as_str() {
        "black" => Ok(Color::Standard(crate::StandardColor::Black)),
        "red" => Ok(Color::Standard(crate::StandardColor::Red)),
        "green" => Ok(Color::Standard(crate::StandardColor::Green)),
        "yellow" => Ok(Color::Standard(crate::StandardColor::Yellow)),
        "blue" => Ok(Color::Standard(crate::StandardColor::Blue)),
        "magenta" => Ok(Color::Standard(crate::StandardColor::Magenta)),
        "cyan" => Ok(Color::Standard(crate::StandardColor::Cyan)),
        "white" => Ok(Color::Standard(crate::StandardColor::White)),
        "bright_black" | "gray" | "grey" => Ok(Color::Standard(crate::StandardColor::BrightBlack)),
        "bright_red" => Ok(Color::Standard(crate::StandardColor::BrightRed)),
        "bright_green" => Ok(Color::Standard(crate::StandardColor::BrightGreen)),
        "bright_yellow" => Ok(Color::Standard(crate::StandardColor::BrightYellow)),
        "bright_blue" => Ok(Color::Standard(crate::StandardColor::BrightBlue)),
        "bright_magenta" => Ok(Color::Standard(crate::StandardColor::BrightMagenta)),
        "bright_cyan" => Ok(Color::Standard(crate::StandardColor::BrightCyan)),
        "bright_white" => Ok(Color::Standard(crate::StandardColor::BrightWhite)),
        _ => {
            // Try to parse as hex color
            if token.starts_with('#') || token.len() == 3 || token.len() == 6 {
                Color::from_hex(token)
            } else {
                Err(LuxorError::color(format!("Unknown color: {}", token)))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Color, StandardColor};

    #[test]
    fn test_style_new() {
        let style = Style::new();
        assert!(style.is_empty());
    }

    #[test]
    fn test_style_builder() {
        let style = Style::new()
            .color(Color::rgb(255, 0, 0))
            .background(Color::rgb(0, 255, 0))
            .bold()
            .italic();

        assert_eq!(style.color, Some(Color::rgb(255, 0, 0)));
        assert_eq!(style.background, Some(Color::rgb(0, 255, 0)));
        assert_eq!(style.bold, Some(true));
        assert_eq!(style.italic, Some(true));
        assert!(!style.is_empty());
    }

    #[test]
    fn test_style_combine() {
        let base = Style::new().color(Color::rgb(255, 0, 0)).bold();

        let overlay = Style::new().color(Color::rgb(0, 255, 0)).italic();

        let combined = base.combine(overlay);

        assert_eq!(combined.color, Some(Color::rgb(0, 255, 0))); // From overlay
        assert_eq!(combined.bold, Some(true)); // From base
        assert_eq!(combined.italic, Some(true)); // From overlay
    }

    #[test]
    fn test_style_parse_simple() {
        let style = Style::parse("bold").unwrap();
        assert_eq!(style.bold, Some(true));

        let style = Style::parse("red").unwrap();
        assert_eq!(style.color, Some(Color::Standard(StandardColor::Red)));
    }

    #[test]
    fn test_style_parse_complex() {
        let style = Style::parse("bold italic red on blue").unwrap();
        assert_eq!(style.bold, Some(true));
        assert_eq!(style.italic, Some(true));
        assert_eq!(style.color, Some(Color::Standard(StandardColor::Red)));
        assert_eq!(style.background, Some(Color::Standard(StandardColor::Blue)));
    }

    #[test]
    fn test_style_parse_hex() {
        let style = Style::parse("bold #FF0000 on #0000FF").unwrap();
        assert_eq!(style.bold, Some(true));
        assert_eq!(style.color, Some(Color::rgb(255, 0, 0)));
        assert_eq!(style.background, Some(Color::rgb(0, 0, 255)));
    }

    #[test]
    fn test_style_parse_invalid() {
        assert!(Style::parse("invalid_color").is_err());
        assert!(Style::parse("bold on").is_err()); // Missing color after 'on'
    }

    #[test]
    fn test_style_display() {
        let style = Style::new().bold().color(Color::rgb(255, 0, 0));
        let string_repr = format!("{}", style);
        assert!(string_repr.contains("bold"));
        assert!(string_repr.contains("color"));
    }

    #[test]
    fn test_parse_color_token() {
        assert_eq!(
            parse_color_token("red").unwrap(),
            Color::Standard(StandardColor::Red)
        );
        assert_eq!(
            parse_color_token("bright_blue").unwrap(),
            Color::Standard(StandardColor::BrightBlue)
        );
        assert_eq!(parse_color_token("#FF0000").unwrap(), Color::rgb(255, 0, 0));
        assert!(parse_color_token("invalid").is_err());
    }
}
