//! Core traits and protocols for rendering and measurement.

use crate::{Console, ConsoleOptions, Measurement, Result, Segment};

/// The result of a rendering operation - a vector of segments.
pub type RenderResult = Result<Vec<Segment>>;

/// A trait for objects that can be rendered to the console.
///
/// This is the core trait that enables any type to be rendered by Luxor.
/// Implementing this trait allows objects to participate in the rendering pipeline.
///
/// # Examples
///
/// ```rust
/// use luxor::{Renderable, Console, ConsoleOptions, Segment, Style};
///
/// struct SimpleText {
///     content: String,
/// }
///
/// impl Renderable for SimpleText {
///     fn render(&self, _console: &Console, _options: &ConsoleOptions) -> luxor::Result<Vec<Segment>> {
///         Ok(vec![Segment::new(self.content.clone(), Style::default())])
///     }
/// }
/// ```
pub trait Renderable: Send + Sync {
    /// Render this object into a series of segments.
    ///
    /// # Arguments
    ///
    /// * `console` - The console that is performing the rendering
    /// * `options` - Options that control how rendering is performed
    ///
    /// # Returns
    ///
    /// A `Result` containing a vector of `Segment`s that represent the rendered output,
    /// or an error if rendering fails.
    fn render(&self, console: &Console, options: &ConsoleOptions) -> RenderResult;
}

/// A trait for objects that can be measured for layout purposes.
///
/// This trait enables objects to participate in layout calculations by providing
/// their minimum and maximum width requirements.
///
/// # Examples
///
/// ```rust
/// use luxor::{Measurable, Console, ConsoleOptions, Measurement};
///
/// struct FixedWidth {
///     width: usize,
/// }
///
/// impl Measurable for FixedWidth {
///     fn measure(&self, _console: &Console, _options: &ConsoleOptions) -> luxor::Result<Measurement> {
///         Ok(Measurement::new(self.width, self.width))
///     }
/// }
/// ```
pub trait Measurable: Send + Sync {
    /// Calculate the measurement requirements for this object.
    ///
    /// # Arguments
    ///
    /// * `console` - The console that is performing the measurement
    /// * `options` - Options that control how measurement is performed
    ///
    /// # Returns
    ///
    /// A `Result` containing a `Measurement` that describes the minimum and maximum
    /// width requirements, or an error if measurement fails.
    fn measure(&self, console: &Console, options: &ConsoleOptions) -> Result<Measurement>;
}

/// Implement `Renderable` for `String` to enable direct rendering of strings.
impl Renderable for String {
    fn render(&self, _console: &Console, _options: &ConsoleOptions) -> RenderResult {
        use crate::Style;
        Ok(vec![Segment::new(self.clone(), Style::default())])
    }
}

/// Implement `Renderable` for `&str` to enable direct rendering of string slices.
impl Renderable for &str {
    fn render(&self, _console: &Console, _options: &ConsoleOptions) -> RenderResult {
        use crate::Style;
        Ok(vec![Segment::new((*self).to_string(), Style::default())])
    }
}

/// Implement `Measurable` for `String` using Unicode width calculation.
impl Measurable for String {
    fn measure(&self, _console: &Console, _options: &ConsoleOptions) -> Result<Measurement> {
        use unicode_width::UnicodeWidthStr;
        let width = self.width();
        Ok(Measurement::new(width, width))
    }
}

/// Implement `Measurable` for `&str` using Unicode width calculation.
impl Measurable for &str {
    fn measure(&self, _console: &Console, _options: &ConsoleOptions) -> Result<Measurement> {
        use unicode_width::UnicodeWidthStr;
        let width = self.width();
        Ok(Measurement::new(width, width))
    }
}

/// Implement `Renderable` for `Box<dyn Renderable>` to enable trait object rendering.
impl Renderable for Box<dyn Renderable> {
    fn render(&self, console: &Console, options: &ConsoleOptions) -> RenderResult {
        (**self).render(console, options)
    }
}

/// Implement `Measurable` for `Box<dyn Measurable>` to enable trait object measurement.
impl Measurable for Box<dyn Measurable> {
    fn measure(&self, console: &Console, options: &ConsoleOptions) -> Result<Measurement> {
        (**self).measure(console, options)
    }
}

/// A convenience trait for objects that are both renderable and measurable.
///
/// This trait is automatically implemented for any type that implements both
/// `Renderable` and `Measurable`.
pub trait RenderableMeasurable: Renderable + Measurable {}

impl<T> RenderableMeasurable for T where T: Renderable + Measurable {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Console, ConsoleOptions};

    #[test]
    fn test_string_renderable() {
        let console = Console::new();
        let options = ConsoleOptions::default();
        let text = "Hello, world!".to_string();

        let result = text.render(&console, &options);
        assert!(result.is_ok());
        let segments = result.unwrap();
        assert_eq!(segments.len(), 1);
        assert_eq!(segments[0].text(), "Hello, world!");
    }

    #[test]
    fn test_str_renderable() {
        let console = Console::new();
        let options = ConsoleOptions::default();
        let text = "Hello, world!";

        let result = text.render(&console, &options);
        assert!(result.is_ok());
        let segments = result.unwrap();
        assert_eq!(segments.len(), 1);
        assert_eq!(segments[0].text(), "Hello, world!");
    }

    #[test]
    fn test_string_measurable() {
        let console = Console::new();
        let options = ConsoleOptions::default();
        let text = "Hello".to_string();

        let result = text.measure(&console, &options);
        assert!(result.is_ok());
        let measurement = result.unwrap();
        assert_eq!(measurement.minimum(), 5);
        assert_eq!(measurement.maximum(), 5);
    }

    #[test]
    fn test_str_measurable() {
        let console = Console::new();
        let options = ConsoleOptions::default();
        let text = "Hello";

        let result = text.measure(&console, &options);
        assert!(result.is_ok());
        let measurement = result.unwrap();
        assert_eq!(measurement.minimum(), 5);
        assert_eq!(measurement.maximum(), 5);
    }
}
