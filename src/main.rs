#![no_std]
#![no_main] // Disables language-level entry points

use core::panic::PanicInfo;

#[panic_handler]
// Never returns as it is called on a crash
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO_WORLD: &[u8] = b"Hello World!";

/*
Disable name mangling so the compiler actually outputs a function with the name _start and not
some unique character string to give every function a unique name.
By doing this we can tell the linker the name of the entry point function is _start (default name for most systems).
Rust doesn't support function overloading anyway.
*/
#[unsafe(no_mangle)]
/*
Use C calling convention (should be __cdecl)
Instead of Rust's because it's unspecified w/o std library
*/
/*
The function is diverging because the entry point is not called by any function, but by the os or bootloader.
Instead of returning, it should invoke the exit syscall (for example)

TLDR _start = main
*/
pub extern "C" fn _start() -> ! {
    let vga_buffer: *mut u8 = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO_WORLD.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb; // 0xb = light cyan
        }
    }

    loop {}
}
