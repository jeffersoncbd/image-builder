use image::{self, DynamicImage, ImageBuffer, Rgba};
use imageproc::{drawing::{draw_text_mut, draw_filled_rect}, rect::Rect};
use rusttype::{Font, Scale};

pub struct Position {
  pub x: i32,
  pub y: i32,
}
pub struct Text<'a> {
  content: &'a str,
  size: u32,
  position: Position,
}
impl<'a> Text<'a> {
  pub fn new(content: &'a str, size: u32, position: Position) -> Text<'a> {
    Text { content, size, position }
  }
}

pub struct Image<'a> {
  image: ImageBuffer<Rgba<u8>, Vec<u8>>,
  font: Font<'a>,
}
impl<'a> Image<'a> {
  pub fn new(width: u32, height: u32) -> Image<'a> {
    let image = DynamicImage::new_rgb16(width, height);
    let image = draw_filled_rect(
      &image,
      Rect::at(0, 0).of_size(width, height),
      image::Rgba([255u8, 255u8, 255u8, 255u8])
    );
    let font = Vec::from(include_bytes!("Roboto-Regular.ttf") as &[u8]);
    let font = Font::try_from_vec(font).unwrap();

    Image { image, font }
  }

  pub fn print_text(&mut self, text: Text<'a>) {
    let scale = Scale { x: text.size as f32, y: text.size as f32 };
    draw_text_mut(
      &mut self.image,
      image::Rgba([0u8, 0u8, 0u8, 0u8]),
      text.position.x, text.position.y, scale, &self.font,
      text.content
    );
  }

  pub fn save(&self, path: &str) {
    self.image.save(path).unwrap();
  }
}
