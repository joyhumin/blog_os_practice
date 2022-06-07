#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

pub mod serial;
pub mod vga_buffer;
pub mod interrupts;
pub mod gdt;

use core::panic::PanicInfo;

pub trait Testable {
    fn run(&self) {}
}

impl<T> Testable for T
    where T: Fn(),
{
    fn run(&self) {
        // \t character used to align Ok message.
        serial_print!("{}...\t", core::any::type_name::<T>()); // any::type_name will print out the function name.
        self();
        serial_println!("[Ok]");
    }
}

pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }

    exit_qemu(QemuExitCode::Success);
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

/// Entry point for `cargo test`
#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    init(); // independent to main, there fore we also need to set up IDT
    test_main();
    loop {}
}

/// This function is called on panic.
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info);
    loop {}
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

pub fn init() {
    gdt::init();
    interrupts::init_idt();
    // initialise programmable Interrupt controller
    unsafe { interrupts::PICS.lock().initialize()};
    x86_64::instructions::interrupts::enable(); // executes the set inerrupts to enable external interrupts
}