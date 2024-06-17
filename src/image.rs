use std::{collections::HashMap, fs::File};

pub use image::imageops::FilterType;

use image::{
    codecs::png::PngEncoder,
    imageops::{crop, overlay, resize},
    open, ImageBuffer, ImageEncoder, Rgba,
};
use imageproc::{
    drawing::{draw_filled_rect_mut, draw_text_mut},
    rect as procRect,
};
use rusttype::Font;

use crate::{
    colors::Color,
    picture::{self, Picture},
    rect::{self, Rect},
    text::{self, Text},
};

pub enum Element {
    Text(Text),
    Rect(Rect),
    Picture(Picture),
}

/// This is the structure of the image that will be created.
///
/// > Use the `new` function to get started.
///
/// It is important to remember that the order in which elements are added to the image defines which
/// element goes on top of which. For example, adding text that starts at point 0,0 and then adding a
/// rectangle that also starts at the same point will cause the rectangle to cover the text. However,
/// by reversing the order and adding the rectangle first, it will be placed underneath the text. It
/// is essential to keep this order in mind when creating images with multiple elements to ensure that
/// the elements are in the desired order.
/// ## Examples
/// ```rust
/// # use image_builder::Image;
/// # use image_builder::Rect;
/// # use image_builder::Text;
/// # use image_builder::colors;
/// let mut image = Image::new(500, 500, colors::WHITE);
/// image.add_text(Text::new("Image Builder"));
/// image.add_rect(Rect::new().size(200, 200)); // This rectangle covers the text.
/// ```
///
/// ```rust
/// # use image_builder::Image;
/// # use image_builder::Rect;
/// # use image_builder::Text;
/// # use image_builder::colors;
/// let mut image = Image::new(500, 500, colors::WHITE);
/// image.add_rect(Rect::new().size(200, 200)); // This rectangle is in the background of the text.
/// image.add_text(Text::new("Image Builder"));
/// ```
pub struct Image<'a> {
    background: Color,
    size: (u32, u32),
    fonts: HashMap<&'a str, Font<'a>>,
    elements: Vec<Element>,
}

impl<'a> Image<'a> {
    /// This method creates a new instance of an image, setting the background color, and size in
    /// pixels, and allocating memory to add fonts and elements to be drawn.
    /// ## Example
    /// ```
    /// use image_builder::{colors, Image};
    ///
    /// let mut image = Image::new(400, 300, colors::GRAY);
    /// ```
    pub fn new(width: u32, height: u32, background: Color) -> Image<'a> {
        let default_font = Vec::from(include_bytes!("Roboto-Regular.ttf") as &[u8]);
        let default_font = Font::try_from_vec(default_font)
            .expect("Fail to load the default font \"Roboto-Regular.ttf\"");

        Image {
            background,
            size: (width, height),
            fonts: HashMap::from([("default", default_font)]),
            elements: Vec::new(),
        }
    }

    /// The add_custom_font method requires that a .ttf font file (not provided) be loaded using fs.read,
    /// and internally linked to the provided name in a HashMap. This will allow you to use this font in
    /// your text by simply passing the font name as a parameter. Trying to use a font that has not been
    /// loaded cause the application to panic. Additionally, providing an invalid Vec<u8> will also
    /// result in a panic.
    /// ## Example
    /// ```
    /// use image_builder::Image;
    /// use std::fs;
    /// use image_builder::colors;
    ///
    /// let mut image = Image::new(500, 500, colors::WHITE);
    /// let roboto_bold = fs::read("fonts/Roboto/Roboto-Bold.ttf").unwrap();
    /// image.add_custom_font("Roboto bold", roboto_bold);
    /// ```
    pub fn add_custom_font(&mut self, name: &'a str, font: Vec<u8>) {
        let font = Font::try_from_vec(font).expect(&format!("Fail to load the font \"{}\"", name));
        self.fonts.insert(name, font);
    }

    /// With this method, it is possible to add an image on top of the image being built, taking into account
    /// transparent backgrounds. This means that transparent areas of the added image will not overlap areas
    /// already drawn in the main image. Please refer to the [`Picture`] for more details.
    pub fn add_picture(&mut self, picture: Picture) {
        self.elements.push(Element::Picture(picture));
    }

    /// This method allows for adding formatted text to the image being built. Refer to the [`Text`] for more details.
    pub fn add_text(&mut self, text: Text) {
        self.elements.push(Element::Text(text));
    }

    /// This method allows for adding rectangular shapes to the image being built. Refer to the [`Rect`] for more details.
    pub fn add_rect(&mut self, rect: Rect) {
        self.elements.push(Element::Rect(rect));
    }

    /// The save method is responsible for the entire rendering process of the library. It creates the image buffer and
    /// renders the list of elements added in the order they were inserted by the user. Then, it creates the image file,
    /// adds the generated buffer, and encodes the content to save it to the disk.
    pub fn save(&mut self, file_name: &str) {
        let mut image = ImageBuffer::from_pixel(self.size.0, self.size.1, Rgba(self.background));

        for element in self.elements.iter() {
            match element {
                Element::Picture(element) => {
                    let p = picture::extract(element);
                    let mut pic = open(p.path)
                        .expect(&format!("Unable to load the picture \"{}\"", p.path))
                        .to_rgba8();

                    if let Some(values) = p.crop {
                        pic = crop(&mut pic, values.x, values.y, values.width, values.height)
                            .to_image();
                    }
                    if let Some(values) = p.resize {
                        pic = resize(&mut pic, values.nwidth, values.nheight, values.filter)
                    }

                    overlay(&mut image, &pic, p.x, p.y);
                }
                Element::Text(element) => {
                    let t = text::extract(&element);
                    let font = self.fonts.get(t.font_name).expect(&format!("Unable to load the \"{}\" font, please verify that the name is correct or that it was loaded using the \"add_custom_font\" method.", t.font_name));
                    let mut text_image =
                        ImageBuffer::from_pixel(self.size.0, self.size.1, Rgba([0, 0, 0, 0]));
                    draw_text_mut(&mut text_image, t.color, 0, 0, t.scale, font, t.content);
                    overlay(&mut image, &text_image, t.x as i64, t.y as i64);
                }
                Element::Rect(element) => {
                    let r = rect::extract(element);
                    let mut rect_image =
                        ImageBuffer::from_pixel(r.width, r.height, Rgba([0, 0, 0, 0]));

                    draw_filled_rect_mut(
                        &mut rect_image,
                        procRect::Rect::at(0, 0).of_size(r.width, r.height),
                        r.color,
                    );

                    overlay(&mut image, &rect_image, r.x as i64, r.y as i64);
                }
            }
        }

        let file = File::create(file_name).expect(&format!(
            "It was not possible to create the file \"{}\" because the file path does not exist.",
            file_name
        ));
        let encoder = PngEncoder::new(file);
        encoder
            .write_image(
                &image,
                image.width(),
                image.height(),
                image::ColorType::Rgba8,
            )
            .unwrap();
    }
}
