#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rust_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use bootloader_api::config::{BootloaderConfig, Mapping};
use bootloader_api::{entry_point, BootInfo};
use core::panic::PanicInfo;
use kernel::{init, println};
use kernel::frame_buffer_writer::FRAME_BUFFER_WRITER;
use kernel::frame_buffer_writer::pixel_color::PixelColor;
use kernel::frame_buffer_writer::vector2d::Vector2D;


entry_point!(kernel_main, config = &BOOTLOADER_CONFIG);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    init(boot_info);
    {
        let mut writer = FRAME_BUFFER_WRITER.lock();
        let width = writer.info.width;
        let height = writer.info.height;

        writer.fill_rectangle(Vector2D::new(0, 0), Vector2D::new(width, height - 50), PixelColor::black());
        writer.fill_rectangle(Vector2D::new(0, height - 50), Vector2D::new(width, 50), PixelColor::cyan());
        writer.fill_rectangle(Vector2D::new(0, height - 50), Vector2D::new(width / 5, 50), PixelColor::new(80, 80, 80));
        writer.draw_rectangle(Vector2D::new(10, height - 40), Vector2D::new(30, 30), PixelColor::new(160, 160, 160));
    }

    println!("Hello World{}", "!");

    #[cfg(test)]
    test_main();

    kernel::hlt_loop();
}


pub static BOOTLOADER_CONFIG: BootloaderConfig = {
    let mut config = BootloaderConfig::new_default();
    config.mappings.physical_memory = Some(Mapping::Dynamic);
    config
};

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    kernel::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kernel::test_panic_handler(info);
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
