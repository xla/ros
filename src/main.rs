// Dont link the Rust standard library.
#![no_std]
// Disable all Rust runtime entry points.
#![no_main]
// Enable custom test frameworks.
#![feature(custom_test_frameworks)]
#![reexport_test_harness_main = "test_main"]
#![test_runner(ros::test_runner)]

use core::panic::PanicInfo;

use ros::println;

/// Kernel entry point, since the linker looks for a fucntion named `_start` by default.
/// needs to remain unmangled.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    #[cfg(test)]
    test_main();

    loop {}
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

/// Panic handler in test mode.
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    ros::test_panic_handler(info)
}
