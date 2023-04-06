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

pub struct Image<'a> {
    background: Color,
    size: (u32, u32),
    fonts: HashMap<&'a str, Font<'a>>,
    elements: Vec<Element>,
}

impl<'a> Image<'a> {
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

    pub fn add_custom_font(&mut self, name: &'a str, font: Vec<u8>) {
        let font = Font::try_from_vec(font).expect(&format!("Fail to load the font \"{}\"", name));
        self.fonts.insert(name, font);
    }

    pub fn add_picture(&mut self, picture: Picture) {
        self.elements.push(Element::Picture(picture));
    }

    pub fn add_text(&mut self, text: Text) {
        self.elements.push(Element::Text(text));
    }
    pub fn add_rect(&mut self, rect: Rect) {
        self.elements.push(Element::Rect(rect));
    }

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
                    draw_text_mut(&mut image, t.color, t.x, t.y, t.scale, font, t.content);
                }
                Element::Rect(element) => {
                    let r = rect::extract(element);

                    draw_filled_rect_mut(
                        &mut image,
                        procRect::Rect::at(r.x, r.y).of_size(r.width, r.height),
                        r.color,
                    )
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
