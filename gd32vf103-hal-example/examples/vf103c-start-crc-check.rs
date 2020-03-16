// vf103c-start-crc-check.rs (GD32VF103 hardware CRC peripheral example)
// Feb 24 2020, author: @luojia65
// If CRC is checked correct, LED with anode on PA7 will be lit.
// 2020-3-14: verified on GD32VF103C-START board

#![no_std]
#![no_main]

use gd32vf103_hal::{crc::Crc, pac, prelude::*};
use panic_halt as _;

#[riscv_rt::entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let mut rcu = dp.RCU.constrain();
    let mut gpioa = dp.GPIOA.split(&mut rcu.apb2);
    let mut pa7 = gpioa.pa7.into_push_pull_output(&mut gpioa.ctl0);

    let src: u32 = 0xABCD1234;
    let crc = Crc::crc(dp.CRC, &mut rcu.ahb);
    let mut digest = crc.new_digest();
    digest.write_u32(src);

    if digest.finish() == 0xF7018A40 {
        pa7.set_high().unwrap();
    }

    loop {}
}
