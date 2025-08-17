//! Console - the central rendering engine for rich text output.

use crate::{
    ColorSystem, LuxorError, Measurable, Measurement, Renderable, Result, Segment, Style, ansi,
};
use crossterm::terminal;
use std::io::{self, Write};

/// Options that control how rendering is performed.
#[derive(Debug, Clone)]
pub struct ConsoleOptions {
    /// Maximum width for rendering (None = use terminal width).
    pub max_width: Option<usize>,
    /// Minimum width for rendering.
    pub min_width: usize,
    /// Whether to use ANSI color codes.
    pub enable_color: bool,
    /// Color system capability.
    pub color_system: ColorSystem,
    /// Whether to use alternative screen buffer.
    pub alt_screen: bool,
    /// Legacy Windows mode (for compatibility).
    pub legacy_windows: bool,
}

impl ConsoleOptions {
    /// Create new console options with default settings.
    pub fn new() -> Self {
        Self {
            max_width: None,
            min_width: 0,
            enable_color: true,
            color_system: ColorSystem::detect(),
            alt_screen: false,
            legacy_windows: false,
        }
    }

    /// Set the maximum width.
    pub fn with_max_width(mut self, width: usize) -> Self {
        self.max_width = Some(width);
        self
    }

    /// Set the minimum width.
    pub fn with_min_width(mut self, width: usize) -> Self {
        self.min_width = width;
        self
    }

    /// Enable or disable color output.
    pub fn with_color(mut self, enable: bool) -> Self {
        self.enable_color = enable;
        self
    }

    /// Set the color system.
    pub fn with_color_system(mut self, color_system: ColorSystem) -> Self {
        self.color_system = color_system;
        self
    }

    /// Get the effective maximum width, using terminal width if not set.
    pub fn get_max_width(&self) -> usize {
        self.max_width
            .unwrap_or_else(|| terminal::size().map(|(w, _)| w as usize).unwrap_or(80))
    }

    /// Get the effective color system, respecting the enable_color setting.
    pub fn get_color_system(&self) -> ColorSystem {
        if self.enable_color {
            self.color_system
        } else {
            ColorSystem::Standard // Minimal color support when disabled
        }
    }
}

impl Default for ConsoleOptions {
    fn default() -> Self {
        Self::new()
    }
}

/// The central console for rendering rich text.
///
/// The Console is responsible for:
/// - Managing terminal capabilities and settings
/// - Coordinating the rendering of complex objects
/// - Handling output streams and buffering
/// - Providing measurement and layout services
#[derive(Debug)]
pub struct Console {
    /// Console options and settings.
    options: ConsoleOptions,
    /// Current terminal width (cached).
    width: Option<usize>,
    /// Current terminal height (cached).
    height: Option<usize>,
    /// Whether we're in alternative screen mode.
    in_alt_screen: bool,
}

impl Console {
    /// Create a new console with default settings.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use luxor::Console;
    ///
    /// let console = Console::new();
    /// ```
    pub fn new() -> Self {
        Self {
            options: ConsoleOptions::default(),
            width: None,
            height: None,
            in_alt_screen: false,
        }
    }

    /// Create a new console with the given options.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use luxor::{Console, ConsoleOptions, ColorSystem};
    ///
    /// let options = ConsoleOptions::new()
    ///     .with_max_width(120)
    ///     .with_color_system(ColorSystem::TrueColor);
    /// let console = Console::with_options(options);
    /// ```
    pub fn with_options(options: ConsoleOptions) -> Self {
        Self {
            options,
            width: None,
            height: None,
            in_alt_screen: false,
        }
    }

    /// Get the console options.
    pub fn options(&self) -> &ConsoleOptions {
        &self.options
    }

    /// Get the terminal width, detecting it if not cached.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use luxor::Console;
    ///
    /// let console = Console::new();
    /// let width = console.width();
    /// println!("Terminal width: {}", width);
    /// ```
    pub fn width(&self) -> usize {
        self.width.unwrap_or_else(|| {
            self.options
                .max_width
                .unwrap_or_else(|| terminal::size().map(|(w, _)| w as usize).unwrap_or(80))
        })
    }

    /// Get the terminal height, detecting it if not cached.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use luxor::Console;
    ///
    /// let console = Console::new();
    /// let height = console.height();
    /// println!("Terminal height: {}", height);
    /// ```
    pub fn height(&self) -> usize {
        self.height
            .unwrap_or_else(|| terminal::size().map(|(_, h)| h as usize).unwrap_or(24))
    }

