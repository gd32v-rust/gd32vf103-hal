#![no_std]
#![no_main]

extern crate panic_halt;

#[riscv_rt::entry]
unsafe fn main() -> ! {
    use gd32vf103_hal::{pac, prelude::*};
    let dp = pac::Peripherals::take().unwrap();
    let mut rcu = dp.RCU.constrain();
    // Split and enable clock for GPIO port A
    let mut gpioa = dp.GPIOA.split(&mut rcu.apb2);
    // Switch PA0 to push-pull output with 50-MHz maximum freq
    let pa0 = gpioa.pa0.into_push_pull_output(&mut gpioa.ctl0);
    // Lock port A to prevent mode configurations
    let mut _pa0 = pa0.lock(&mut gpioa.lock);
    // Drops the ownership of lock
    gpioa.lock.freeze();
    // Try to switch mode for PA0 again
    // let pa0 = pa0.into_open_drain_output(&mut gpioa.ctl0);
    // ^ ERROR: no such method found for type `Locked<PA0<...>>`
    loop {}
}
