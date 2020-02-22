#![feature(asm)]
#![no_std]
#![no_main]

extern crate panic_halt;

use gd32vf103_hal as hal;
use hal::pac;
use hal::prelude::*;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let mut rcu = dp.RCU.constrain();
    let mut gpioa = dp.GPIOA.split(&mut rcu.apb2);
    // let mut gpioc = dp.GPIOC.split(&mut rcu.apb2);
    let mut pa1 = gpioa.pa1.into_push_pull_output(&mut gpioa.ctl0);
    let mut pa2 = gpioa.pa2.into_push_pull_output(&mut gpioa.ctl0);
    // let mut pc13 = gpioc.pc13.into_push_pull_output(&mut gpioc.ctl1);
    loop {
        for _ in 0..10_000 {
            unsafe { asm!("nop") };
        }
        pa2.set_low().unwrap();
        pa1.set_high().unwrap();
        for _ in 0..10_000 {
            unsafe { asm!("nop") };
        }
        pa1.set_low().unwrap();
        pa2.set_low().unwrap();
        for _ in 0..10_000 {
            unsafe { asm!("nop") };
        }
        pa1.set_low().unwrap();
        pa2.set_high().unwrap();
        for _ in 0..10_000 {
            unsafe { asm!("nop") };
        }
        pa1.set_low().unwrap();
        pa2.set_low().unwrap();
    }
}

// const FCK_L: usize = 0xd1000000;
// const FCK_H: usize = 0xd1000004;

// fn delay_ms(ms: u32) {
//     // use riscv::register::time::read; //fxck! this is not supported here
//     let begin = unsafe { *(FCK_L as *const u32) }; //read() as u32;
//     let end = begin + (ms) * 16;
//     while unsafe { *(FCK_L as *const u32) }/*(read() as u32)*/
//         < end {}
// }
