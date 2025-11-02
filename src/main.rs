#![no_std]
#![no_main] // Disables language-level entry points

use core::panic::PanicInfo;

mod vga_buffer;

#[panic_handler]
// Never returns as it is called on a crash
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
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
but by the os or bootloader.
Instead of returning, it should invoke the exit syscall (for example)

TLDR _start = main
*/
pub extern "C" fn _start() -> ! {
    println!("Hello, World{}", "!");

    loop {}
}
