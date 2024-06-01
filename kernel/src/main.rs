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
use kernel::usb::{Device, DEVICES, scan_all_bus};


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

    if let Err(err) = scan_all_bus() {
        println!("Error: {:?}", err);
    }
    println!("Scan all Bus Success");
    {
        let devices = DEVICES.lock();
        for i in 0..devices.num {
            let device = &devices.devices[i];
            println!("{}.{}.{}: vend {:04X}, class {:08X}, head {:02X}",
            device.bus, device.device, device.function,
            device.read_vendor_id(), device.read_class_code(), device.header_type);
        }
        let mut xhc_dev: Option<&Device> = None;
        for i in 0..devices.num {
            if [0x0C, 0x03, 0x30].contains(&devices.devices[i].read_class_code()) {
                xhc_dev = Some(&devices.devices[i]);

                if 0x8086 == xhc_dev.unwrap().read_vendor_id() {
                    break;
                }
            }
        }

        if let Some(xhc_dev) = xhc_dev {
            println!("xHC has been found: {}.{}.{}",
            xhc_dev.bus, xhc_dev.device, xhc_dev.function);
        }

        let xhc_bar = xhc_dev.unwrap().read_bar(0).unwrap();
        println!("read_bar: {}", xhc_bar);
        let xhc_mmio_base = xhc_bar & !0xFu64;
        println!("xHC mmio_base = {:08}",xhc_mmio_base);
    }


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
