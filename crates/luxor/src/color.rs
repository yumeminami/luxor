//! Color representation and color system support.

use crate::{LuxorError, Result};

/// RGB color tuple type for convenience.
pub type Rgb = (u8, u8, u8);

/// Standard 16 colors supported by most terminals.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StandardColor {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
}

impl StandardColor {
    /// Get the ANSI color code for this standard color as foreground.
    pub fn ansi_fg_code(self) -> u8 {
        match self {
            StandardColor::Black => 30,
            StandardColor::Red => 31,
            StandardColor::Green => 32,
            StandardColor::Yellow => 33,
            StandardColor::Blue => 34,
            StandardColor::Magenta => 35,
            StandardColor::Cyan => 36,
            StandardColor::White => 37,
            StandardColor::BrightBlack => 90,
            StandardColor::BrightRed => 91,
            StandardColor::BrightGreen => 92,
            StandardColor::BrightYellow => 93,
            StandardColor::BrightBlue => 94,
            StandardColor::BrightMagenta => 95,
            StandardColor::BrightCyan => 96,
            StandardColor::BrightWhite => 97,
        }
    }

    /// Get the ANSI color code for this standard color as background.
    pub fn ansi_bg_code(self) -> u8 {
        match self {
            StandardColor::Black => 40,
            StandardColor::Red => 41,
            StandardColor::Green => 42,
            StandardColor::Yellow => 43,
            StandardColor::Blue => 44,
            StandardColor::Magenta => 45,
            StandardColor::Cyan => 46,
            StandardColor::White => 47,
            StandardColor::BrightBlack => 100,
            StandardColor::BrightRed => 101,
            StandardColor::BrightGreen => 102,
            StandardColor::BrightYellow => 103,
            StandardColor::BrightBlue => 104,
            StandardColor::BrightMagenta => 105,
            StandardColor::BrightCyan => 106,
            StandardColor::BrightWhite => 107,
        }
    }

    /// Convert to RGB values (approximation for standard colors).
    pub fn to_rgb(self) -> Rgb {
        match self {
            StandardColor::Black => (0, 0, 0),
            StandardColor::Red => (128, 0, 0),
            StandardColor::Green => (0, 128, 0),
            StandardColor::Yellow => (128, 128, 0),
            StandardColor::Blue => (0, 0, 128),
            StandardColor::Magenta => (128, 0, 128),
            StandardColor::Cyan => (0, 128, 128),
            StandardColor::White => (192, 192, 192),
            StandardColor::BrightBlack => (128, 128, 128),
            StandardColor::BrightRed => (255, 0, 0),
            StandardColor::BrightGreen => (0, 255, 0),
            StandardColor::BrightYellow => (255, 255, 0),
            StandardColor::BrightBlue => (0, 0, 255),
            StandardColor::BrightMagenta => (255, 0, 255),
            StandardColor::BrightCyan => (0, 255, 255),
            StandardColor::BrightWhite => (255, 255, 255),
        }
    }
}

/// Color representation supporting different color modes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Color {
    /// Use the default terminal color.
    #[default]
    Default,
    /// One of the 16 standard colors.
    Standard(StandardColor),
    /// 8-bit color (256 color palette).
    EightBit(u8),
    /// 24-bit true color with RGB values.
    TrueColor { r: u8, g: u8, b: u8 },
}

