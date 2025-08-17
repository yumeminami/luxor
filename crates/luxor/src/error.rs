//! Error types and handling for Luxor.

use thiserror::Error;

/// The main error type for Luxor operations.
#[derive(Error, Debug)]
pub enum LuxorError {
    /// Unicode-related errors, such as invalid character width calculations.
    #[error("Unicode error: {message}")]
    Unicode { message: String },

    /// Input/Output errors when writing to streams.
    #[error("IO error: {source}")]
    Io {
        #[from]
        source: std::io::Error,
    },

    /// Rendering errors that occur during the rendering process.
    #[error("Rendering error: {message}")]
    Rendering { message: String },

    /// Style parsing or composition errors.
    #[error("Style error: {message}")]
    Style { message: String },

    /// Color parsing or conversion errors.
    #[error("Color error: {message}")]
    Color { message: String },

    /// Measurement calculation errors.
    #[error("Measurement error: {message}")]
    Measurement { message: String },

    /// Terminal capability detection errors.
    #[error("Terminal error: {message}")]
    Terminal { message: String },

    /// Markup parsing errors.
    #[error("Markup error: {0}")]
    MarkupError(String),

    /// Invalid range errors for text operations.
    #[error("Invalid range: {0}")]
    InvalidRange(String),
}

impl LuxorError {
    /// Create a new Unicode error.
    pub fn unicode(message: impl Into<String>) -> Self {
        Self::Unicode {
            message: message.into(),
        }
    }

    /// Create a new rendering error.
    pub fn rendering(message: impl Into<String>) -> Self {
        Self::Rendering {
            message: message.into(),
        }
    }

    /// Create a new style error.
    pub fn style(message: impl Into<String>) -> Self {
        Self::Style {
            message: message.into(),
        }
    }

    /// Create a new color error.
    pub fn color(message: impl Into<String>) -> Self {
        Self::Color {
            message: message.into(),
        }
    }

    /// Create a new measurement error.
    pub fn measurement(message: impl Into<String>) -> Self {
        Self::Measurement {
            message: message.into(),
        }
    }

    /// Create a new terminal error.
    pub fn terminal(message: impl Into<String>) -> Self {
        Self::Terminal {
            message: message.into(),
        }
    }
}

/// A convenient Result type for Luxor operations.
pub type Result<T> = std::result::Result<T, LuxorError>;
