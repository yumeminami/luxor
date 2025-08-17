//! Demonstrate a looping wave animation.

use luxor::{animation::wave_animation, Console, ConsoleOptions, Result, Text};
use std::{thread, time::Duration};

fn main() -> Result<()> {
    let options = ConsoleOptions::new().with_max_width(50);
    let console = Console::with_options(options);

    let mut anim = wave_animation(40, '~', '.', 6, Duration::from_millis(60));
    anim.start();

    for _ in 0..60 {
        if let Some(frame) = anim.current_frame() {
            console.print(Text::new(frame))?;
            print!("\r");
        }
        let _ = anim.update();
        thread::sleep(Duration::from_millis(40));
    }

    println!();
    Ok(())
}


