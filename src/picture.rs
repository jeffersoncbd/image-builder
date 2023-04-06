use image::imageops::FilterType;

/// External images.
///
/// The Picture structure is responsible for defining and adjusting external images that will be
/// included in the image being built. It allows for cropping, resizing, and positioning to compose
/// the final image.
///
/// **⚠️ It's important to note that if there is both cropping and resizing on the same
/// image, the library will always perform cropping first, and then resizing.** ⚠️
/// ## Example
/// ```
/// use image_builder::Picture;
///
/// Picture::new("/home/user/logo.png")
///     .resize(100, 100, FilterType::Triangle) // Resizing is specified here, but the library will first perform the cropping below, and then this resizing.
///     .crop(50, 50, 200, 200);
/// ```
/// In the example above, an image of 300x300 pixels was imported, a square of 200x200 pixels was
/// cropped, and then this cropped portion was resized by half, resulting in an image of 100x100 pixels.
#[derive(Clone)]
pub struct Picture {
    path: String,
    crop: Option<(u32, u32, u32, u32)>,
    resize: Option<(u32, u32, FilterType)>,
    position: (u32, u32),
}
impl Picture {
    /// This method instantiates an external image using the file path of the image
    /// and positions it at the point (0,0) of the image being built.
    /// ## Example
    /// ```
    /// use image_builder::Picture;
    ///
    /// Picture::new("/home/user/logo.png");
    /// ```
    pub fn new(path: &str) -> Picture {
        let path = String::from(path);
        Picture {
            path,
            resize: None,
            crop: None,
            position: (0, 0),
        }
    }

    /// This method allows resizing an image by specifying the desired new height, width and [`FilterType`].
    /// ## Example
    /// ```
    /// use image_builder::Picture;
    ///
    /// Picture::new("/home/user/logo.png")
    ///     .resize(200, 100, FilterType::Triangle);
    /// ```
    pub fn resize(&mut self, width: u32, height: u32, filter: FilterType) -> Self {
        self.resize = Some((width, height, filter));
        self.clone()
    }

    /// Use this method to crop an imported image by providing the starting point of the crop (x, y),
    /// as well as the desired height and width to be cropped.
    /// ## Example
    /// ```
    /// use image_builder::Picture;
    ///
    /// Picture::new("/home/user/logo.png")
    ///     .crop(50, 50, 200, 200);
    /// ```
    pub fn crop(&mut self, x: u32, y: u32, width: u32, height: u32) -> Self {
        self.crop = Some((x, y, width, height));
        self.clone()
    }

    /// This method allows you to adjust the position of the imported image within the image being constructed.
    /// ## Example
    /// ```
    /// use image_builder::Picture;
    ///
    /// Picture::new("/home/user/logo.png")
    ///     .position(100, 100);
    /// ```
    pub fn position(&mut self, x: u32, y: u32) -> Self {
        self.position = (x, y);
        self.clone()
    }
}

pub struct CropValues {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}
pub struct ResizeValues {
    pub nwidth: u32,
    pub nheight: u32,
    pub filter: FilterType,
}
pub struct PictureValues<'a> {
    pub path: &'a String,
    pub x: i64,
    pub y: i64,
    pub crop: Option<CropValues>,
    pub resize: Option<ResizeValues>,
}
pub fn extract(picture: &Picture) -> PictureValues {
    PictureValues {
        path: &picture.path,
        x: picture.position.0 as i64,
        y: picture.position.1 as i64,
        crop: match picture.crop {
            None => None,
            Some(values) => Some(CropValues {
                x: values.0,
                y: values.1,
                width: values.2,
                height: values.3,
            }),
        },
        resize: match picture.resize {
            None => None,
            Some(values) => Some(ResizeValues {
                nwidth: values.0,
                nheight: values.1,
                filter: values.2,
            }),
        },
    }
}
