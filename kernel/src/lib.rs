#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]
#![feature(const_mut_refs)]
#![feature(error_in_core)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::alloc::Layout;
use bootloader_api::BootInfo;

#[cfg(test)]
use bootloader_api::{entry_point};

#[cfg(test)]
entry_point!(test_kernel_main);

#[cfg(test)]
fn test_kernel_main(boot_info: &'static mut BootInfo) -> ! {
    init(boot_info);
    test_main();
    hlt_loop();
}

extern crate alloc;

pub mod usb;
pub mod serial;
pub mod interrupts;
pub mod gdt;
pub mod memory;
pub mod allocator;
pub mod task;
pub mod frame_buffer_writer;
pub mod register;

use core::panic::PanicInfo;
use crate::frame_buffer_writer::{FRAME_BUFFER_WRITER};


pub fn init(boot_info: &'static mut BootInfo) {
    let BootInfo {
        framebuffer,
        ..
    } = boot_info;

    let frame_buffer_info = framebuffer.as_ref().unwrap().info();
    FRAME_BUFFER_WRITER.lock().init(framebuffer.as_mut().unwrap().buffer_mut(), frame_buffer_info);
    // gdt::init();
    // interrupts::init_idt();
    // unsafe { interrupts::PICS.lock().initialize() };
    // x86_64::instructions::interrupts::enable();
}


pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T where T: Fn(), {
    fn run(&self) -> () {
        serial_print!("{}..\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}

pub fn test_runner(tests: &[&dyn Testable]) {
    test_start(tests.len());
    for test in tests {
        test.run();
    }

    exit_qemu(QemuExitCode::Success);
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[failed]");
    serial_println!("Error: {}", info);
    exit_qemu(QemuExitCode::Failed);
    hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}

#[alloc_error_handler]
fn alloc_error_handler(layout: Layout) -> ! {
    panic!("allocation error: {:?}", layout)
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
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
        let mut port = Port::new(0xF4);
        port.write(exit_code as u32);
    }
}

pub fn test_start(len: usize)
{
    serial_println!("// ============================================================================================");
    serial_println!("// Running {} Tests", len);
    serial_println!("// ============================================================================================");
}