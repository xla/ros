/// Codes used to signal exit condition to Qemu.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum ExitCode {
    Success = 0x10,
    Failed = 0x11,
}

/// Interacts with prot mapped memory for signaling to Qemu.
pub fn exit(exit_code: ExitCode) {
    unsafe {
        let mut port = x86_64::instructions::port::Port::new(0xf4);
        port.write(exit_code as u32);
    }
}
