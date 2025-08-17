//! Luxor Showcase Example
//!
//! This example demonstrates the current capabilities of Luxor,
//! including rich markup parsing, text styling, and console rendering.

use luxor::{Color, Console, LuxorError, Style, Text};

fn main() -> Result<(), LuxorError> {
    let console = Console::new();

    // Basic colored output
    println!("=== Luxor Showcase ===\n");

    // 1. Basic text styling
    let mut basic_text = Text::new("Hello, Luxor!");
    basic_text = basic_text.with_style(Style::new().bold().color(Color::rgb(0, 255, 0)));
    console.print(basic_text)?;

    // 2. Rich markup parsing
    let markup_text = Text::from_markup("[bold red]Error:[/bold red] Something went wrong!")?;
    console.print(markup_text)?;

    let info_text = Text::from_markup(
        "[bold blue]Info:[/bold blue] [italic]This is informational text[/italic]",
    )?;
    console.print(info_text)?;

    // 3. Style ranges
    let mut range_text = Text::new("This text has multiple styles applied");
    range_text.stylize_range(0..4, Style::new().bold().color(Color::rgb(255, 0, 0)))?; // "This"
    range_text.stylize_range(5..9, Style::new().italic().color(Color::rgb(0, 255, 0)))?; // "text"
    range_text.stylize_range(
        10..13,
        Style::new().underline().color(Color::rgb(0, 0, 255)),
    )?; // "has"
    console.print(range_text)?;

    // 4. Complex nested markup
    let complex = Text::from_markup(
        "[bold]Welcome to [red]Luxor[/red] - [italic blue]Rich text for Rust[/italic blue][/bold]",
    )?;
    console.print(complex)?;

    // 5. Color demonstrations
    println!("\n=== Color System Demo ===");

    // RGB colors (using manual construction since rgb() parsing not implemented yet)
    let mut rgb_demo = Text::new("RGB Color (255,100,50)");
    rgb_demo = rgb_demo.with_style(Style::new().color(Color::rgb(255, 100, 50)));
    console.print(rgb_demo)?;

    // Standard colors
    let colors = ["red", "green", "blue", "yellow", "magenta", "cyan"];
    for color in &colors {
        let colored_text =
            Text::from_markup(&format!("[{}]Standard {} color[/{}]", color, color, color))?;
        console.print(colored_text)?;
    }

    // 6. Style combinations
    println!("\n=== Style Combinations ===");
    let combinations = [
        "[bold red]Bold Red[/bold red]",
        "[italic green]Italic Green[/italic green]",
        "[underline blue]Underline Blue[/underline blue]",
        "[bold italic yellow]Bold Italic Yellow[/bold italic yellow]",
        "[bold underline magenta]Bold Underline Magenta[/bold underline magenta]",
    ];

    for combo in &combinations {
        let styled = Text::from_markup(combo)?;
        console.print(styled)?;
    }

    // 7. Background colors
    println!("\n=== Background Colors ===");
    let bg_text =
        Text::from_markup("[black on white]Black text on white background[/black on white]")?;
    console.print(bg_text)?;

    let bg_text2 = Text::from_markup("[white on red]White text on red background[/white on red]")?;
    console.print(bg_text2)?;

    // 8. Error handling demonstration
    println!("\n=== Error Handling ===");
    match Text::from_markup("[bold [italic]Malformed markup") {
        Ok(_) => println!("This shouldn't happen"),
        Err(e) => println!("Caught error as expected: {}", e),
    }

    // 9. Performance demonstration with large text
    println!("\n=== Performance Test ===");
    let large_text = "Lorem ipsum dolor sit amet ".repeat(50);
    let mut perf_text = Text::new(&large_text);
    perf_text = perf_text.with_style(Style::new().italic().color(Color::rgb(128, 128, 128)));

    let start = std::time::Instant::now();
    console.print(perf_text)?;
    let duration = start.elapsed();
    println!("Rendered {} characters in {:?}", large_text.len(), duration);

    println!("\n=== End of Showcase ===");
    Ok(())
}
