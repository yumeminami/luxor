//! # Luxor
//!
//! A Rust implementation of the Python Rich library for rich text and beautiful formatting in the terminal.
//!
//! Luxor provides a rich text rendering engine with support for colors, styles, and complex layouts,
//! designed for high performance and memory safety.
//!
//! ## Quick Start
//!
//! ```rust
//! use luxor::{Console, Style, Color};
//!
//! let console = Console::new();
//! // Basic usage will be available after Phase 1 implementation
//! ```
//!
//! ## Architecture
//!
//! Luxor is built around several core concepts:
//!
//! - **Console**: The central rendering engine that manages output streams
//! - **Renderable**: A trait for objects that can be rendered to the console
//! - **Segment**: The fundamental rendering unit containing text, style, and control codes
//! - **Style**: Text styling information (color, bold, italic, etc.)
//! - **Color**: Color representation supporting standard, 8-bit, and 24-bit colors

pub mod ansi;
pub mod color;
pub mod console;
pub mod error;
pub mod markup;
pub mod measure;
pub mod protocol;
pub mod segment;
pub mod style;
pub mod text;

// Re-export core types for convenient access
pub use color::{Color, ColorSystem, StandardColor};
pub use console::{Console, ConsoleOptions, StyledText};
pub use error::{LuxorError, Result};
pub use markup::{Span, Tag, escape as escape_markup, render as render_markup};
pub use measure::Measurement;
pub use protocol::{Measurable, Renderable};
pub use segment::{ControlCode, Segment, Segments};
pub use style::Style;
pub use text::Text;
