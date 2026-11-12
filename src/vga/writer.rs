use crate::vga::colors;
use crate::vga::buffer;

const VGA_BUFFER_ADDR: i32 = 0xb8000;

pub struct Writer {
  buffer: &'static mut buffer::Buffer,
  column: usize,
  color: colors::CharColor,
}

impl Writer {
  pub fn new() -> Writer {
    Writer {
      column: 0,
      color: colors::CharColor::new(
        colors::VgaColor::LightGray,
        colors::VgaColor::Black
      ),
      buffer: unsafe { &mut *(VGA_BUFFER_ADDR as *mut buffer::Buffer) },
    }
  }

  pub fn string(&mut self, string: &str) {
    for byte in string.bytes() {
      match byte {
        0x20..=0x7e | b'\n' => self.byte(byte),
        _ => self.byte(0xfe),
      }
    }
  }

  pub fn byte(&mut self, byte: u8) {
    match byte {
      b'\n' => self.newline(),
      byte => {
        if self.column >= buffer::BUFFER_WIDTH {
          self.newline();
        }

        let row = buffer::BUFFER_HEIGHT - 1;
        let col = self.column;
        self.buffer.chars[row][col] = buffer::ScreenChar {
          character: byte,
          color: self.color
        };

        self.column += 1;
      }
    }
  }

  fn newline(&mut self) { // FIXME: This is still WIP
    for row in 0..(buffer::BUFFER_HEIGHT - 1) {
      self.buffer.chars[row] = self.buffer.chars[row + 1];
    }
  }
}