use core::panic::PanicInfo;

#[panic_handler] // We cannot use the rust panic handler
fn panic(_info: &PanicInfo) -> ! {
  loop {}
}