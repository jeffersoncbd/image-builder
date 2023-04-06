/// The library provides a variety of basic colors for quick use but imposes no limits on the
/// choice of colors used. Any RGBA color can be used as long as it follows this structure.
pub type Color = [u8; 4];

pub const YELLOW: Color = [255, 255, 0, 255];
pub const GREEN: Color = [0, 176, 80, 255];
pub const GRAY: Color = [191, 191, 191, 255];
pub const RED: Color = [255, 0, 0, 255];
pub const BLUE: Color = [0, 112, 192, 255];
pub const ORANGE: Color = [255, 153, 0, 255];
pub const PURPLE: Color = [112, 48, 160, 255];
pub const WHITE: Color = [255, 255, 255, 255];
pub const BLACK: Color = [0, 0, 0, 255];
