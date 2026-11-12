use crate::vga::colors;

pub const BUFFER_HEIGHT: usize = 25;
pub const BUFFER_WIDTH: usize = 80;

#[derive(Copy, Clone)]
pub struct ScreenChar {
  pub character: u8,
  pub color: colors::CharColor,
}

pub struct Buffer {
  pub chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}