// red-pill-lights.rs (Turn on the light on PA1)
// Author: luojia65 <luojia65@hust.edu.cn> Wuhan, China
// This example is verified on Longan Nano board at 18 Sep 2019
// Update (2020-03-05): use Rust 2018 syntax

#![no_std]
#![no_main]

use gd32vf103_hal::{pac, prelude::*};
use panic_halt as _;

#[riscv_rt::entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let mut rcu = dp.RCU.constrain();
    let mut gpioa = dp.GPIOA.split(&mut rcu.apb2);
    let mut pa1 = gpioa.pa1.into_push_pull_output(&mut gpioa.ctl0);
    pa1.set_low().unwrap();
    loop {}
}
