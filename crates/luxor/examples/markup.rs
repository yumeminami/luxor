//! Markup Parsing Demonstration
//!
//! This example showcases the rich markup parsing capabilities of Luxor,
//! demonstrating BBCode-style syntax for styling text.

use luxor::{Console, LuxorError, Text};

fn main() -> Result<(), LuxorError> {
    let console = Console::new();

    println!("=== Markup Parsing Demonstration ===\n");

    // Demonstrate different markup features
    demonstrate_basic_markup(&console)?;
    demonstrate_nested_markup(&console)?;
    demonstrate_color_markup(&console)?;
    demonstrate_complex_markup(&console)?;
    demonstrate_error_handling(&console)?;

    Ok(())
}

fn demonstrate_basic_markup(console: &Console) -> Result<(), LuxorError> {
    println!("1. Basic markup tags:");

    let examples = [
        "[bold]Bold text[/bold]",
        "[italic]Italic text[/italic]",
        "[underline]Underlined text[/underline]",
        "[strikethrough]Strikethrough text[/strikethrough]",
        "[dim]Dimmed text[/dim]",
    ];

    for example in examples {
        println!("   Input: {}", example);
        let text = Text::from_markup(example)?;
        print!("   Output: ");
        console.print(text)?;
        println!();
    }
    println!();
    Ok(())
}

fn demonstrate_nested_markup(console: &Console) -> Result<(), LuxorError> {
    println!("2. Nested markup:");

    let examples = [
        "[bold]Bold and [italic]italic[/italic] combined[/bold]",
        "[red]Red with [bold]bold[/bold] inside[/red]",
        "[underline]Underlined [green]green[/green] text[/underline]",
        "[bold][italic][underline]Triple nested[/underline][/italic][/bold]",
    ];

    for example in examples {
        println!("   Input: {}", example);
        let text = Text::from_markup(example)?;
        print!("   Output: ");
        console.print(text)?;
        println!();
    }
    println!();
    Ok(())
}

fn demonstrate_color_markup(console: &Console) -> Result<(), LuxorError> {
    println!("3. Color markup:");

    let examples = [
        "[red]Red text[/red]",
        "[green]Green text[/green]",
        "[blue]Blue text[/blue]",
        "[yellow]Yellow text[/yellow]",
        "[magenta]Magenta text[/magenta]",
        "[cyan]Cyan text[/cyan]",
    ];

    for example in examples {
        println!("   Input: {}", example);
        let text = Text::from_markup(example)?;
        print!("   Output: ");
        console.print(text)?;
        println!();
    }
    println!();
    Ok(())
}

fn demonstrate_complex_markup(console: &Console) -> Result<(), LuxorError> {
    println!("4. Complex markup combinations:");

    let examples = [
        "[bold red]Bold red text[/bold red]",
        "[italic blue]Italic blue text[/italic blue]",
        "[bold][red]Nested bold red[/red][/bold]",
        "[underline green]Underlined green with [yellow]yellow[/yellow] inside[/underline green]",
    ];

    for example in examples {
        println!("   Input: {}", example);
        let text = Text::from_markup(example)?;
        print!("   Output: ");
        console.print(text)?;
        println!();
    }
    println!();
    Ok(())
}

fn demonstrate_error_handling(console: &Console) -> Result<(), LuxorError> {
    println!("5. Error handling:");

    let examples = [
        "[bold]Unclosed bold tag",        // Missing closing tag
        "[invalid]Invalid tag[/invalid]", // Invalid tag
        "[bold]Mismatched[/italic] tags", // Mismatched tags
    ];

    for example in examples {
        println!("   Input: {}", example);
        match Text::from_markup(example) {
            Ok(text) => {
                print!("   Output: ");
                console.print(text)?;
                println!();
            }
            Err(e) => {
                println!("   Error: {:?}", e);
            }
        }
    }
    println!();
    Ok(())
}
