use std::collections::HashMap;

const YELLOW: [u8; 4] = [255, 255, 0, 255];
const GREEN: [u8; 4] = [0, 176, 80, 255];

pub struct Colors<'a, 'b> {
  colors: HashMap<&'b str, &'a[u8; 4]>
}
impl<'a, 'b> Colors<'a, 'b> {
  pub fn new() -> Colors<'a, 'b> {
    Colors {
      colors: HashMap::from([
        ("yellow", &YELLOW),
        ("green", &GREEN),
      ])
    }
  }
  pub fn get(&self, name: &str) -> Option<&[u8; 4]> {
    self.colors.get(name).copied()
  }
}
