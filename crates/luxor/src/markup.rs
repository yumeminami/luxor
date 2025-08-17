//! Rich markup parser for BBCode-style text formatting.
//!
//! This module implements Rich's markup syntax which allows inline styling using BBCode-like tags:
//! - `[bold]text[/bold]` - Apply bold styling
//! - `[red]text[/red]` - Apply red color
//! - `[bold red]text[/bold red]` - Combine styles
//! - `[/]` - Close the most recent tag
//! - `\[` - Escape square brackets
//!
//! The parser converts markup strings into `Text` instances with appropriate style spans.

use crate::{error::LuxorError, style::Style, text::Text};
use std::collections::VecDeque;

/// A tuple of (start_position, Tag, Style) for tracking open style tags.
type StyleStackEntry = (usize, Tag, Style);

/// Result type for token parsing operations.
type ParseResult = Result<Vec<Token>, LuxorError>;

/// Result type for span split operations.
type SplitResult = (Span, Option<Span>);

/// A markup tag with optional parameters.
#[derive(Debug, Clone, PartialEq)]
pub struct Tag {
    /// The tag name (e.g., "bold", "red", "on blue")
    pub name: String,
    /// Optional parameters after the tag name (e.g., "=value" part)
    pub parameters: Option<String>,
}

impl Tag {
    /// Create a new tag with the given name.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            parameters: None,
        }
    }

    /// Create a new tag with name and parameters.
    pub fn with_parameters(name: impl Into<String>, parameters: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            parameters: Some(parameters.into()),
        }
    }

    /// Get the markup representation of this tag.
    pub fn markup(&self) -> String {
        match &self.parameters {
            Some(params) => format!("[{}={}]", self.name, params),
            None => format!("[{}]", self.name),
        }
    }

    /// Check if this is a closing tag (starts with "/").
    pub fn is_closing(&self) -> bool {
        self.name.starts_with('/')
    }

    /// Get the style name for closing tags (removes the leading "/").
    pub fn closing_name(&self) -> &str {
        if self.is_closing() {
            &self.name[1..]
        } else {
            &self.name
        }
    }
}

impl std::fmt::Display for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.parameters {
            Some(params) => write!(f, "{} {}", self.name, params),
            None => write!(f, "{}", self.name),
        }
    }
}

/// A span of styled text within markup.
#[derive(Debug, Clone, PartialEq)]
pub struct Span {
    /// Start position in the text
    pub start: usize,
    /// End position in the text
    pub end: usize,
    /// Style to apply to this span
    pub style: Style,
}

impl Span {
    /// Create a new span.
    pub fn new(start: usize, end: usize, style: Style) -> Self {
        Self { start, end, style }
    }

    /// Check if this span is empty.
    pub fn is_empty(&self) -> bool {
        self.end <= self.start
    }

    /// Get the length of this span.
    pub fn len(&self) -> usize {
        self.end.saturating_sub(self.start)
    }

    /// Split this span at the given offset.
    /// Returns (left_span, right_span) where right_span is None if offset is outside the span.
    pub fn split(&self, offset: usize) -> SplitResult {
        if offset <= self.start {
            return (self.clone(), None);
        }
        if offset >= self.end {
            return (self.clone(), None);
        }

        let left = Span::new(self.start, offset, self.style.clone());
        let right = Span::new(offset, self.end, self.style.clone());
        (left, Some(right))
    }

    /// Move this span by the given offset.
    pub fn move_by(&self, offset: isize) -> Self {
        let start = if offset < 0 {
            self.start.saturating_sub((-offset) as usize)
        } else {
            self.start + (offset as usize)
        };
        let end = if offset < 0 {
            self.end.saturating_sub((-offset) as usize)
        } else {
            self.end + (offset as usize)
        };
        Span::new(start, end, self.style.clone())
    }
}

/// Token produced by the markup parser.
#[derive(Debug, Clone, PartialEq)]
enum Token {
    /// Plain text content
    Text(String),
    /// An opening or closing tag
    Tag(Tag),
}

