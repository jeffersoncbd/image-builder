//! # Overview
//!
//! Image Builder is a high-level library that uses the [image](https://crates.io/crates/image)
//! crate as the engine to generate simple PNG images, but with convenience and simplicity.

mod image;
mod picture;
mod rect;
mod text;

pub use crate::image::FilterType;

/// Contain some basic colors for quick use, as well as the structure that other colors must
/// follow to be accepted by the library.
pub mod colors;

pub use crate::image::Image;
pub use picture::Picture;
pub use rect::Rect;
pub use text::Text;
