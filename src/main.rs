#![no_std] // unlink the standard library
#![no_main] // disable rust-level entry points

use core::panic::PanicInfo;

mod vga_buffer;

#[no_mangle] // don't mangle name randomly
pub extern "C" fn _start() -> ! {
    println!("hello world{}", "!");
    // this is our entry point called in C
    // the linker selects function _start by default
    loop {}
}

// this function is our panic handler
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}


