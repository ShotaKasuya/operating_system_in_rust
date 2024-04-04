#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use rust_os::{exit_qemu, println, test_panic_handler, test_start};
use rust_os::QemuExitCode::Success;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}

fn test_runner(_tests: &[& dyn Fn()]) {
    test_start(_tests.len());
    // unimplemented!();
    exit_qemu(Success);
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}

#[test_case]
fn test_println() {
    println!("test_println output");
}