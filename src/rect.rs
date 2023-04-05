use image::Rgba;

use crate::colors::{self, Color};

#[derive(Clone)]
pub struct Rect {
    position: (u32, u32),
    size: (u32, u32),
    color: Color,
}
impl Rect {
    pub fn new() -> Rect {
        Rect {
            position: (0, 0),
            size: (10, 10),
            color: colors::GREEN,
        }
    }
    pub fn position(&mut self, x: u32, y: u32) -> Self {
        self.position = (x, y);
        self.clone()
    }
    pub fn size(&mut self, width: u32, height: u32) -> Self {
        self.size = (width, height);
        self.clone()
    }
    pub fn color(&mut self, color: Color) -> Self {
        self.color = color;
        self.clone()
    }
}

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