/// Parse markup tokens from a string.
fn parse_tokens(markup: &str) -> ParseResult {
    let mut tokens = Vec::new();
    let mut chars = markup.char_indices().peekable();
    let mut current_pos = 0;

    while let Some((pos, ch)) = chars.next() {
        if ch == '[' {
            // Check for escaped bracket
            if let Some((_, next_ch)) = chars.peek() {
                if *next_ch == '[' {
                    // Escaped bracket, add text up to this point and the bracket
                    if pos > current_pos {
                        tokens.push(Token::Text(markup[current_pos..pos].to_string()));
                    }
                    tokens.push(Token::Text("[".to_string()));
                    chars.next(); // consume the second '['
                    current_pos = pos + 2;
                    continue;
                }
            }

            // Find the closing bracket
            let tag_start = pos + 1;
            let mut tag_end = None;

            for (bracket_idx, bracket_ch) in chars.by_ref() {
                if bracket_ch == ']' {
                    tag_end = Some(bracket_idx);
                    break;
                }
            }

            if let Some(end_pos) = tag_end {
                // Add any text before this tag
                if pos > current_pos {
                    tokens.push(Token::Text(markup[current_pos..pos].to_string()));
                }

                // Parse the tag content
                let tag_content = &markup[tag_start..end_pos];
                if !tag_content.is_empty() {
                    let tag = parse_tag(tag_content)?;
                    tokens.push(Token::Tag(tag));
                }

                current_pos = end_pos + 1;
            } else {
                // No closing bracket found, treat as regular text
                // Continue to next character
                current_pos = pos;
            }
        }
    }

    // Add any remaining text
    if current_pos < markup.len() {
        tokens.push(Token::Text(markup[current_pos..].to_string()));
    }

    Ok(tokens)
}

/// Parse a tag from its content (without brackets).
fn parse_tag(content: &str) -> Result<Tag, LuxorError> {
    if let Some(equals_pos) = content.find('=') {
        let name = content[..equals_pos].trim().to_string();
        let parameters = content[equals_pos + 1..].trim().to_string();
        Ok(Tag::with_parameters(name, parameters))
    } else {
        Ok(Tag::new(content.trim()))
    }
}

/// Escape markup syntax in plain text.
pub fn escape(text: &str) -> String {
    text.replace('[', "\\[").replace(']', "\\]")
}

