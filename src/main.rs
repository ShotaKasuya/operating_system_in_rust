#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rust_os::test_runner)]
#![reexport_test_harness_main="test_main"]


extern crate alloc;

use alloc::boxed::Box;
use alloc::rc::Rc;
use alloc::vec;
use alloc::vec::Vec;
use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};
use rust_os::{allocator, println};
use rust_os::memory::BootInfoFrameAllocator;

entry_point!(kernel_main);


fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use rust_os::allocator;
    use rust_os::memory;
    use x86_64::VirtAddr;
    use x86_64::structures::paging::Page;

    println!("Hello World{}", "!");

    rust_os::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe {memory::init(phys_mem_offset)};
    let mut frame_allocator = unsafe {BootInfoFrameAllocator::init(&boot_info.memory_map)};

    // 未使用ページマッピング
    let page = Page::containing_address(VirtAddr::new(0xdeadbeaf000));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    // 新しいマッピングで画面に文字を出す
    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe {page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e)};

    allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("heap initialization failed");

    let heap_value= Box::new(41);
    println!("heap_value at {:p}", heap_value);

    // 動的サイズのベクタを作成する
    let mut vec = Vec::new();
    for i in 0..500 {
        vec.push(i);
    }
    println!("vec at {:p}", vec.as_slice());

    let reference_counted = Rc::new(vec![1,2,3]);
    let cloned_reference = reference_counted.clone();
    println!("current reference count is {}", Rc::strong_count(&cloned_reference));
    core::mem::drop(reference_counted);
    println!("reference count is {} now", Rc::strong_count(&cloned_reference));

    #[cfg(test)]
    test_main();

    println!("It did not crash!");

    rust_os::hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    rust_os::hlt_loop();
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
