#[allow(dead_code)]
#[repr(u8)]
pub enum VgaColor {
  Black = 0x0,
  Blue = 0x1,
  Green = 0x2,
  Cyan = 0x3,
  Red = 0x4,
  Magenta = 0x5,
  Brown = 0x6,
  LightGray = 0x7,
  DarkGray = 0x8,
  LightBlue = 0x9,
  LightGreen = 0xA,
  LightCyan = 0xB,
  LightRed = 0xC,
  Pink = 0xD,
  Yellow = 0xE,
  White = 0xF,
}

#[derive(Clone, Copy)]
pub struct CharColor(u8);

impl CharColor {
  pub fn new(fg: VgaColor, bg: VgaColor) -> CharColor {
    CharColor((bg as u8) << 4 | (fg as u8))
  }
}