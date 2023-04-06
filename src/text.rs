use image::Rgba;
use rusttype::Scale;

use crate::colors::{self, Color};

/// Content and formatting of a text.
#[derive(Clone)]
pub struct Text {
    content: String,
    size: u32,
    position: (u32, u32),
    font_name: String,
    color: Color,
}
impl Text {
    /// This method instantiates a specifications of a text.
    /// ## Example
    /// ```
    /// use image_builder::Text;
    ///
    /// Text::new("Any text here");
    /// ```
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

    /// Define the size of the text.
    /// ## Example
    /// ```
    /// use image_builder::Text;
    ///
    /// Text::new("Any text here")
    ///     .size(50);
    /// ```
    pub fn size(&mut self, size: u32) -> Self {
        self.size = size;
        self.clone()
    }

    /// This method allows you to adjust the position of the text within the image being constructed.
    /// ## Example
    /// ```
    /// use image_builder::Text;
    ///
    /// Text::new("Any text here")
    ///     .position(100, 100);
    /// ```
    pub fn position(&mut self, x: u32, y: u32) -> Self {
        self.position = (x, y);
        self.clone()
    }

    /// This method is used to set the font of a text, but it's important to remember to import the
    /// font using the add_custom_font method of the [`crate::Image`] structure (refer to the documentation
    /// for more details). Trying to use a font that hasn't been imported will result in an error
    /// in the application. Make sure to import the font correctly to avoid panics.
    /// ## Example
    /// ```
    /// use image_builder::Text;
    ///
    /// Text::new("Any text here")
    ///     .font("Any font");
    /// ```
    pub fn font(&mut self, font_name: &str) -> Self {
        self.font_name = String::from(font_name);
        self.clone()
    }

    /// Define the color of the text.
    /// ## Examples
    /// ```
    /// use image_builder::{Text, colors};
    ///
    /// Text::new("Any text here")
    ///     .color(colors::BLUE);
    /// ```
    /// ```
    /// use image_builder::Text;
    ///
    /// Text::new("Any text here")
    ///     .color([30, 90, 150, 255]); // rgba values
    /// ```
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
