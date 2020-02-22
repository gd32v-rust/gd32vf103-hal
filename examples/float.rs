#![feature(asm, alloc_error_handler)]
#![no_std]
#![no_main]

extern crate alloc;
extern crate panic_halt;

use linked_list_allocator::LockedHeap;

// This is an empty heap, no allocations are possible.
// Will result in an `oom`.
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
    // uses variant `s` so it's not optimized out
    unsafe { asm!(""::"r"(s.len())) }; 
    loop {}
}
