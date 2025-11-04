#![no_std]
#![no_main] // Disables language-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
/*
* By default the test framework generates a main function that calls test_runner, but the no_main
* flag ignores it. this changes the generated function's name, which is then called from the entry
* point (only on tests).
*/
#![reexport_test_harness_main = "test_main"]

#[cfg(test)] // Include only for tests
pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    exit_qemu(QemuExitCode::Success);
}

// Avoids manually printing the output of every test
pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

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
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
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

    #[cfg(test)]
    test_main();

    loop {}
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, true as u8);
}
