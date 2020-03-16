// vf103c-start-blinky.rs (Blink the LED on PA7)
// Author: Luo Jia (@luojia65), Wuhan, China, 2020-03-14
// Tested on GD32VF103C-START board.

// You may need a GDLink OpenOCD configuration. Get it on:
// https://github.com/riscv-mcu/GD32VF103_Demo_Suites/blob/master/openocd_gdlink.cfg
// Replace the `openocd.cfg` file; replace the `-expected-id` to 0x1e200a6d in line 10.
// Then start Nuclei OpenOCD (supports RISC-V). Use `cargo run` to execute.

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
