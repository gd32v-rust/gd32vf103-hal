#![feature(asm, alloc_error_handler)]
#![no_std]
#![no_main]

extern crate panic_halt;
extern crate alloc;

use linked_list_allocator::LockedHeap;

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

#[alloc_error_handler]
fn oom(_: core::alloc::Layout) -> ! {
    loop {}
}

use alloc::format;

#[riscv_rt::entry]
fn main() -> ! {
    let a = 2.33f32;
    let s = format!("{}", a);
    unsafe { asm!(""::"r"(s.len())) };
    loop {}
}
