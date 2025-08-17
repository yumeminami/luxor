//! Text Highlighter Example
//!
//! This example demonstrates how to create text highlighters that can
//! automatically style specific patterns in text, similar to Rich's highlighter system.

use luxor::{Color, Console, LuxorError, Style, Text};

fn main() -> Result<(), LuxorError> {
    let console = Console::new();

    println!("=== Text Highlighter Example ===\n");

    // Create different types of highlighters
    demonstrate_email_highlighter(&console)?;
    demonstrate_url_highlighter(&console)?;
    demonstrate_keyword_highlighter(&console)?;
    demonstrate_number_highlighter(&console)?;

    Ok(())
}

fn demonstrate_email_highlighter(console: &Console) -> Result<(), LuxorError> {
    println!("1. Email highlighter:");

    let text = "Contact us at support@luxor.com or admin@example.org for help.";
    let highlighted = highlight_emails(text)?;
    console.print(highlighted)?;
    println!();
    Ok(())
}

fn demonstrate_url_highlighter(console: &Console) -> Result<(), LuxorError> {
    println!("2. URL highlighter:");

    let text = "Visit https://github.com/luxor or check out http://example.com for more info.";
    let highlighted = highlight_urls(text)?;
    console.print(highlighted)?;
    println!();
    Ok(())
}

fn demonstrate_keyword_highlighter(console: &Console) -> Result<(), LuxorError> {
    println!("3. Keyword highlighter (Rust keywords):");

    let text = "fn main() { let mut x = 5; if x > 0 { return Ok(()); } }";
    let highlighted = highlight_rust_keywords(text)?;
    console.print(highlighted)?;
    println!();
    Ok(())
}

fn demonstrate_number_highlighter(console: &Console) -> Result<(), LuxorError> {
    println!("4. Number highlighter:");

    let text = "The answer is 42, or maybe 3.14159, but definitely not -123.456e10.";
    let highlighted = highlight_numbers(text)?;
    console.print(highlighted)?;
    println!();
    Ok(())
}

/// Simple email highlighter using basic pattern matching
fn highlight_emails(text: &str) -> Result<Text, LuxorError> {
    let mut result = Text::new(text);
    let email_style = Style::new().bold().color(Color::rgb(255, 0, 255)); // Magenta

    // Simple email pattern: word@word.word
    let chars: Vec<char> = text.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        if let Some(email_range) = find_email_at_position(&chars, i) {
            let byte_start = char_pos_to_byte_pos(text, email_range.0);
            let byte_end = char_pos_to_byte_pos(text, email_range.1);
            result.stylize_range(byte_start..byte_end, email_style.clone())?;
            i = email_range.1;
        } else {
            i += 1;
        }
    }

    Ok(result)
}

/// Simple URL highlighter
fn highlight_urls(text: &str) -> Result<Text, LuxorError> {
    let mut result = Text::new(text);
    let url_style = Style::new().underline().color(Color::rgb(0, 100, 255)); // Blue

    // Look for http:// or https://
    if let Some(start) = text.find("http://") {
        if let Some(end) = text[start..].find(char::is_whitespace) {
            result.stylize_range(start..start + end, url_style.clone())?;
        } else {
            result.stylize_range(start..text.len(), url_style.clone())?;
        }
    }

    if let Some(start) = text.find("https://") {
        if let Some(end) = text[start..].find(char::is_whitespace) {
            result.stylize_range(start..start + end, url_style.clone())?;
        } else {
            result.stylize_range(start..text.len(), url_style.clone())?;
        }
    }

    Ok(result)
}

