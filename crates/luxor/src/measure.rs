//! Measurement utilities for layout calculations.

/// Represents the measurement requirements for a renderable object.
///
/// Measurements are used by the layout system to determine how much space
/// objects need and how they can be arranged within available space.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Measurement {
    /// The minimum width required by the object.
    minimum: usize,
    /// The maximum width the object can expand to.
    maximum: usize,
}

impl Measurement {
    /// Create a new measurement with the given minimum and maximum widths.
    ///
    /// # Panics
    ///
    /// Panics if `minimum > maximum`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use luxor::Measurement;
    ///
    /// let measurement = Measurement::new(10, 20);
    /// assert_eq!(measurement.minimum(), 10);
    /// assert_eq!(measurement.maximum(), 20);
    /// ```
    pub fn new(minimum: usize, maximum: usize) -> Self {
        assert!(
            minimum <= maximum,
            "Minimum width ({}) cannot be greater than maximum width ({})",
            minimum,
            maximum
        );
        Self { minimum, maximum }
    }

    /// Create a measurement for a fixed width.
    ///
    /// This is equivalent to `Measurement::new(width, width)`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use luxor::Measurement;
    ///
    /// let measurement = Measurement::fixed(15);
    /// assert_eq!(measurement.minimum(), 15);
    /// assert_eq!(measurement.maximum(), 15);
    /// ```
    pub fn fixed(width: usize) -> Self {
        Self::new(width, width)
    }

    /// Create a measurement that spans the available width.
    ///
    /// The minimum width is 0, and the maximum is the given width.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use luxor::Measurement;
    ///
    /// let measurement = Measurement::span(100);
    /// assert_eq!(measurement.minimum(), 0);
    /// assert_eq!(measurement.maximum(), 100);
    /// ```
    pub fn span(width: usize) -> Self {
        Self::new(0, width)
    }

    /// Get the minimum width.
    pub fn minimum(&self) -> usize {
        self.minimum
    }

    /// Get the maximum width.
    pub fn maximum(&self) -> usize {
        self.maximum
    }

    /// Check if this is a fixed width measurement.
    ///
    /// Returns `true` if the minimum and maximum widths are equal.
    pub fn is_fixed(&self) -> bool {
        self.minimum == self.maximum
    }

    /// Get the width if this is a fixed measurement.
    ///
    /// Returns `Some(width)` if `is_fixed()` is true, otherwise `None`.
    pub fn get_fixed(&self) -> Option<usize> {
        if self.is_fixed() {
            Some(self.minimum)
        } else {
            None
        }
    }

    /// Clamp the measurement to fit within the given constraints.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use luxor::Measurement;
    ///
    /// let measurement = Measurement::new(10, 50);
    /// let clamped = measurement.clamp(20, 40);
    /// assert_eq!(clamped.minimum(), 20);
    /// assert_eq!(clamped.maximum(), 40);
    /// ```
    pub fn clamp(self, min_width: usize, max_width: usize) -> Self {
        Self::new(
            self.minimum.max(min_width).min(max_width),
            self.maximum.max(min_width).min(max_width),
        )
    }

    /// Add a fixed width to both minimum and maximum.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use luxor::Measurement;
    ///
    /// let measurement = Measurement::new(10, 20);
    /// let expanded = measurement.add_width(5);
    /// assert_eq!(expanded.minimum(), 15);
    /// assert_eq!(expanded.maximum(), 25);
    /// ```
    pub fn add_width(self, width: usize) -> Self {
        Self::new(self.minimum + width, self.maximum + width)
    }

    /// Subtract a width from both minimum and maximum.
    ///
    /// The result is clamped to ensure minimum <= maximum and both >= 0.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use luxor::Measurement;
    ///
    /// let measurement = Measurement::new(10, 20);
    /// let reduced = measurement.subtract_width(3);
    /// assert_eq!(reduced.minimum(), 7);
    /// assert_eq!(reduced.maximum(), 17);
    /// ```
    pub fn subtract_width(self, width: usize) -> Self {
        Self::new(
            self.minimum.saturating_sub(width),
            self.maximum.saturating_sub(width),
        )
    }

