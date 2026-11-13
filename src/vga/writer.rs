use lazy_static::lazy_static;
use spin::Mutex;
use crate::vga::colors;
use crate::vga::buffer;

const VGA_BUFFER_ADDR: usize = 0xb8000;

pub struct Writer {
  buffer: &'static mut buffer::Buffer,
  column: usize,
  color: colors::CharColor,
}

impl Writer {
  pub fn string(&mut self, string: &str) -> Result<(), ()> {
    for byte in string.bytes() {
      match byte {
        0x20..=0x7e | b'\n' => self.byte(byte),
        _ => self.byte(0xfe),
      }
    }

    Ok(())
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
        self.buffer.chars[row][col].write(buffer::ScreenChar {
          character: byte,
          color: self.color
        });

        self.column += 1;
      }
    }
  }

  fn newline(&mut self) {
    for row in 1..buffer::BUFFER_HEIGHT {
      for col in 0..buffer::BUFFER_WIDTH {
        let character = self.buffer.chars[row][col].read();
        self.buffer.chars[row - 1][col].write(character);
      }
    }

    let blank = buffer::ScreenChar { character: b' ', color: self.color };
    for column in 0..buffer::BUFFER_WIDTH {
      self.buffer.chars[buffer::BUFFER_HEIGHT - 1][column].write(blank);
    }
    self.column = 0;
  }
}

lazy_static! {
  pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
    column: 0,
    color: colors::CharColor::new(
      colors::VgaColor::LightGray,
      colors::VgaColor::Black
    ),
    buffer: unsafe { &mut *(VGA_BUFFER_ADDR as *mut buffer::Buffer) },
  });
}

pub fn println(string: &str) {
  let mut lock = WRITER.lock();
  lock.string(string).unwrap();
  lock.string("\n").unwrap();
}