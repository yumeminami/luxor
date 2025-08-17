//! Style Combination Showcase
//!
//! This example demonstrates various text styling capabilities including
//! colors, text attributes, and style combinations.

use luxor::{Color, Console, LuxorError, Style, Text};

fn main() -> Result<(), LuxorError> {
    let console = Console::new();

    println!("=== Style Combination Showcase ===\n");

    // Demonstrate different aspects of styling
    demonstrate_basic_styles(&console)?;
    demonstrate_color_variations(&console)?;
    demonstrate_style_combinations(&console)?;
    demonstrate_background_colors(&console)?;
    demonstrate_style_inheritance(&console)?;

    Ok(())
}

fn demonstrate_basic_styles(console: &Console) -> Result<(), LuxorError> {
    println!("1. Basic text attributes:");

    let styles = [
        ("Normal text", Style::new()),
        ("Bold text", Style::new().bold()),
        ("Italic text", Style::new().italic()),
        ("Underlined text", Style::new().underline()),
        ("Strikethrough text", Style::new().strikethrough()),
        ("Dimmed text", Style::new().dim()),
    ];

    for (description, style) in styles {
        let mut text = Text::new(description);
        text = text.with_style(style);
        console.print(text)?;
    }
    println!();
    Ok(())
}

fn demonstrate_color_variations(console: &Console) -> Result<(), LuxorError> {
    println!("2. Color variations:");

    // Standard colors
    println!("   Standard colors:");
    let standard_colors = [
        ("Red", Color::rgb(255, 0, 0)),
        ("Green", Color::rgb(0, 255, 0)),
        ("Blue", Color::rgb(0, 0, 255)),
        ("Yellow", Color::rgb(255, 255, 0)),
        ("Magenta", Color::rgb(255, 0, 255)),
        ("Cyan", Color::rgb(0, 255, 255)),
        ("White", Color::rgb(255, 255, 255)),
    ];

    for (name, color) in standard_colors {
        let mut text = Text::new(&format!("   {}", name));
        text = text.with_style(Style::new().color(color));
        console.print(text)?;
    }

    // Color gradients
    println!("   RGB color gradient:");
    let text = "Color gradient from red to blue";
    let mut gradient_text = Text::new(text);

    for (i, (pos, _)) in text.char_indices().enumerate() {
        let next_pos = text
            .char_indices()
            .nth(i + 1)
            .map(|(p, _)| p)
            .unwrap_or(text.len());
        let ratio = i as f32 / (text.chars().count() - 1) as f32;
        let red = ((1.0 - ratio) * 255.0) as u8;
        let blue = (ratio * 255.0) as u8;
        let color = Color::rgb(red, 0, blue);
        gradient_text.stylize_range(pos..next_pos, Style::new().color(color))?;
    }

    console.print(gradient_text)?;
    println!();
    Ok(())
}

fn demonstrate_style_combinations(console: &Console) -> Result<(), LuxorError> {
    println!("3. Style combinations:");

    let combinations = [
        (
            "Bold + Red",
            Style::new().bold().color(Color::rgb(255, 0, 0)),
        ),
        (
            "Italic + Green",
            Style::new().italic().color(Color::rgb(0, 255, 0)),
        ),
        (
            "Underline + Blue",
            Style::new().underline().color(Color::rgb(0, 0, 255)),
        ),
        (
            "Bold + Italic + Yellow",
            Style::new().bold().italic().color(Color::rgb(255, 255, 0)),
        ),
        (
            "Bold + Underline + Magenta",
            Style::new()
                .bold()
                .underline()
                .color(Color::rgb(255, 0, 255)),
        ),
        (
            "All attributes + Cyan",
            Style::new()
                .bold()
                .italic()
                .underline()
                .strikethrough()
                .color(Color::rgb(0, 255, 255)),
        ),
    ];

    for (description, style) in combinations {
        let mut text = Text::new(&format!("   {}", description));
        text = text.with_style(style);
        console.print(text)?;
    }
    println!();
    Ok(())
}

fn demonstrate_background_colors(console: &Console) -> Result<(), LuxorError> {
    println!("4. Background colors:");

    let backgrounds = [
        (
            "White on Red",
            Color::rgb(255, 255, 255),
            Color::rgb(255, 0, 0),
        ),
        (
            "Black on Yellow",
            Color::rgb(0, 0, 0),
            Color::rgb(255, 255, 0),
        ),
        (
            "White on Blue",
            Color::rgb(255, 255, 255),
            Color::rgb(0, 0, 255),
        ),
        (
            "Yellow on Purple",
            Color::rgb(255, 255, 0),
            Color::rgb(128, 0, 128),
        ),
        ("Green on Black", Color::rgb(0, 255, 0), Color::rgb(0, 0, 0)),
    ];

    for (description, fg_color, bg_color) in backgrounds {
        let mut text = Text::new(&format!("   {}", description));
        text = text.with_style(Style::new().color(fg_color).background(bg_color));
        console.print(text)?;
    }
    println!();
    Ok(())
}

fn demonstrate_style_inheritance(console: &Console) -> Result<(), LuxorError> {
    println!("5. Style composition and inheritance:");

    // Create a base style
    let base_style = Style::new().bold().color(Color::rgb(100, 100, 255));

    // Combine with other styles
    let mut text = Text::new("This text demonstrates style composition:");
    text = text.with_style(base_style.clone());
    console.print(text)?;

    // Apply additional styles to parts of the text
    let demo_text = "Bold blue base, then italic overlay, then underline addition";
    let mut styled_text = Text::new(demo_text);

    // Apply base style to entire text
    styled_text = styled_text.with_style(base_style.clone());

    // Add italic to a portion
    let italic_style = Style::new().italic();
    styled_text.stylize_range(17..31, italic_style)?; // "italic overlay"

    // Add underline to another portion
    let underline_style = Style::new().underline();
    styled_text.stylize_range(38..56, underline_style)?; // "underline addition"

    console.print(styled_text)?;

    // Demonstrate style combination
    let combined_style = base_style.combine(Style::new().italic().strikethrough());
    let mut combined_text = Text::new("Combined style: bold + blue + italic + strikethrough");
    combined_text = combined_text.with_style(combined_style);
    console.print(combined_text)?;

    println!();
    Ok(())
}

#[allow(dead_code)]
fn demonstrate_color_palette(console: &Console) -> Result<(), LuxorError> {
    println!("6. Color palette showcase:");

    // Create a color palette
    let palette = [
        // Reds
        Color::rgb(255, 0, 0),
        Color::rgb(255, 100, 100),
        Color::rgb(200, 0, 0),
        // Greens
        Color::rgb(0, 255, 0),
        Color::rgb(100, 255, 100),
        Color::rgb(0, 200, 0),
        // Blues
        Color::rgb(0, 0, 255),
        Color::rgb(100, 100, 255),
        Color::rgb(0, 0, 200),
    ];

    let palette_text = "■".repeat(palette.len());
    let mut colored_palette = Text::new(&palette_text);

    for (i, color) in palette.iter().enumerate() {
        let start = i;
        let end = i + 1;
        colored_palette.stylize_range(start..end, Style::new().color(*color))?;
    }

    console.print(Text::new("Color palette: "))?;
    console.print(colored_palette)?;

    Ok(())
}