    /// Get the size of the terminal as (width, height).
    pub fn size(&self) -> (usize, usize) {
        (self.width(), self.height())
    }

    /// Update the cached terminal size.
    ///
    /// This should be called when the terminal is resized.
    pub fn update_size(&mut self) -> Result<()> {
        match terminal::size() {
            Ok((w, h)) => {
                self.width = Some(w as usize);
                self.height = Some(h as usize);
                Ok(())
            }
            Err(e) => Err(LuxorError::terminal(format!(
                "Failed to get terminal size: {}",
                e
            ))),
        }
    }

    /// Enable alternative screen buffer.
    ///
    /// This switches to an alternative screen buffer, allowing you to draw
    /// full-screen applications without affecting the terminal history.
    pub fn enable_alt_screen(&mut self) -> Result<()> {
        if !self.in_alt_screen {
            print!("{}", ansi::codes::ALT_SCREEN_ENABLE);
            io::stdout().flush()?;
            self.in_alt_screen = true;
        }
        Ok(())
    }

    /// Disable alternative screen buffer.
    ///
    /// This returns to the normal screen buffer.
    pub fn disable_alt_screen(&mut self) -> Result<()> {
        if self.in_alt_screen {
            print!("{}", ansi::codes::ALT_SCREEN_DISABLE);
            io::stdout().flush()?;
            self.in_alt_screen = false;
        }
        Ok(())
    }

    /// Clear the screen.
    pub fn clear(&self) -> Result<()> {
        print!("{}{}", ansi::codes::CLEAR_SCREEN, ansi::codes::CURSOR_HOME);
        io::stdout().flush()?;
        Ok(())
    }

    /// Hide the cursor.
    pub fn hide_cursor(&self) -> Result<()> {
        print!("{}", ansi::codes::CURSOR_HIDE);
        io::stdout().flush()?;
        Ok(())
    }

    /// Show the cursor.
    pub fn show_cursor(&self) -> Result<()> {
        print!("{}", ansi::codes::CURSOR_SHOW);
        io::stdout().flush()?;
        Ok(())
    }

    /// Print a renderable object to the console.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use luxor::{Console, Style, Color};
    ///
    /// let console = Console::new();
    /// let styled_text = "Hello, world!";
    /// console.print(styled_text).unwrap();
    /// ```
    pub fn print<R: Renderable>(&self, renderable: R) -> Result<()> {
        let segments = renderable.render(self, &self.options)?;
        self.write_segments(&segments)?;
        Ok(())
    }

    /// Print a renderable object followed by a newline.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use luxor::{Console, Style, Color};
    ///
    /// let console = Console::new();
    /// console.println("Hello, world!").unwrap();
    /// ```
    pub fn println<R: Renderable>(&self, renderable: R) -> Result<()> {
        self.print(renderable)?;
        println!();
        Ok(())
    }

    /// Render a renderable object to segments without printing.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use luxor::{Console, ConsoleOptions};
    ///
    /// let console = Console::new();
    /// let options = ConsoleOptions::new();
    /// let segments = console.render("Hello, world!", &options).unwrap();
    /// ```
    pub fn render<R: Renderable>(
        &self,
        renderable: R,
        options: &ConsoleOptions,
    ) -> Result<Vec<Segment>> {
        renderable.render(self, options)
    }

    /// Measure a renderable object.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use luxor::{Console, ConsoleOptions};
    ///
    /// let console = Console::new();
    /// let options = ConsoleOptions::new();
    /// let measurement = console.measure("Hello, world!", &options).unwrap();
    /// println!("Width: {} - {}", measurement.minimum(), measurement.maximum());
    /// ```
    pub fn measure<M: Measurable>(
        &self,
        measurable: M,
        options: &ConsoleOptions,
    ) -> Result<Measurement> {
        measurable.measure(self, options)
    }

    /// Write segments directly to the output.
    fn write_segments(&self, segments: &[Segment]) -> Result<()> {
        for segment in segments {
            let output = segment.render(self.options.get_color_system());
            print!("{}", output);
        }
        io::stdout().flush()?;
        Ok(())
    }

    /// Create a styled string with the given style.
    ///
    /// This is a convenience method for applying a style to a string.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use luxor::{Console, Style, Color};
    ///
    /// let console = Console::new();
    /// let styled = console.styled("Hello", Style::new().color(Color::rgb(255, 0, 0)));
    /// console.print(styled).unwrap();
    /// ```
    pub fn styled(&self, text: &str, style: Style) -> StyledText {
        StyledText {
            text: text.to_string(),
            style,
        }
    }

