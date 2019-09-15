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
    unsafe {
        (*pac::RCU::ptr()).cfg0.write(|w| w
            .ahbpsc().bits(0b0111)
            .apb1psc().bits(0b111)
        );
        
        (*pac::RCU::ptr()).apb1en.write(|w| w.timer1en().set_bit());
        (*pac::RCU::ptr()).apb1rst.write(|w| w.timer1rst().set_bit());
                
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
    loop {    
        pa1.set_high().unwrap();
        while unsafe { &(*pac::TIMER1::ptr()) }
            .intf.read().upif().bit() {}
        unsafe { &(*pac::TIMER1::ptr()) }.intf.write(|w| w.upif().clear_bit());
        pa1.set_low().unwrap();
        while unsafe { &(*pac::TIMER1::ptr()) }
            .intf.read().upif().bit() {}
        unsafe { &(*pac::TIMER1::ptr()) }.intf.write(|w| w.upif().clear_bit());
    }
}
