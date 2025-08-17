//! Demonstrate the `Rule` component: basic, with title, alignment, and custom style.

use luxor::{
    Color, Console, ConsoleOptions, Result, Rule, Style,
    Align,
};

fn main() -> Result<()> {
    // Fix the width so the demo looks consistent regardless of terminal size
    let options = ConsoleOptions::new().with_max_width(40);
    let console = Console::with_options(options);

    console.println("Basic horizontal rule:")?;
    console.println(Rule::new())?;

    console.println("\nCentered title:")?;
    console.println(Rule::with_title("Section").with_align(Align::Center))?;

    console.println("\nRight-aligned title:")?;
    console.println(Rule::with_title("Details").with_align(Align::Right))?;

    console.println("\nCustom character and style:")?;
    let style = Style::new().bold().color(Color::rgb(255, 128, 0));
    console.println(Rule::new().with_character('=').with_style(style))?;

    Ok(())
}


