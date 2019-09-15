#![no_std]
#![no_main]

extern crate panic_halt;

use riscv_rt::entry;
use gd32vf103_hal as hal;
use hal::prelude::*;
use hal::pac as pac;

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let mut rcu = dp.RCU.constrain();
    let mut gpioa = dp.GPIOA.split(&mut rcu.apb2);
    let mut pa1 = gpioa.pa1.into_push_pull_output(&mut gpioa.ctl0);
    loop {    
        delay_ms(1000);
        pa1.set_high().unwrap();
        delay_ms(1000);
        pa1.set_low().unwrap();
    }
}

fn delay_ms(ms: u32) {
    use riscv::register::time::read64; //fxck! this is not supported here
    let begin = read64();
    let end = begin + (ms as u64) * 16000;
    while read64() < end {} 
}
