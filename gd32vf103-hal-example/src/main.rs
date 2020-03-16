#![no_std]
#![no_main]

use gd32vf103_hal::{ctimer, delay, pac, prelude::*, rcu};
use panic_halt as _;

#[riscv_rt::entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let mut rcu = dp.RCU.constrain();

    let mut gpioa = dp.GPIOA.split(&mut rcu.apb2);
    let mut pa7 = gpioa.pa7.into_push_pull_output(&mut gpioa.ctl0);

    let clocks = rcu::Strict::new().freeze(&mut rcu.cfg);
    let ctimer = ctimer::CoreTimer::new(dp.CTIMER);
    let mut delay = delay::Delay::new(clocks, ctimer);
    loop {
        pa7.toggle().unwrap();
        delay.delay_ms(1000u32);
    }
}
