#![no_std] // Use no std lib
#![no_main] // Nothing can call main in our kernel

use bootloader::{entry_point, BootInfo};

mod panic;
mod vga;

entry_point!(kernel_main);

fn kernel_main(_boot_info: &'static BootInfo) -> ! {
  vga::writer::println("Hello world");
  vga::writer::println("Hello world");

  loop {}
}