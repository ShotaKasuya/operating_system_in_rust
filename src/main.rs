#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rust_os::test_runner)]
#![reexport_test_harness_main="test_main"]


use core::panic::PanicInfo;
use rust_os::{println};


#[no_mangle]
pub extern "C" fn _start()->! {
    println!("Hello World{}", "!");

    rust_os::init();

    // Page fault
    unsafe {
        *(0xdeadbeef as *mut u8) = 42;
    }

    x86_64::instructions::interrupts::int3();

    #[cfg(test)]
    test_main();

    println!("It did not crash!");

    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rust_os::test_panic_handler(info);
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
