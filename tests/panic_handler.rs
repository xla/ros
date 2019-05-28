#![no_std]
#![no_main]
#![feature(panic_info_message)]

use core::fmt::{self, Write};
use core::panic::PanicInfo;

use ros::{serial_print, test_fail, test_ok};

const MESSAGE: &str = "Example panic message from panic_handler test";
const PANIC_LINE: u32 = 16;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    serial_print!("panic_handler...");
    panic!(MESSAGE);
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    check_message(info);
    check_location(info);

    test_ok!();

    loop {}
}

fn check_location(info: &PanicInfo) {
    if let Some(location) = info.location() {
        if location.file() != file!() {
            test_fail!("wrong file name: {} != {}", location.file(), file!());
        }
        if location.line() != PANIC_LINE {
            test_fail!("file line wrong: {} != {}", location.line(), PANIC_LINE);
        }
    } else {
        test_fail!("no location");
    }
}

fn check_message(info: &PanicInfo) {
    if let Some(message) = info.message() {
        let mut compare_message = CompareMessage::new(MESSAGE);

        if let Ok(_) = write!(&mut compare_message, "{}", message) {
            if !compare_message.expected.is_empty() {
                test_fail!("message shorter than expected message");
            }
        } else {
            test_fail!("write failed");
        }
    } else {
        test_fail!("no message");
    }
}

/// Compares a `fmt::Arguments` instance with the `MESSAGE` string.
///
/// To use this type, write the `fmt::Arguments`  instance to it using the `write` macro, If the
/// message component matches `MESSAGE`, the `expected` field is the empty string.
struct CompareMessage {
    expected: &'static str,
}

impl CompareMessage {
    fn new(expected: &'static str) -> Self {
        Self { expected }
    }
}

impl fmt::Write for CompareMessage {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        if self.expected.starts_with(s) {
            self.expected = &self.expected[s.len()..];
        } else {
            test_fail!("message not equal to expected message");
        }

        Ok(())
    }
}
