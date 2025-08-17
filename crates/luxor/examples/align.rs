//! Text Alignment Example
//!
//! This example demonstrates different text alignment approaches
//! using manual spacing and padding techniques.

use luxor::{Color, Console, LuxorError, Style, Text};

fn main() -> Result<(), LuxorError> {
    let console = Console::new();

    println!("=== Text Alignment Example ===\n");

    // Demonstrate different alignment approaches
    demonstrate_manual_alignment(&console)?;
    demonstrate_styled_alignment(&console)?;
    demonstrate_centered_titles(&console)?;

    Ok(())
}

fn demonstrate_manual_alignment(console: &Console) -> Result<(), LuxorError> {
    println!("1. Manual alignment with padding:");

    let width = 30;
    let text = "Luxor";

    // Left alignment (default)
    let left_aligned = format!("{:<width$}", text, width = width);
    console.print(Text::new(&format!("[{}]", left_aligned)))?;

    // Center alignment
    let center_aligned = format!("{:^width$}", text, width = width);
    console.print(Text::new(&format!("[{}]", center_aligned)))?;

    // Right alignment
    let right_aligned = format!("{:>width$}", text, width = width);
    console.print(Text::new(&format!("[{}]", right_aligned)))?;

    println!();
    Ok(())
}

fn demonstrate_styled_alignment(console: &Console) -> Result<(), LuxorError> {
    println!("2. Styled text with alignment:");

    let width = 40;
    let style = Style::new()
        .bold()
        .color(Color::rgb(255, 255, 255))
        .background(Color::rgb(0, 100, 200));

    // Left aligned with style
    let text = format!("{:<width$}", "LEFT", width = width);
    let mut left_text = Text::new(&text);
    left_text = left_text.with_style(style.clone());
    console.print(left_text)?;

    // Center aligned with style
    let text = format!("{:^width$}", "CENTER", width = width);
    let mut center_text = Text::new(&text);
    center_text = center_text.with_style(style.clone());
    console.print(center_text)?;

    // Right aligned with style
    let text = format!("{:>width$}", "RIGHT", width = width);
    let mut right_text = Text::new(&text);
    right_text = right_text.with_style(style);
    console.print(right_text)?;

    println!();
    Ok(())
}

fn demonstrate_centered_titles(console: &Console) -> Result<(), LuxorError> {
    println!("3. Centered titles and headers:");

    // Create a centered title
    create_centered_title(console, "LUXOR LIBRARY", 50)?;

    // Create section headers
    create_section_header(console, "Features", 50)?;
    console.print(Text::new("• Rich text markup parsing"))?;
    console.print(Text::new("• Style composition and inheritance"))?;
    console.print(Text::new("• Unicode-aware text processing"))?;

    println!();

    create_section_header(console, "Performance", 50)?;
    console.print(Text::new("• Zero-cost abstractions"))?;
    console.print(Text::new("• Minimal memory allocations"))?;
    console.print(Text::new("• SIMD-ready architecture"))?;

    println!();
    Ok(())
}

fn create_centered_title(console: &Console, title: &str, width: usize) -> Result<(), LuxorError> {
    // Create decorative border
    let border = "═".repeat(width);
    console.print(Text::new(&border).with_style(Style::new().color(Color::rgb(100, 200, 255))))?;

    // Center the title
    let centered = format!("{:^width$}", title, width = width);
    let mut title_text = Text::new(&centered);
    title_text = title_text.with_style(
        Style::new()
            .bold()
            .color(Color::rgb(255, 255, 255))
            .background(Color::rgb(50, 100, 200)),
    );
    console.print(title_text)?;

    // Bottom border
    console.print(Text::new(&border).with_style(Style::new().color(Color::rgb(100, 200, 255))))?;

    Ok(())
}

fn create_section_header(console: &Console, header: &str, width: usize) -> Result<(), LuxorError> {
    let centered = format!("{:^width$}", header, width = width);
    let mut header_text = Text::new(&centered);
    header_text = header_text.with_style(
        Style::new()
            .bold()
            .underline()
            .color(Color::rgb(255, 200, 100)),
    );
    console.print(header_text)?;

    Ok(())
}
