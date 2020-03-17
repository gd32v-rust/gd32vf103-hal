// vf103c-start-backup-tamper.rs
// Use tweezers to connect PC13 (pin 2) and VDD_3 (pin 48)
// The LED should be switched on or off.
// Note: on this board GD32VF103Cx chip has 48 pins in total

#![no_std]
#![no_main]

use gd32vf103_hal as hal;
use hal::{pac, prelude::*, backup::*};
use panic_halt as _;

#[riscv_rt::entry]
fn main() -> ! { 
    let mut dp = pac::Peripherals::take().unwrap();
    let mut rcu = dp.RCU.constrain();
    let mut bkp = dp.BKP.split(&mut rcu.apb1, &mut dp.PMU);

    let mut gpioa = dp.GPIOA.split(&mut rcu.apb2);
    let mut gpioc = dp.GPIOC.split(&mut rcu.apb2);
    let mut pa7 = gpioa.pa7.into_push_pull_output(&mut gpioa.ctl0);
    let _pc13 = gpioc.pc13.into_alternate_open_drain(&mut gpioc.ctl1);

    bkp.tamper.set_pin_active_high();
    bkp.tamper.clear_event_bit();
    bkp.tamper.enable();

    loop {
        if bkp.tamper.check_event() {
            pa7.toggle().ok();
            bkp.tamper.clear_event_bit();
        }
    }
}