/// Parse markup and render it into a `Text` instance.
pub fn render(markup: &str, base_style: Option<Style>) -> Result<Text, LuxorError> {
    // If no markup tags are present, return simple text
    if !markup.contains('[') {
        return Ok(Text::new(markup).with_style(base_style.unwrap_or_default()));
    }

    let tokens = parse_tokens(markup)?;
    let mut text_content = String::new();
    let mut spans = Vec::new();
    let mut style_stack: VecDeque<StyleStackEntry> = VecDeque::new();

    // Helper to create style from tag
    let create_style_from_tag = |tag: &Tag| -> Result<Style, LuxorError> {
        // For now, parse basic styles. This will be expanded.
        Style::parse(&tag.name)
    };

    for token in tokens {
        match token {
            Token::Text(text) => {
                // Add the text content
                text_content.push_str(&text);
            }
            Token::Tag(tag) => {
                if tag.is_closing() {
                    let style_name = tag.closing_name();

                    if style_name.is_empty() {
                        // Implicit close - close the most recent tag
                        if let Some((start_pos, _open_tag, style)) = style_stack.pop_back() {
                            spans.push(Span::new(start_pos, text_content.len(), style));
                        }
                    } else {
                        // Explicit close - find matching tag
                        let mut found_index = None;
                        for (index, (_, open_tag, _)) in style_stack.iter().enumerate().rev() {
                            if open_tag.name == style_name {
                                found_index = Some(index);
                                break;
                            }
                        }

                        if let Some(index) = found_index {
                            // Close all tags from this point to the end
                            let removed_tags: Vec<_> = style_stack.drain(index..).collect();
                            for (start_pos, _tag, style) in removed_tags {
                                spans.push(Span::new(start_pos, text_content.len(), style));
                            }
                        } else {
                            return Err(LuxorError::MarkupError(format!(
                                "Closing tag '{}' has no matching opening tag",
                                tag.name
                            )));
                        }
                    }
                } else {
                    // Opening tag
                    let style = create_style_from_tag(&tag)?;
                    style_stack.push_back((text_content.len(), tag, style));
                }
            }
        }
    }

    // Close any remaining open tags
    while let Some((start_pos, _tag, style)) = style_stack.pop_back() {
        spans.push(Span::new(start_pos, text_content.len(), style));
    }

    // Create the text with spans
    let mut text = Text::new(&text_content);
    if let Some(base) = base_style {
        text = text.with_style(base);
    }

    // Apply all spans
    for span in spans {
        if !span.is_empty() {
            text.stylize_range(span.start..span.end, span.style)?;
        }
    }

    Ok(text)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::Color;

    #[test]
    fn test_tag_creation() {
        let tag = Tag::new("bold");
        assert_eq!(tag.name, "bold");
        assert_eq!(tag.parameters, None);
        assert_eq!(tag.markup(), "[bold]");
        assert!(!tag.is_closing());
    }

    #[test]
    fn test_tag_with_parameters() {
        let tag = Tag::with_parameters("color", "red");
        assert_eq!(tag.name, "color");
        assert_eq!(tag.parameters, Some("red".to_string()));
        assert_eq!(tag.markup(), "[color=red]");
    }

    #[test]
    fn test_closing_tag() {
        let tag = Tag::new("/bold");
        assert!(tag.is_closing());
        assert_eq!(tag.closing_name(), "bold");
    }

    #[test]
    fn test_parse_simple_tag() {
        let tag = parse_tag("bold").unwrap();
        assert_eq!(tag.name, "bold");
        assert_eq!(tag.parameters, None);
    }

    #[test]
    fn test_parse_tag_with_parameters() {
        let tag = parse_tag("color=red").unwrap();
        assert_eq!(tag.name, "color");
        assert_eq!(tag.parameters, Some("red".to_string()));
    }

    #[test]
    fn test_parse_tokens_plain_text() {
        let tokens = parse_tokens("Hello world").unwrap();
        assert_eq!(tokens.len(), 1);
        match &tokens[0] {
            Token::Text(text) => assert_eq!(text, "Hello world"),
            _ => panic!("Expected text token"),
        }
    }

    #[test]
    fn test_parse_tokens_with_tag() {
        let tokens = parse_tokens("Hello [bold]world[/bold]").unwrap();
        assert_eq!(tokens.len(), 4);

        match &tokens[0] {
            Token::Text(text) => assert_eq!(text, "Hello "),
            _ => panic!("Expected text token"),
        }

        match &tokens[1] {
            Token::Tag(tag) => {
                assert_eq!(tag.name, "bold");
                assert!(!tag.is_closing());
            }
            _ => panic!("Expected tag token"),
        }

        match &tokens[2] {
            Token::Text(text) => assert_eq!(text, "world"),
            _ => panic!("Expected text token"),
        }

        match &tokens[3] {
            Token::Tag(tag) => {
                assert_eq!(tag.name, "/bold");
                assert!(tag.is_closing());
            }
            _ => panic!("Expected tag token"),
        }
    }

    #[test]
    fn test_render_plain_text() {
        let text = render("Hello world", None).unwrap();
        assert_eq!(text.plain(), "Hello world");
        assert_eq!(text.spans().len(), 0);
    }

    #[test]
    fn test_render_bold_text() {
        let text = render("[bold]Hello world[/bold]", None).unwrap();
        assert_eq!(text.plain(), "Hello world");
        assert_eq!(text.spans().len(), 1);

        let span = &text.spans()[0];
        assert_eq!(span.start, 0);
        assert_eq!(span.end, 11);
        assert_eq!(span.style.bold, Some(true));
    }

    #[test]
    fn test_render_colored_text() {
        let text = render("[red]Hello world[/red]", None).unwrap();
        assert_eq!(text.plain(), "Hello world");
        assert_eq!(text.spans().len(), 1);

        let span = &text.spans()[0];
        assert_eq!(span.start, 0);
        assert_eq!(span.end, 11);
        assert_eq!(
            span.style.color,
            Some(Color::Standard(crate::StandardColor::Red))
        );
    }

    #[test]
    fn test_render_nested_tags() {
        let text = render("[bold]Hello [red]world[/red][/bold]", None).unwrap();
        assert_eq!(text.plain(), "Hello world");
        assert_eq!(text.spans().len(), 2);

        // The bold span should cover the entire text
        let bold_span = text
            .spans()
            .iter()
            .find(|s| s.style.bold == Some(true))
            .unwrap();
        assert_eq!(bold_span.start, 0);
        assert_eq!(bold_span.end, 11);

        // The red span should only cover "world"
        let red_span = text
            .spans()
            .iter()
            .find(|s| s.style.color == Some(Color::Standard(crate::StandardColor::Red)))
            .unwrap();
        assert_eq!(red_span.start, 6);
        assert_eq!(red_span.end, 11);
    }

    #[test]
    fn test_render_implicit_close() {
        let text = render("[bold]Hello world[/]", None).unwrap();
        assert_eq!(text.plain(), "Hello world");
        assert_eq!(text.spans().len(), 1);

        let span = &text.spans()[0];
        assert_eq!(span.start, 0);
        assert_eq!(span.end, 11);
        assert_eq!(span.style.bold, Some(true));
    }

    #[test]
    fn test_escape_markup() {
        assert_eq!(escape("Hello [world]"), "Hello \\[world\\]");
        assert_eq!(escape("No markup here"), "No markup here");
    }

    #[test]
    fn test_span_operations() {
        let style = Style::new().bold();
        let span = Span::new(0, 10, style.clone());

        assert!(!span.is_empty());
        assert_eq!(span.len(), 10);

        let (left, right) = span.split(5);
        assert_eq!(left.start, 0);
        assert_eq!(left.end, 5);
        assert!(right.is_some());

        let right = right.unwrap();
        assert_eq!(right.start, 5);
        assert_eq!(right.end, 10);

        let moved = span.move_by(3);
        assert_eq!(moved.start, 3);
        assert_eq!(moved.end, 13);
    }
}
