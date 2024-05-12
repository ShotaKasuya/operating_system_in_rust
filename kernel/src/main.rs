#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rust_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use bootloader_api::config::{BootloaderConfig, Mapping};
use bootloader_api::{entry_point, BootInfo};
use core::panic::PanicInfo;
// use kernel::memory::BootInfoFrameAllocator;
// use kernel::task::simple_executor::SimpleExecutor;
// use kernel::task::{keyboard, Task};
use kernel::{ println};


// add a `config` argument to the `entry_point` macro call
entry_point!(kernel_main, config = &BOOTLOADER_CONFIG);

fn kernel_main(_boot_info: &'static mut BootInfo) -> ! {
    // use kernel::allocator;
    // use kernel::memory;
    // use x86_64::structures::paging::Page;
    // use x86_64::VirtAddr;

    println!("Hello World{}", "!");

    kernel::init();

    // let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset.take().unwrap());
    // let mut mapper = unsafe { memory::init(phys_mem_offset) };
    // let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    // 未使用ページマッピング
    // let page = Page::containing_address(VirtAddr::new(0xdeadbeaf000));
    // memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    // // 新しいマッピングで画面に文字を出す
    // let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    // unsafe {page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e)};

    // allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    // let mut executor = SimpleExecutor::new();
    // executor.spawn(Task::new(example_task()));
    // executor.spawn(Task::new(keyboard::print_keypress()));
    // executor.run();

    #[cfg(test)]
    test_main();

    println!("It did not crash!");

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
