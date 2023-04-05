use image::imageops::FilterType;

#[derive(Clone)]
pub struct Picture {
    path: String,
    crop: Option<(u32, u32, u32, u32)>,
    resize: Option<(u32, u32, FilterType)>,
    position: (u32, u32),
}
impl Picture {
    pub fn new(path: &str) -> Picture {
        let path = String::from(path);
        Picture {
            path,
            resize: None,
            crop: None,
            position: (0, 0),
        }
    }

    pub fn resize(&mut self, width: u32, height: u32, filter: FilterType) -> Self {
        self.resize = Some((width, height, filter));
        self.clone()
    }

    pub fn crop(&mut self, x: u32, y: u32, width: u32, height: u32) -> Self {
        self.crop = Some((x, y, width, height));
        self.clone()
    }

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