    /// Combine two measurements by taking the maximum of both dimensions.
    ///
    /// This is useful for stacking elements vertically where the width
    /// requirement is the maximum of all elements.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use luxor::Measurement;
    ///
    /// let m1 = Measurement::new(10, 30);
    /// let m2 = Measurement::new(15, 25);
    /// let combined = m1.max_with(m2);
    /// assert_eq!(combined.minimum(), 15);
    /// assert_eq!(combined.maximum(), 30);
    /// ```
    pub fn max_with(self, other: Self) -> Self {
        Self::new(
            self.minimum.max(other.minimum),
            self.maximum.max(other.maximum),
        )
    }

    /// Combine two measurements by adding their widths.
    ///
    /// This is useful for placing elements side by side where the total
    /// width requirement is the sum of both elements.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use luxor::Measurement;
    ///
    /// let m1 = Measurement::new(10, 20);
    /// let m2 = Measurement::new(5, 15);
    /// let combined = m1.add_with(m2);
    /// assert_eq!(combined.minimum(), 15);
    /// assert_eq!(combined.maximum(), 35);
    /// ```
    pub fn add_with(self, other: Self) -> Self {
        Self::new(self.minimum + other.minimum, self.maximum + other.maximum)
    }
}

impl Default for Measurement {
    /// Create a measurement with zero width.
    fn default() -> Self {
        Self::fixed(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let measurement = Measurement::new(10, 20);
        assert_eq!(measurement.minimum(), 10);
        assert_eq!(measurement.maximum(), 20);
    }

    #[test]
    #[should_panic(expected = "Minimum width (20) cannot be greater than maximum width (10)")]
    fn test_new_invalid() {
        Measurement::new(20, 10);
    }

    #[test]
    fn test_fixed() {
        let measurement = Measurement::fixed(15);
        assert_eq!(measurement.minimum(), 15);
        assert_eq!(measurement.maximum(), 15);
        assert!(measurement.is_fixed());
        assert_eq!(measurement.get_fixed(), Some(15));
    }

    #[test]
    fn test_span() {
        let measurement = Measurement::span(100);
        assert_eq!(measurement.minimum(), 0);
        assert_eq!(measurement.maximum(), 100);
        assert!(!measurement.is_fixed());
        assert_eq!(measurement.get_fixed(), None);
    }

    #[test]
    fn test_clamp() {
        let measurement = Measurement::new(10, 50);
        let clamped = measurement.clamp(20, 40);
        assert_eq!(clamped.minimum(), 20);
        assert_eq!(clamped.maximum(), 40);
    }

    #[test]
    fn test_add_width() {
        let measurement = Measurement::new(10, 20);
        let expanded = measurement.add_width(5);
        assert_eq!(expanded.minimum(), 15);
        assert_eq!(expanded.maximum(), 25);
    }

    #[test]
    fn test_subtract_width() {
        let measurement = Measurement::new(10, 20);
        let reduced = measurement.subtract_width(3);
        assert_eq!(reduced.minimum(), 7);
        assert_eq!(reduced.maximum(), 17);
    }

    #[test]
    fn test_subtract_width_saturating() {
        let measurement = Measurement::new(5, 10);
        let reduced = measurement.subtract_width(8);
        assert_eq!(reduced.minimum(), 0);
        assert_eq!(reduced.maximum(), 2);
    }

    #[test]
    fn test_max_with() {
        let m1 = Measurement::new(10, 30);
        let m2 = Measurement::new(15, 25);
        let combined = m1.max_with(m2);
        assert_eq!(combined.minimum(), 15);
        assert_eq!(combined.maximum(), 30);
    }

    #[test]
    fn test_add_with() {
        let m1 = Measurement::new(10, 20);
        let m2 = Measurement::new(5, 15);
        let combined = m1.add_with(m2);
        assert_eq!(combined.minimum(), 15);
        assert_eq!(combined.maximum(), 35);
    }

    #[test]
    fn test_default() {
        let measurement = Measurement::default();
        assert_eq!(measurement.minimum(), 0);
        assert_eq!(measurement.maximum(), 0);
        assert!(measurement.is_fixed());
    }
}
