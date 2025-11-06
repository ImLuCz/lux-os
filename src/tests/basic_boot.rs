#![test_runner(lux_os::test_runner)]

use blog_os::println;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    lux_os::test_panic_handler(info)
}

#[test_case]
fn test_println() {
    println!("test_println output");
}
