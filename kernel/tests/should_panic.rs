#![no_std]
#![no_main]

use core::panic::PanicInfo;
use rust_os::{QemuExitCode, exit_qemu, serial_print, serial_println, test_start};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_start(1);
    should_fail();
    serial_print!("[test did not panic]");
    exit_qemu(QemuExitCode::Failed);

    loop{}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("[ok]");
    exit_qemu(QemuExitCode::Success);
    loop {}
}

fn should_fail() {
    serial_println!("should_panic::should_fail\t");
    assert_eq!(0, 1);
}