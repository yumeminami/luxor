//! Demonstrate a simple progress fill animation.

use luxor::{animation::progress_fill_animation, Console, ConsoleOptions, Result, Style, Color, Text};
use std::{thread, time::Duration};

fn main() -> Result<()> {
    let options = ConsoleOptions::new().with_max_width(40);
    let console = Console::with_options(options);

    let mut anim = progress_fill_animation(30, '█', '░', Duration::from_millis(80));
    anim.start();

    while !anim.is_finished() {
        if let Some(frame) = anim.current_frame() {
            let styled = Text::new(frame).with_style(Style::new().bold().color(Color::rgb(0, 200, 100)));
            console.print(styled)?;
            print!("\r");
        }
        let _ = anim.update();
        thread::sleep(Duration::from_millis(40));
    }

    if let Some(frame) = anim.current_frame() {
        let styled = Text::new(frame).with_style(Style::new().bold().color(Color::rgb(0, 200, 100)));
        console.println(styled)?;
    }

    Ok(())
}