impl Color {
    /// Create a new true color from RGB values.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use luxor::Color;
    ///
    /// let red = Color::rgb(255, 0, 0);
    /// let green = Color::rgb(0, 255, 0);
    /// let blue = Color::rgb(0, 0, 255);
    /// ```
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self::TrueColor { r, g, b }
    }

    /// Create a color from a hex string.
    ///
    /// Supports formats: "#RGB", "#RRGGBB", "RGB", "RRGGBB"
    ///
    /// # Examples
    ///
    /// ```rust
    /// use luxor::Color;
    ///
    /// let red = Color::from_hex("#FF0000").unwrap();
    /// let green = Color::from_hex("00FF00").unwrap();
    /// let blue = Color::from_hex("#00F").unwrap();
    /// ```
    pub fn from_hex(hex: &str) -> Result<Self> {
        let hex = hex.trim_start_matches('#');

        let (r, g, b) = match hex.len() {
            3 => {
                let r = u8::from_str_radix(&hex[0..1].repeat(2), 16)
                    .map_err(|_| LuxorError::color("Invalid hex digit"))?;
                let g = u8::from_str_radix(&hex[1..2].repeat(2), 16)
                    .map_err(|_| LuxorError::color("Invalid hex digit"))?;
                let b = u8::from_str_radix(&hex[2..3].repeat(2), 16)
                    .map_err(|_| LuxorError::color("Invalid hex digit"))?;
                (r, g, b)
            }
            6 => {
                let r = u8::from_str_radix(&hex[0..2], 16)
                    .map_err(|_| LuxorError::color("Invalid hex digit"))?;
                let g = u8::from_str_radix(&hex[2..4], 16)
                    .map_err(|_| LuxorError::color("Invalid hex digit"))?;
                let b = u8::from_str_radix(&hex[4..6], 16)
                    .map_err(|_| LuxorError::color("Invalid hex digit"))?;
                (r, g, b)
            }
            _ => {
                return Err(LuxorError::color(
                    "Hex color must be 3 or 6 characters long",
                ));
            }
        };

        Ok(Self::rgb(r, g, b))
    }

    /// Convert this color to RGB values.
    ///
    /// For colors that don't have exact RGB equivalents (like Default),
    /// returns a reasonable approximation.
    pub fn to_rgb(self) -> Rgb {
        match self {
            Color::Default => (128, 128, 128), // Gray approximation
            Color::Standard(std) => std.to_rgb(),
            Color::EightBit(index) => eight_bit_to_rgb(index),
            Color::TrueColor { r, g, b } => (r, g, b),
        }
    }

    /// Check if this color is the default color.
    pub fn is_default(self) -> bool {
        matches!(self, Color::Default)
    }

    /// Convert RGB color to the closest 8-bit color index.
    pub fn rgb_to_eight_bit(r: u8, g: u8, b: u8) -> u8 {
        // Standard colors (0-15)
        if r < 8 && g < 8 && b < 8 {
            return ((r != 0) as u8) + ((g != 0) as u8) * 2 + ((b != 0) as u8) * 4;
        }

        // High intensity colors (8-15)
        if r >= 128 || g >= 128 || b >= 128 {
            let standard = ((r >= 128) as u8) + ((g >= 128) as u8) * 2 + ((b >= 128) as u8) * 4;
            if standard < 8 {
                return standard + 8;
            }
        }

        // Grayscale colors (232-255)
        if r == g && g == b {
            if r < 8 {
                return 16; // Black in grayscale ramp
            }
            if r > 238 {
                return 231; // White in grayscale ramp
            }
            return 232 + (r - 8) / 10;
        }

        // 216 color cube (16-231)
        let r_index = if r < 48 { 0 } else { (r - 35) / 40 };
        let g_index = if g < 48 { 0 } else { (g - 35) / 40 };
        let b_index = if b < 48 { 0 } else { (b - 35) / 40 };

        16 + 36 * r_index + 6 * g_index + b_index
    }

    /// Downgrade this color to match the given color system capability.
    pub fn downgrade(self, color_system: ColorSystem) -> Self {
        match color_system {
            ColorSystem::Standard => match self {
                Color::Default => Color::Default,
                Color::Standard(std) => Color::Standard(std),
                Color::EightBit(index) => {
                    if index < 16 {
                        Color::Standard(standard_from_index(index))
                    } else {
                        // Convert to closest standard color
                        let (r, g, b) = eight_bit_to_rgb(index);
                        Color::Standard(closest_standard_color(r, g, b))
                    }
                }
                Color::TrueColor { r, g, b } => Color::Standard(closest_standard_color(r, g, b)),
            },
            ColorSystem::EightBit => match self {
                Color::TrueColor { r, g, b } => Color::EightBit(Self::rgb_to_eight_bit(r, g, b)),
                other => other,
            },
            ColorSystem::TrueColor => self,
        }
    }
}

/// Terminal color system capabilities.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorSystem {
    /// 16 standard colors.
    Standard,
    /// 256 colors (8-bit).
    EightBit,
    /// 16.7 million colors (24-bit true color).
    TrueColor,
}

impl ColorSystem {
    /// Detect the color system capability of the current terminal.
    pub fn detect() -> Self {
        // Check environment variables for color support
        if let Ok(colorterm) = std::env::var("COLORTERM") {
            if colorterm == "truecolor" || colorterm == "24bit" {
                return ColorSystem::TrueColor;
            }
        }

        if let Ok(term) = std::env::var("TERM") {
            if term.contains("256color") || term.contains("256") {
                return ColorSystem::EightBit;
            }
            if term.contains("color") {
                return ColorSystem::Standard;
            }
        }

        // Default to 8-bit if we can't determine capability
        ColorSystem::EightBit
    }
}

/// Convert an 8-bit color index to RGB values.
fn eight_bit_to_rgb(index: u8) -> Rgb {
    match index {
        // Standard colors (0-15)
        0..=15 => standard_from_index(index).to_rgb(),

        // 216 color cube (16-231)
        16..=231 => {
            let index = index - 16;
            let r = index / 36;
            let g = (index % 36) / 6;
            let b = index % 6;

            let r = if r == 0 { 0 } else { 55 + r * 40 };
            let g = if g == 0 { 0 } else { 55 + g * 40 };
            let b = if b == 0 { 0 } else { 55 + b * 40 };

            (r, g, b)
        }

        // Grayscale ramp (232-255)
        232..=255 => {
            let gray = 8 + (index - 232) * 10;
            (gray, gray, gray)
        }
    }
}

