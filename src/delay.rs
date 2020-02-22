//! Delay
use crate::rcu::Clocks;
use crate::ctimer::CoreTimer;
use crate::time::*;
use embedded_hal::blocking::delay::DelayMs;

/// Hardware timers
pub struct Delay {
    ctimer: CoreTimer,
    clock_frequency: Hertz,
}

impl Delay {
    // note: Clocks : Copy
    pub fn new(clocks: Clocks, ctimer: CoreTimer) -> Self {
        Delay {
            ctimer,
            clock_frequency: clocks.ck_ahb(),
        }
    }
}

impl<T: Into<u32>> DelayMs<T> for Delay {
    // This doesn't wait for a systick tick, so may be off by a few ns. Sorry
    // The divide by two may be incorrect for other dividors. It should be 8
    // according to the clock diagram, but 2 is accurate. I suspect
    // this will need to change with further documentation updates.
    fn delay_ms(&mut self, ms: T) {
        let count: u32 = ms.into() * self.clock_frequency.0 / 1000 / (2);
        let tmp: u64 = self.ctimer.get_value();
        let end = tmp + count as u64;
        while self.ctimer.get_value() < end {}
    }
}
