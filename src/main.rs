// disable standard library links
#![no_std]
#![no_main] // disable all Rust-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod vga_buffer;
mod serial;

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
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

// linker, combines the generated code into an executable.

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode{
    Success = 0x10, // hexadecimal integers (base 16)
    Failed = 0x11
}

pub fn exit_qemu(exit_code: QemuExitCode) {

    unsafe {
        let mut port = Port::new(0xf4); // iobase of isa-debug-exit
        port.write(exit_code as u32);
    }
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }

    exit_qemu(QemuExitCode::Success);
}

pub trait Testable {
    fn run(&self) {}
}

impl<T> Testable for T
where T: Fn(),
{
    fn run(&self) {
        serial_print!("{}...\t", core::any::type_name::<T>()); // any::type_name will print out the function name.
        self();
        serial_println!("[Ok]");
    }
}

#[test_case]
fn trivial_assertion(){
    assert_eq!(1, 1);
}