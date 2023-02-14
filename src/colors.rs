use std::collections::HashMap;

const YELLOW: [u8; 4] = [255, 255, 0, 255];
const GREEN: [u8; 4] = [0, 176, 80, 255];
const GRAY: [u8; 4] = [191, 191, 191, 255];
const RED: [u8; 4] = [255, 0, 0, 255];
const BLUE: [u8; 4] = [0, 112, 192, 255];
const ORANGE: [u8; 4] = [255, 153, 0, 255];
const PURPLE: [u8; 4] = [112, 48, 160, 255];
const WHITE: [u8; 4] = [255, 255, 255, 255];
const BLACK: [u8; 4] = [0, 0, 0, 255];

pub struct Colors<'a, 'b> {
    colors: HashMap<&'b str, &'a [u8; 4]>,
}
impl<'a, 'b> Colors<'a, 'b> {
    pub fn new() -> Colors<'a, 'b> {
        Colors {
            colors: HashMap::from([
                ("yellow", &YELLOW),
                ("green", &GREEN),
                ("gray", &GRAY),
                ("red", &RED),
                ("blue", &BLUE),
                ("orange", &ORANGE),
                ("purple", &PURPLE),
                ("white", &WHITE),
                ("black", &BLACK),
            ]),
        }
    }
    pub fn get(&self, name: &str) -> Option<&[u8; 4]> {
        self.colors.get(name).copied()
    }
}
