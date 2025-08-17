//! Rainbow Text Example
//!
//! This example demonstrates how to create rainbow-colored text by applying
//! different colors to each character, similar to the Rich Python rainbow example.

use luxor::{Color, Console, LuxorError, Style, Text};

fn main() -> Result<(), LuxorError> {
    let console = Console::new();

    println!("=== Rainbow Text Example ===\n");

    // Create rainbow text with different approaches
    create_rainbow_sentence(&console)?;
    create_gradient_text(&console)?;
    create_hue_cycle_text(&console)?;

    Ok(())
}

fn create_rainbow_sentence(console: &Console) -> Result<(), LuxorError> {
    println!("1. Rainbow sentence (character-by-character coloring):");

    let text = "I must not fear. Fear is the mind-killer.";
    let mut rainbow_text = Text::new(text);

    // Apply rainbow colors to each character
    let color_count = 6;
    let colors = [
        Color::rgb(255, 0, 0),   // Red
        Color::rgb(255, 165, 0), // Orange
        Color::rgb(255, 255, 0), // Yellow
        Color::rgb(0, 255, 0),   // Green
        Color::rgb(0, 0, 255),   // Blue
        Color::rgb(128, 0, 128), // Purple
    ];

    for (i, _) in text.char_indices() {
        let next_pos = text
            .char_indices()
            .nth(i + 1)
            .map(|(pos, _)| pos)
            .unwrap_or(text.len());
        let color = colors[i % color_count];
        rainbow_text.stylize_range(i..next_pos, Style::new().color(color))?;
    }

    console.print(rainbow_text)?;
    println!();
    Ok(())
}

fn create_gradient_text(console: &Console) -> Result<(), LuxorError> {
    println!("2. Gradient text (smooth color transition):");

    let text = "This is a smooth gradient from red to blue";
    let mut gradient_text = Text::new(text);

    let char_count = text.chars().count();

    for (i, (byte_pos, _)) in text.char_indices().enumerate() {
        let next_pos = text
            .char_indices()
            .nth(i + 1)
            .map(|(pos, _)| pos)
            .unwrap_or(text.len());

        // Calculate color interpolation (red to blue)
        let ratio = i as f32 / (char_count - 1) as f32;
        let red = ((1.0 - ratio) * 255.0) as u8;
        let blue = (ratio * 255.0) as u8;
        let color = Color::rgb(red, 0, blue);

        gradient_text.stylize_range(byte_pos..next_pos, Style::new().color(color))?;
    }

    console.print(gradient_text)?;
    println!();
    Ok(())
}

fn create_hue_cycle_text(console: &Console) -> Result<(), LuxorError> {
    println!("3. HSV hue cycle (cycling through all hues):");

    let text = "Luxor: Rich text rendering for Rust!";
    let mut hue_text = Text::new(text);

    let char_count = text.chars().count();

    for (i, (byte_pos, _)) in text.char_indices().enumerate() {
        let next_pos = text
            .char_indices()
            .nth(i + 1)
            .map(|(pos, _)| pos)
            .unwrap_or(text.len());

        // Calculate hue (0-360 degrees)
        let hue = (i as f32 / char_count as f32) * 360.0;
        let color = hsv_to_rgb(hue, 1.0, 1.0);

        hue_text.stylize_range(byte_pos..next_pos, Style::new().color(color))?;
    }

    console.print(hue_text)?;
    println!();
    Ok(())
}

/// Convert HSV color to RGB
fn hsv_to_rgb(h: f32, s: f32, v: f32) -> Color {
    let c = v * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = v - c;

    let (r_prime, g_prime, b_prime) = match h {
        h if h < 60.0 => (c, x, 0.0),
        h if h < 120.0 => (x, c, 0.0),
        h if h < 180.0 => (0.0, c, x),
        h if h < 240.0 => (0.0, x, c),
        h if h < 300.0 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };

    let r = ((r_prime + m) * 255.0) as u8;
    let g = ((g_prime + m) * 255.0) as u8;
    let b = ((b_prime + m) * 255.0) as u8;

    Color::rgb(r, g, b)
}
