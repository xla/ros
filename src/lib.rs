#![no_std]
#![no_main]
#![cfg_attr(no_main, test)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

pub mod qemu;
pub mod serial;
pub mod vga_buffer;

/// Custom test runner.
pub fn test_runner(tests: &[&dyn Fn()]) {
    serial_println!("Running {} tests", tests.len());

    for test in tests {
        test();
    }

    serial_println!("Tests complete!");

    qemu::exit(qemu::ExitCode::Success);
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[failure]\n");
    serial_println!("Error: {}\n", info);
    qemu::exit(qemu::ExitCode::Failed);
    loop {}
}

/// Entry point for `cargo xtest`.
#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}

/// Test helpers for common case of success and failure report.
#[macro_export]
macro_rules! test_fail {
    ($fmt:expr) => {
        $crate::serial_println!("[failed]");
        $crate::serial_print!(concat!("\t", $fmt, "\n"));
        $crate::qemu::exit($crate::qemu::ExitCode::Failed);

        loop {}
    };
    ($fmt:expr, $($arg:tt)*) => {
        $crate::serial_println!("[failed]");
        $crate::serial_print!(concat!("\t", $fmt, "\n"), $($arg)*);
        $crate::qemu::exit($crate::qemu::ExitCode::Failed);

        loop {}
    };
}

#[macro_export]
macro_rules! test_ok {
    () => {
        $crate::serial_println!("[ok]");
        $crate::qemu::exit($crate::qemu::ExitCode::Success);
    };
}
