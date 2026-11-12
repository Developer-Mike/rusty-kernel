#![no_std] // Use no std lib
#![no_main] // Nothing can call main in our kernel

mod panic;
mod vga;

#[unsafe(no_mangle)] // Do not mangle _start name
pub extern "C" fn _start() -> ! { // Expose as a C function
  let mut writer = vga::writer::Writer::new();

  writer.string("Hello world\n");
  writer.string("Hello world\n");

  loop {}
}