    /// Get console options suitable for the current terminal.
    pub fn get_render_options(&self) -> ConsoleOptions {
        self.options.clone()
    }

    /// Check if color output is enabled and supported.
    pub fn color_enabled(&self) -> bool {
        self.options.enable_color
    }

    /// Get the effective color system being used.
    pub fn color_system(&self) -> ColorSystem {
        self.options.get_color_system()
    }
}

impl Default for Console {
    fn default() -> Self {
        Self::new()
    }
}

/// A styled text object that can be rendered by the console.
#[derive(Debug, Clone)]
pub struct StyledText {
    text: String,
    style: Style,
}

impl StyledText {
    /// Create new styled text.
    pub fn new(text: String, style: Style) -> Self {
        Self { text, style }
    }

    /// Get the text content.
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Get the style.
    pub fn style(&self) -> &Style {
        &self.style
    }
}

impl Renderable for StyledText {
    fn render(&self, _console: &Console, _options: &ConsoleOptions) -> Result<Vec<Segment>> {
        Ok(vec![Segment::new(self.text.clone(), self.style.clone())])
    }
}

impl Measurable for StyledText {
    fn measure(&self, _console: &Console, _options: &ConsoleOptions) -> Result<Measurement> {
        use unicode_width::UnicodeWidthStr;
        let width = self.text.width();
        Ok(Measurement::fixed(width))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Color, Style, Text};

    #[test]
    fn test_console_new() {
        let console = Console::new();
        assert!(console.width() > 0);
        assert!(console.height() > 0);
    }

    #[test]
    fn test_console_options() {
        let options = ConsoleOptions::new()
            .with_max_width(120)
            .with_min_width(20)
            .with_color(false);

        let console = Console::with_options(options);
        assert_eq!(console.width(), 120);
        assert!(!console.color_enabled());
    }

    #[test]
    fn test_styled_text() {
        let style = Style::new().bold().color(Color::rgb(255, 0, 0));
        let styled = StyledText::new("Hello".to_string(), style.clone());

        assert_eq!(styled.text(), "Hello");
        assert_eq!(styled.style(), &style);
    }

    #[test]
    fn test_styled_text_renderable() {
        let console = Console::new();
        let options = ConsoleOptions::default();
        let styled = StyledText::new("Hello".to_string(), Style::new().bold());

        let segments = styled.render(&console, &options).unwrap();
        assert_eq!(segments.len(), 1);
        assert_eq!(segments[0].text(), "Hello");
        assert_eq!(segments[0].style().bold, Some(true));
    }

    #[test]
    fn test_text() {
        let text = Text::new("Hello").with_style(Style::new().italic());
        assert_eq!(text.plain(), "Hello");
        assert_eq!(text.base_style().italic, Some(true));
    }

    #[test]
    fn test_text_renderable() {
        let console = Console::new();
        let options = ConsoleOptions::default();
        let text = Text::new("Hello").with_style(Style::new().bold());

        let segments = text.render(&console, &options).unwrap();
        assert_eq!(segments.len(), 1);
        assert_eq!(segments[0].text(), "Hello");
    }

    #[test]
    fn test_text_measurable() {
        let console = Console::new();
        let options = ConsoleOptions::default();
        let text = Text::new("Hello");

        let measurement = text.measure(&console, &options).unwrap();
        assert_eq!(measurement.minimum(), 5);
        assert_eq!(measurement.maximum(), 5);
    }

    #[test]
    fn test_console_styled() {
        let console = Console::new();
        let style = Style::new().bold();
        let styled = console.styled("Hello", style.clone());

        assert_eq!(styled.text(), "Hello");
        assert_eq!(styled.style(), &style);
    }

    #[test]
    fn test_console_options_get_max_width() {
        let options = ConsoleOptions::new().with_max_width(100);
        assert_eq!(options.get_max_width(), 100);

        let options = ConsoleOptions::new();
        assert!(options.get_max_width() > 0); // Should use terminal width
    }

    #[test]
    fn test_console_options_color_system() {
        let options = ConsoleOptions::new().with_color(true);
        assert_ne!(options.get_color_system(), ColorSystem::Standard); // Should detect system

        let options = ConsoleOptions::new().with_color(false);
        assert_eq!(options.get_color_system(), ColorSystem::Standard);
    }
}
