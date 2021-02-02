#![no_std] // unlink the standard library
#![no_main] // disable rust-level entry points
#![feature(custom_test_frameworks)] // custom test framework without libs
#![test_runner(rust_os::test_runner)] // for overwriting cargo test
#![reexport_test_harness_main = "test_main"] // rename generated function so it doesn't get recognized as main

use core::panic::PanicInfo;

use rust_os::println;
use bootloader::{BootInfo, entry_point};
use rust_os::memory::BootInfoFrameAllocator;

entry_point!(kernel_main);

/// Our kernel entry point
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use rust_os::memory;
    use x86_64::{
        structures::paging::Page,
        VirtAddr,
    };

    println!("Hello World{}", "!");
    rust_os::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };
    // map an unused page
    let page = Page::containing_address(VirtAddr::new(0));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    // write the string `New!` to the screen through the new mapping
    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e)}; // the hex we write prints out New!

    #[cfg(test)]
    test_main(); // execute test when in test mode

    println!("It did not crash!");
    rust_os::hlt_loop();
}

// this will be our panic handler
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    rust_os::hlt_loop();
}

// panic for tests should display in console, not on screen
#[cfg(test)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    rust_os::test_panic_handler(_info);
}