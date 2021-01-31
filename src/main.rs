#![no_std] // unlink the standard library
#![no_main] // disable rust-level entry points

use core::panic::PanicInfo;

#[no_mangle] // don't mangle name randomly
pub extern "C" fn _start() -> ! {
    print_string();
    // this is our entry point called in C
    // the linker selects function _start by default
    loop {}
}

fn print_string() {
    // prints simple hello world to screen
    static HELLO: &[u8] = b"Hello world!";
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }
}

// this function is our panic handler
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}


