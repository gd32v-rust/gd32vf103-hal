//! Delay
use embedded_hal::blocking::delay::{DelayMs};
use crate::rcu::Clocks;
use crate::systick::Systick;
use crate::time::*;
use gd32vf103_pac::SYSTICK;
/// Hardware timers
pub struct Delay{
    syst: SYSTICK,
    clock_frequency : Hertz,
}


impl Delay {
    pub fn new(clocks: Clocks, syst: SYSTICK) -> Self 
    {
        Delay {
            syst: syst,
            clock_frequency: clocks.ck_ahb(),
        }
    }
}

impl <T: Into<u32>> DelayMs<T> for Delay {
    // This doesn't wait for a systick tick, so may be off by a few ns. Sorry
    fn delay_ms(&mut self, ms: T) {
        let count : u32= (ms.into() * self.clock_frequency.0 / 1000 / (2));
        let tmp : u64 = Systick::get_systick(&self.syst);
        let end = tmp + count as u64;
        while Systick::get_systick(&self.syst) < end{
        }
    }
}
