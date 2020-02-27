// crc-check.rs (GD32VF103 CRC peripheral example)
// If CRC is checked correct, LED with cathode on PA1 will be lit.

#![no_std]
#![no_main]

extern crate panic_halt;

use gd32vf103_hal::{crc::Crc, pac, prelude::*};

#[riscv_rt::entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let mut rcu = dp.RCU.constrain();
    let mut gpioa = dp.GPIOA.split(&mut rcu.apb2);
    let mut pa1 = gpioa.pa1.into_push_pull_output(&mut gpioa.ctl0);

    let src: u32 = 0xABCD1234;
    let crc = Crc::crc(dp.CRC, &mut rcu.ahb);
    let mut digest = crc.new_digest();
    digest.write_u32(src);

    if digest.finish() == 0xF7018A40 {
        pa1.set_low().unwrap();
    }

    loop {}
}