/// Rust keyword highlighter
fn highlight_rust_keywords(text: &str) -> Result<Text, LuxorError> {
    let mut result = Text::new(text);

    let keywords = [
        ("fn", Color::rgb(255, 100, 0)),     // Orange
        ("let", Color::rgb(0, 255, 100)),    // Green
        ("mut", Color::rgb(255, 200, 0)),    // Yellow
        ("if", Color::rgb(200, 0, 255)),     // Purple
        ("return", Color::rgb(255, 0, 100)), // Pink
        ("Ok", Color::rgb(0, 200, 200)),     // Cyan
    ];

    for (keyword, color) in &keywords {
        let mut start = 0;
        while let Some(pos) = text[start..].find(keyword) {
            let absolute_pos = start + pos;
            let keyword_end = absolute_pos + keyword.len();

            // Check if it's a whole word (simple check)
            let is_word_boundary = (absolute_pos == 0
                || !text
                    .chars()
                    .nth(absolute_pos - 1)
                    .unwrap()
                    .is_alphanumeric())
                && (keyword_end == text.len()
                    || !text.chars().nth(keyword_end).unwrap().is_alphanumeric());

            if is_word_boundary {
                result
                    .stylize_range(absolute_pos..keyword_end, Style::new().bold().color(*color))?;
            }

            start = absolute_pos + 1;
        }
    }

    Ok(result)
}

/// Number highlighter
fn highlight_numbers(text: &str) -> Result<Text, LuxorError> {
    let mut result = Text::new(text);
    let number_style = Style::new().color(Color::rgb(100, 255, 100)); // Light green

    let chars: Vec<char> = text.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        if chars[i].is_ascii_digit()
            || (chars[i] == '-' && i + 1 < chars.len() && chars[i + 1].is_ascii_digit())
        {
            let start = i;

            // Skip optional minus
            if chars[i] == '-' {
                i += 1;
            }

            // Skip digits
            while i < chars.len() && chars[i].is_ascii_digit() {
                i += 1;
            }

            // Skip decimal point and more digits
            if i < chars.len() && chars[i] == '.' {
                i += 1;
                while i < chars.len() && chars[i].is_ascii_digit() {
                    i += 1;
                }
            }

            // Skip scientific notation
            if i < chars.len() && (chars[i] == 'e' || chars[i] == 'E') {
                i += 1;
                if i < chars.len() && (chars[i] == '+' || chars[i] == '-') {
                    i += 1;
                }
                while i < chars.len() && chars[i].is_ascii_digit() {
                    i += 1;
                }
            }

            let byte_start = char_pos_to_byte_pos(text, start);
            let byte_end = char_pos_to_byte_pos(text, i);
            result.stylize_range(byte_start..byte_end, number_style.clone())?;
        } else {
            i += 1;
        }
    }

    Ok(result)
}

/// Find email pattern starting at position (returns char positions)
#[allow(clippy::type_complexity)]
fn find_email_at_position(chars: &[char], start: usize) -> Option<(usize, usize)> {
    if start >= chars.len() {
        return None;
    }

    // Look for pattern: alphanumeric@alphanumeric.alphanumeric
    let mut i = start;

    // Find start of potential email (alphanumeric or some special chars)
    while i < chars.len() && !chars[i].is_alphanumeric() {
        i += 1;
    }

    if i >= chars.len() {
        return None;
    }

    let email_start = i;

    // Skip to @
    while i < chars.len()
        && chars[i] != '@'
        && (chars[i].is_alphanumeric() || chars[i] == '.' || chars[i] == '-' || chars[i] == '_')
    {
        i += 1;
    }

    if i >= chars.len() || chars[i] != '@' {
        return None;
    }

    i += 1; // Skip @

    // Skip domain part
    while i < chars.len() && (chars[i].is_alphanumeric() || chars[i] == '.' || chars[i] == '-') {
        i += 1;
    }

    // Must have at least some domain
    if i <= email_start + 2 {
        return None;
    }

    Some((email_start, i))
}

/// Convert character position to byte position
fn char_pos_to_byte_pos(text: &str, char_pos: usize) -> usize {
    text.char_indices()
        .nth(char_pos)
        .map(|(pos, _)| pos)
        .unwrap_or(text.len())
}
