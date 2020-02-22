#![feature(asm)]
#![no_std]
#![no_main]

extern crate panic_halt;

use embedded_hal::blocking::delay::DelayMs;
use gd32vf103_hal as hal;
use hal::delay;
use hal::pac;
use hal::prelude::*;
use hal::ctimer;
use riscv_rt::entry;

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    let mut rcu = dp.RCU.constrain();

    let mut gpioa = dp.GPIOA.split(&mut rcu.apb2);
    let mut pa1 = gpioa
        .pa1
        .into_push_pull_output(&mut gpioa.ctl0)
        .lock(&mut gpioa.lock);
    gpioa.lock.freeze();

    let clocks = rcu.clocks;
    let ctimer = ctimer::CoreTimer::new(dp.CTIMER);
    let mut delay = delay::Delay::new(clocks, ctimer);
    loop {
        pa1.toggle().unwrap();
        delay.delay_ms(1000 as u32);
    }
}
