//! Showcase all available spinners in a live grid panel (Ctrl+C to exit),
//! similar to rich/examples/spinners.py but limited to Luxor's spinner set.

use luxor::{animation::Spinners, Color, Console, ConsoleOptions, Result, Style, Text};
use std::{thread, time::Duration};

fn pad_to(text: &str, width: usize) -> String {
    let len = text.chars().count();
    if len >= width {
        text.chars().take(width).collect()
    } else {
        format!("{}{}", text, " ".repeat(width - len))
    }
}

fn main() -> Result<()> {
    // Fix width for consistent layout; feel free to adjust
    let options = ConsoleOptions::new().with_max_width(90);
    let console = Console::with_options(options);

    // Prepare animations
    let names = Spinners::available_names();
    let mut spinners: Vec<(&str, _)> = names
        .into_iter()
        .filter_map(|name| Spinners::by_name(name).map(|anim| (name, anim)))
        .collect();
    for (_, anim) in spinners.iter_mut() {
        anim.start();
    }

    // Layout config
    let columns = 4usize;
    let cell_width = 20usize; // frame + name fits
    let panel_inner_width = columns * cell_width + (columns - 1); // spaces between cells
    let panel_width = panel_inner_width + 2; // borders

    // Styles
    let border_style = Style::new().color(Color::rgb(80, 140, 255));
    let title_style = Style::new().bold().color(Color::rgb(80, 140, 255));
    let name_style = Style::new().color(Color::rgb(0, 200, 0));

    console.hide_cursor()?;

    loop {
        // Build rows of cells
        let mut lines: Vec<String> = Vec::new();

        // Title line inside panel
        let title = " Spinners ";
        let title_padded = {
            let tlen = title.chars().count();
            let rem = panel_inner_width.saturating_sub(tlen);
            let left = rem / 2;
            let right = rem - left;
            format!("{}{}{}", "─".repeat(left), title, "─".repeat(right))
        };
        lines.push(title_padded);

        // Grid of spinner frames and names
        for chunk in spinners.chunks_mut(columns) {
            // First row: frames
            let mut row1: Vec<String> = Vec::new();
            // Second row: names
            let mut row2: Vec<String> = Vec::new();

            for (name, anim) in chunk.iter_mut() {
                let frame = anim.current_frame().unwrap_or("");
                let cell1 = pad_to(frame, cell_width);
                let cell2 = pad_to(&format!("'{}'", name), cell_width);
                row1.push(cell1);
                row2.push(cell2);
                let _ = anim.update();
            }

            // Join cells with a single space
            lines.push(row1.join(" "));
            lines.push(row2.join(" "));
        }

        // Draw panel with border
        // Top border
        let top = format!("┌{}┐", "─".repeat(panel_inner_width));
        // Bottom border
        let bottom = format!("└{}┘", "─".repeat(panel_inner_width));

        // Clear and print
        console.clear()?;
        console.println(Text::new(&top).with_style(border_style.clone()))?;
        // Title line styled
        let title_line = format!("│{}│", lines[0]);
        console.println(Text::new(&title_line).with_style(title_style.clone()))?;
        // Content lines
        for line in lines.iter().skip(1) {
            let content_line = format!("│{}│", line);
            console.println(Text::new(&content_line).with_style(Style::new()))?;
        }
        console.println(Text::new(&bottom).with_style(border_style.clone()))?;

        // Slight delay ~20 FPS
        thread::sleep(Duration::from_millis(50));
    }
}


