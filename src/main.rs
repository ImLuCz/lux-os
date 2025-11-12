#![no_std]
#![no_main] // Disables language-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(lux_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

mod serial;
mod vga_buffer;

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)] // print test output to host console
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    lux_os::test_panic_handler(info);
}

/*
Disable name mangling so the compiler actually outputs a function with the name
_start and not some unique character string to give every function a unique name.
By doing this we can tell the linker the name of the entry point function is
_start (default name for most systems).
Rust doesn't support function overloading anyway.
*/
#[unsafe(no_mangle)]
/*
Use C calling convention (should be __cdecl)
Instead of Rust's because it's unspecified without std library
*/
/*
The function is diverging because the entry point is not called by any function,
but by bootloader.
Instead of returning, it should invoke the exit syscall (for example)

TLDR _start = main
*/
pub extern "C" fn _start() -> ! {
    println!("Hello, World{}", "!");
    lux_os::init();

    #[cfg(test)]
    test_main();

    println!("didn't crash");
    loop {}
}
