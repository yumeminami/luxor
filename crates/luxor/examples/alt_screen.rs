//! Demonstrate alternative screen with a spinner.

use luxor::{animation::Spinners, Console, Result, Text};
use std::{thread, time::Duration};

fn main() -> Result<()> {
    let mut console = Console::new();
    console.enable_alt_screen()?;
    console.hide_cursor()?;

    let mut spinner = Spinners::dots();
    spinner.start();

    for i in 0..80 {
        if let Some(frame) = spinner.current_frame() {
            console.clear()?;
            console.println(Text::new("Alternative Screen Demo"))?;
            console.println(Text::new("Press Ctrl+C to exit (auto after demo)"))?;
            console.println(Text::new(&format!("Loading {} {}%", frame, (i * 100) / 79)))?;
        }
        let _ = spinner.update();
        thread::sleep(Duration::from_millis(60));
    }

    console.show_cursor()?;
    console.disable_alt_screen()?;
    Ok(())
}


