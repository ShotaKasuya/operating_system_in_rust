#![no_std]
#![no_main]

mod vga_buffer;

use core::fmt::Write;
use core::panic::PanicInfo;

static HELLO:&[u8]=b"Hello World!";
#[no_mangle]
pub extern "C" fn _start()->!{
    // println!("Hello World{}", "!");
    panic!("some error");

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo)->!{
    println!("{}", _info);
    loop {}
}