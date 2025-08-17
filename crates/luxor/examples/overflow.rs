//! Text Overflow Handling Example
//!
//! This example demonstrates different approaches to handling text overflow
//! when text is longer than the available width.

use luxor::{Color, Console, LuxorError, Style, Text};

fn main() -> Result<(), LuxorError> {
    let console = Console::new();

    println!("=== Text Overflow Example ===\n");

    let long_text = "supercalifragilisticexpialidocious";
    let width = 14;

    // Demonstrate different overflow methods
    demonstrate_crop_overflow(&console, long_text, width)?;
    demonstrate_ellipsis_overflow(&console, long_text, width)?;
    demonstrate_wrap_overflow(&console, long_text, width)?;
    demonstrate_styled_overflow(&console)?;

    Ok(())
}

fn demonstrate_crop_overflow(
    console: &Console,
    text: &str,
    width: usize,
) -> Result<(), LuxorError> {
    println!("1. Crop overflow (cut off at boundary):");
    println!("   Text: '{}' (length: {})", text, text.len());
    println!("   Width limit: {}", width);

    let cropped = crop_text(text, width);
    let border = "─".repeat(width);

    console.print(Text::new(&border).with_style(Style::new().color(Color::rgb(100, 100, 100))))?;
    console.print(
        Text::new(&cropped).with_style(Style::new().bold().color(Color::rgb(0, 100, 255))),
    )?;
    console.print(Text::new(&border).with_style(Style::new().color(Color::rgb(100, 100, 100))))?;
    println!();
    Ok(())
}

fn demonstrate_ellipsis_overflow(
    console: &Console,
    text: &str,
    width: usize,
) -> Result<(), LuxorError> {
    println!("2. Ellipsis overflow (show ... at end):");

    let ellipsis_text = add_ellipsis(text, width);
    let border = "─".repeat(width);

    console.print(Text::new(&border).with_style(Style::new().color(Color::rgb(100, 100, 100))))?;
    console.print(
        Text::new(&ellipsis_text).with_style(Style::new().bold().color(Color::rgb(255, 100, 0))),
    )?;
    console.print(Text::new(&border).with_style(Style::new().color(Color::rgb(100, 100, 100))))?;
    println!();
    Ok(())
}

fn demonstrate_wrap_overflow(
    console: &Console,
    text: &str,
    width: usize,
) -> Result<(), LuxorError> {
    println!("3. Wrap overflow (break into multiple lines):");

    let wrapped_lines = wrap_text(text, width);
    let border = "─".repeat(width);

    console.print(Text::new(&border).with_style(Style::new().color(Color::rgb(100, 100, 100))))?;
    for line in wrapped_lines {
        console.print(
            Text::new(&line).with_style(Style::new().bold().color(Color::rgb(0, 255, 100))),
        )?;
    }
    console.print(Text::new(&border).with_style(Style::new().color(Color::rgb(100, 100, 100))))?;
    println!();
    Ok(())
}

fn demonstrate_styled_overflow(console: &Console) -> Result<(), LuxorError> {
    println!("4. Styled text with overflow handling:");

    let texts = [
        ("Short", 15),
        ("Medium length text", 15),
        ("This is a very long text that will definitely overflow", 15),
    ];

    for (text, width) in texts {
        println!("   Original: '{}'", text);

        // Crop with style
        let cropped = crop_text(text, width);
        let mut crop_text_obj = Text::new(&format!("Crop: {}", cropped));
        crop_text_obj = crop_text_obj.with_style(Style::new().color(Color::rgb(255, 100, 100)));
        console.print(crop_text_obj)?;

        // Ellipsis with style
        let ellipsis = add_ellipsis(text, width);
        let mut ellipsis_text_obj = Text::new(&format!("Ellipsis: {}", ellipsis));
        ellipsis_text_obj =
            ellipsis_text_obj.with_style(Style::new().color(Color::rgb(100, 255, 100)));
        console.print(ellipsis_text_obj)?;

        println!();
    }

    Ok(())
}

/// Crop text to fit within width
fn crop_text(text: &str, width: usize) -> String {
    if text.chars().count() <= width {
        text.to_string()
    } else {
        text.chars().take(width).collect()
    }
}

/// Add ellipsis if text is too long
fn add_ellipsis(text: &str, width: usize) -> String {
    if width < 3 {
        // Not enough space for ellipsis
        return crop_text(text, width);
    }

    if text.chars().count() <= width {
        text.to_string()
    } else {
        let mut result: String = text.chars().take(width - 3).collect();
        result.push_str("...");
        result
    }
}

/// Wrap text into multiple lines
fn wrap_text(text: &str, width: usize) -> Vec<String> {
    if width == 0 {
        return vec![text.to_string()];
    }

    let chars: Vec<char> = text.chars().collect();
    let mut lines = Vec::new();
    let mut current_line = String::new();
    let mut current_width = 0;

    for ch in chars {
        if current_width >= width {
            lines.push(current_line);
            current_line = String::new();
            current_width = 0;
        }

        current_line.push(ch);
        current_width += 1;
    }

    if !current_line.is_empty() {
        lines.push(current_line);
    }

    if lines.is_empty() {
        vec![String::new()]
    } else {
        lines
    }
}
