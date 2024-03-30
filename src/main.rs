#![no_std]
#![no_main]

mod vga_buffer;

use core::fmt::Write;
use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start()->!{
    for i in 1..5 {
        println!("Hello World{}", i);
    }
    panic!("some");

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo)->!{
    println!("{}", _info);
    loop {}
}