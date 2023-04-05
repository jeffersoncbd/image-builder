use image::Rgba;
use rusttype::Scale;

use crate::colors::{self, Color};

#[derive(Clone)]
pub struct Text {
    content: String,
    size: u32,
    position: (u32, u32),
    font_name: String,
    color: Color,
}
impl Text {
    pub fn new(content: &str) -> Text {
        let content = String::from(content);

        Text {
            content,
            size: 14,
            position: (0, 0),
            font_name: String::from("default"),
            color: colors::BLACK,
        }
    }

    pub fn size(&mut self, size: u32) -> Self {
        self.size = size;
        self.clone()
    }

    pub fn position(&mut self, x: u32, y: u32) -> Self {
        self.position = (x, y);
        self.clone()
    }

    pub fn font(&mut self, font_name: &str) -> Self {
        self.font_name = String::from(font_name);
        self.clone()
    }

    pub fn color(&mut self, color: Color) -> Self {
        self.color = color;
        self.clone()
    }
}

pub struct TextValues<'a> {
    pub color: Rgba<u8>,
    pub x: i32,
    pub y: i32,
    pub font_name: &'a str,
    pub scale: Scale,
    pub content: &'a str,
}

pub fn extract<'a>(text: &'a Text) -> TextValues<'a> {
    let scale = Scale {
        x: text.size as f32,
        y: text.size as f32,
    };
    TextValues {
        color: Rgba(text.color),
        x: text.position.0 as i32,
        y: text.position.1 as i32,
        scale,
        font_name: text.font_name.as_str(),
        content: &text.content,
    }
}
