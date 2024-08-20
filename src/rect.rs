use image::Rgba;

use crate::colors::{self, Color};

/// Specifications of a rectangular shape.
#[derive(Clone)]
pub struct Rect {
    position: (u32, u32),
    size: (u32, u32),
    color: Color,
}
impl Rect {
    /// This method instantiates a specifications of a rectangular shape.
    /// ## Example
    /// ```
    /// use image_builder::Rect;
    ///
    /// Rect::new();
    /// ```
    pub fn new() -> Rect {
        Rect {
            position: (0, 0),
            size: (10, 10),
            color: colors::GREEN,
        }
    }

    /// This method allows you to adjust the position of the rect within the image being constructed.
    /// ## Example
    /// ```
    /// use image_builder::Rect;
    ///
    /// Rect::new()
    ///     .position(100, 100);
    /// ```
    pub fn position(&mut self, x: u32, y: u32) -> Self {
        self.position = (x, y);
        self.clone()
    }

    /// Define the size of the rect.
    /// ## Example
    /// ```
    /// use image_builder::Rect;
    ///
    /// Rect::new()
    ///     .size(150, 50);
    /// ```
    pub fn size(&mut self, width: u32, height: u32) -> Self {
        self.size = (width, height);
        self.clone()
    }

    /// Define the color of the rect.
    /// ## Examples
    /// ```
    /// use image_builder::{Rect, colors};
    ///
    /// Rect::new()
    ///     .color(colors::RED);
    /// ```
    /// ```
    /// use image_builder::Rect;
    ///
    /// Rect::new()
    ///     .color([150, 30, 255, 150]); // rgba values
    /// ```
    pub fn color(&mut self, color: Color) -> Self {
        self.color = color;
        self.clone()
    }
}

#[derive(Clone)]
pub struct RectValues {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub color: Rgba<u8>,
}
pub fn extract(rect: &Rect) -> RectValues {
    RectValues {
        x: rect.position.0 as i32,
        y: rect.position.1 as i32,
        width: rect.size.0,
        height: rect.size.1,
        color: Rgba(rect.color),
    }
}
