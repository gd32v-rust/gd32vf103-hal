#![no_std]
#![no_main]

extern crate panic_halt;

use gd32vf103_hal::{pac, prelude::*};

#[riscv_rt::entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let mut rcu = dp.RCU.constrain();
    let mut gpioa = dp.GPIOA.split(&mut rcu.apb2);
    let mut pa1 = gpioa.pa1.into_push_pull_output(&mut gpioa.ctl0);
    unsafe {
        (*pac::RCU::ptr())
            .cfg0
            .write(|w| w.ahbpsc().bits(0b0111).apb1psc().bits(0b111));

        (*pac::RCU::ptr()).apb1en.write(|w| w.timer1en().set_bit());
        (*pac::RCU::ptr())
            .apb1rst
            .write(|w| w.timer1rst().set_bit());

        (*pac::TIMER1::ptr()).ctl0.write(|w| w.cen().clear_bit());

        let freq_hz = 1;
        let timer_clock = 8_000_000 as u32;
        let ticks = timer_clock / freq_hz;
        let psc = ((ticks - 1) / (1 << 16)) as u16;
        (*pac::TIMER1::ptr()).psc.write(|w| w.bits(psc));
        let car = (ticks / ((psc + 1) as u32)) as u16;
        (*pac::TIMER1::ptr()).car.write(|w| w.bits(car));

        (*pac::TIMER1::ptr()).ctl0.write(|w| w.ups().set_bit());
        (*pac::TIMER1::ptr()).swevg.write(|w| w.upg().set_bit());
        (*pac::TIMER1::ptr()).ctl0.write(|w| w.ups().clear_bit());

        (*pac::TIMER1::ptr()).ctl0.write(|w| w.cen().set_bit());
    }
    // clock output
    let mut _pa8 = gpioa.pa8.into_alternate_push_pull(&mut gpioa.ctl1);
    /*
        00xx: No clock selected
        0100: System clock selected
        0101: High Speed 8M Internal Oscillator clock selected
        0110: External High Speed oscillator clock selected
        0111: (CK_PLL / 2) clock selected
        1000: CK_PLL1 clock selected
        1001: CK_PLL2 clock divided by 2 selected
        1010: EXT1 selected
        1011: CK_PLL2 clock selected
    */
    unsafe {
        (*pac::RCU::ptr())
            .cfg0
            .write(|w| w.ckout0sel().bits(0b1000));
    }
    loop {
        pa1.set_high().unwrap();
        while unsafe { &(*pac::TIMER1::ptr()) }.intf.read().upif().bit() {}
        unsafe { &(*pac::TIMER1::ptr()) }
            .intf
            .write(|w| w.upif().clear_bit());
        pa1.set_low().unwrap();
        while unsafe { &(*pac::TIMER1::ptr()) }.intf.read().upif().bit() {}
        unsafe { &(*pac::TIMER1::ptr()) }
            .intf
            .write(|w| w.upif().clear_bit());
    }
}