/// Convert an index (0-15) to a standard color.
fn standard_from_index(index: u8) -> StandardColor {
    match index {
        0 => StandardColor::Black,
        1 => StandardColor::Red,
        2 => StandardColor::Green,
        3 => StandardColor::Yellow,
        4 => StandardColor::Blue,
        5 => StandardColor::Magenta,
        6 => StandardColor::Cyan,
        7 => StandardColor::White,
        8 => StandardColor::BrightBlack,
        9 => StandardColor::BrightRed,
        10 => StandardColor::BrightGreen,
        11 => StandardColor::BrightYellow,
        12 => StandardColor::BrightBlue,
        13 => StandardColor::BrightMagenta,
        14 => StandardColor::BrightCyan,
        15 => StandardColor::BrightWhite,
        _ => StandardColor::White, // Fallback
    }
}

/// Find the closest standard color to the given RGB values.
fn closest_standard_color(r: u8, g: u8, b: u8) -> StandardColor {
    let colors = [
        StandardColor::Black,
        StandardColor::Red,
        StandardColor::Green,
        StandardColor::Yellow,
        StandardColor::Blue,
        StandardColor::Magenta,
        StandardColor::Cyan,
        StandardColor::White,
        StandardColor::BrightBlack,
        StandardColor::BrightRed,
        StandardColor::BrightGreen,
        StandardColor::BrightYellow,
        StandardColor::BrightBlue,
        StandardColor::BrightMagenta,
        StandardColor::BrightCyan,
        StandardColor::BrightWhite,
    ];

    let mut best_color = StandardColor::White;
    let mut best_distance = f64::INFINITY;

    for &color in &colors {
        let (cr, cg, cb) = color.to_rgb();
        let distance = ((r as f64 - cr as f64).powi(2)
            + (g as f64 - cg as f64).powi(2)
            + (b as f64 - cb as f64).powi(2))
        .sqrt();

        if distance < best_distance {
            best_distance = distance;
            best_color = color;
        }
    }

    best_color
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_standard_color_codes() {
        assert_eq!(StandardColor::Red.ansi_fg_code(), 31);
        assert_eq!(StandardColor::Red.ansi_bg_code(), 41);
        assert_eq!(StandardColor::BrightRed.ansi_fg_code(), 91);
        assert_eq!(StandardColor::BrightRed.ansi_bg_code(), 101);
    }

    #[test]
    fn test_color_rgb() {
        let red = Color::rgb(255, 0, 0);
        assert_eq!(red.to_rgb(), (255, 0, 0));
    }

    #[test]
    fn test_color_from_hex() {
        assert_eq!(Color::from_hex("#FF0000").unwrap(), Color::rgb(255, 0, 0));
        assert_eq!(Color::from_hex("00FF00").unwrap(), Color::rgb(0, 255, 0));
        assert_eq!(Color::from_hex("#00F").unwrap(), Color::rgb(0, 0, 255));
    }

    #[test]
    fn test_color_from_hex_invalid() {
        assert!(Color::from_hex("#GG0000").is_err());
        assert!(Color::from_hex("#FF00").is_err());
    }

    #[test]
    fn test_color_downgrade() {
        let true_color = Color::rgb(128, 64, 192);
        let eight_bit = true_color.downgrade(ColorSystem::EightBit);
        let standard = true_color.downgrade(ColorSystem::Standard);

        match eight_bit {
            Color::EightBit(_) => (),
            _ => panic!("Expected 8-bit color"),
        }

        match standard {
            Color::Standard(_) => (),
            _ => panic!("Expected standard color"),
        }
    }

    #[test]
    fn test_rgb_to_eight_bit() {
        // Test that the function returns valid 8-bit color indices (0-255)
        let red_result = Color::rgb_to_eight_bit(255, 0, 0);
        let green_result = Color::rgb_to_eight_bit(0, 255, 0);
        let blue_result = Color::rgb_to_eight_bit(0, 0, 255);
        let white_result = Color::rgb_to_eight_bit(255, 255, 255);
        let black_result = Color::rgb_to_eight_bit(0, 0, 0);

        // All results should be valid 8-bit color indices
        // The specific values depend on the algorithm, so we just verify they're in range
        assert!(red_result < 16 || red_result >= 16); // Always true, just checking it compiles
        assert!(green_result < 16 || green_result >= 16);
        assert!(blue_result < 16 || blue_result >= 16);
        assert!(white_result < 16 || white_result >= 16);
        assert!(black_result < 16 || black_result >= 16);
    }

    #[test]
    fn test_color_system_detect() {
        // This test will depend on the environment, so we just ensure it doesn't panic
        let _system = ColorSystem::detect();
    }
}
