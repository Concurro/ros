#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rso::test_runner)]
#![reexport_test_harness_main = "test_main"]
extern crate alloc;

use alloc::boxed::Box;
use bootloader::{BootInfo, entry_point};
use core::panic::PanicInfo;
use rso::{
    allocator, hit_loop, memory, println,
    task::{Task, executor::Executor, keyboard, simple_executor::SimpleExecutor},
};
use x86_64::VirtAddr;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    rso::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);

    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator =
        unsafe { memory::BootInfoFrameAllocator::init(&boot_info.memory_map) };

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    let heap_varisable = Box::new(114222222222222222u64);
    println!("{}", heap_varisable);
    // async init
    let mut executor = Executor::new();
    executor.spawn(Task::new(example_task()));
    executor.spawn(Task::new(keyboard::print_keypresses()));
    executor.run();

    #[cfg(test)]
    test_main();

    hit_loop()
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    hit_loop()
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    use rso::test_panic_handler;
    test_panic_handler(info)
}

// async example

async fn async_number() -> i32 {
    74
}

async fn example_task() {
    let number = async_number().await;
    println!("{}", number);
}
