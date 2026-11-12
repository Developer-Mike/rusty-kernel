#![no_std] // Use no std lib
#![no_main] // Nothing can call main in our kernel

use bootloader::{entry_point, BootInfo};

mod panic;
mod vga;

entry_point!(kernel_main);

fn kernel_main(_boot_info: &'static BootInfo) -> ! {
  let mut writer = vga::writer::Writer::new();

  writer.string("Hello world\n");
  writer.string("Hello world\n");

  loop {}
}