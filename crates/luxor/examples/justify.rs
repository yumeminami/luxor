//! Demonstrate text justification using layout utilities.

use luxor::{layout::Align, Console, ConsoleOptions, Result, Text};

fn show(console: &Console, label: &str, align: Align) -> Result<()> {
    let width = 40;
    let text = "Luxor brings Rich-like terminal rendering to Rust";
    let aligned = align.apply_to_string(text, width);
    console.println(format!("{}:\n[{}]", label, aligned))
}

fn main() -> Result<()> {
    let options = ConsoleOptions::new().with_max_width(60);
    let console = Console::with_options(options);

    console.println(Text::new("Justification demo:"))?;
    show(&console, "Left", Align::Left)?;
    show(&console, "Center", Align::Center)?;
    show(&console, "Right", Align::Right)?;
    show(&console, "Justify", Align::Justify)?;

    Ok(())
}


