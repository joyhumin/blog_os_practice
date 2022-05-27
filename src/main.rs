// disable standard library links
#![no_std]
#![no_main] // disable all Rust-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod vga_buffer;

use core::fmt::Write;
use core::panic::PanicInfo;
use x86_64::instructions::port::Port;

static HELLO: &[u8] = b"Hello World!";

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    // this function is the entry point, since the linker looks for a function
    // named `_start` by default
    println!("Hello World{}", "!");
    #[cfg(test)]
    test_main();
    loop {
    }
}

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

// linker, combines the generated code into an executable.


#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test()
    }

    exit_qemu(QemuExitCode::Success);
}

#[test_case]
fn trivial_assertion(){
    print!("trivial assertion ...");
    assert_eq!(1, 1);
    println!("[ok]");
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode{
    Success = 0x10, // hexadecimal integers (base 16)
    Failed = 0x11
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4); // iobase of isa-debug-exit
        port.write(exit_code as u32);
    }
}