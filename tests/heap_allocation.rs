#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rso::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

use alloc::{boxed::Box, vec::Vec};
use bootloader::{BootInfo, entry_point};
use rso::{
    allocator::{self, HEAP_SIZE},
    hit_loop, memory,
};
use x86_64::VirtAddr;
extern crate alloc;

entry_point!(main);

fn main(boot_info: &'static BootInfo) -> ! {
    rso::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);

    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator =
        unsafe { memory::BootInfoFrameAllocator::init(&boot_info.memory_map) };

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    test_main();
    hit_loop()
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rso::test_panic_handler(info)
}

#[test_case]
fn large_vec() {
    let n = 1000;
    let mut vec = Vec::new();
    for i in 0..n {
        vec.push(i);
    }
    assert_eq!(vec.iter().sum::<u64>(), (n - 1) * n / 2)
}

#[test_case]
fn many_boxes_long_lived() {
    let long_lived = Box::new(1);
    for i in 0..HEAP_SIZE {
        let x = Box::new(i);
        assert_eq!(*x, i);
    }
    assert_eq!(*long_lived, 1);
}
