// main.rs

#![no_std]
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;

static HELLO: &[u8] = b"Hello RustybaraOS!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    vga_buffer::print_something();

    loop {}
}

// This function is called on panic
#[panic_handler]
fn panic(_info:  &PanicInfo) -> ! {
    loop{}
}