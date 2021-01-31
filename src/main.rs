#![no_std] // unlink the standard library
#![no_main] // disable rust-level entry points
#![feature(custom_test_frameworks)] // custom test framework without libs
#![test_runner(rust_os::test_runner)] // for overwriting cargo test
#![reexport_test_harness_main = "test_main"] // rename generated function so it doesn't get recognized as main

use core::panic::PanicInfo;

use rust_os::println;

// this is our entry point called in C
// the linker selects function _start by default
#[no_mangle] // don't mangle name randomly
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    #[cfg(test)]
    test_main(); // execute test when in test mode

    loop {}
}

// this will be our panic handler
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

// panic for tests should display in console, not on screen
#[cfg(test)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    rust_os::test_panic_handler(_info);
}
