//! Delays
use crate::ctimer::CoreTimer;
use crate::rcu::Clocks;
use crate::time::*;
use embedded_hal::blocking::delay::DelayMs;

/// CoreTimer as delay provider
pub struct Delay {
    ctimer: CoreTimer,
    clock_frequency: Hertz,
}

impl Delay {
    // note: Clocks : Copy
    /// Configures the core timer as a delay provider
    pub fn new(clocks: Clocks, ctimer: CoreTimer) -> Self {
        Delay {
            ctimer,
            clock_frequency: clocks.ck_ahb(),
        }
    }

    /// Release and return the ownership of the core timer resource
    pub fn free(self) -> CoreTimer {
        self.ctimer
    }
}

impl DelayMs<u8> for Delay {
    fn delay_ms(&mut self, ms: u8) {
        self.delay_ms(u32::from(ms))
    }
}

impl DelayMs<u16> for Delay {
    fn delay_ms(&mut self, ms: u16) {
        self.delay_ms(u32::from(ms))
    }
}

impl DelayMs<u32> for Delay {
    // This doesn't wait for a systick tick, so may be off by a few ns. Sorry
    // The divide by two may be incorrect for other dividors. It should be 8
    // according to the clock diagram, but 2 is accurate. I suspect
    // this will need to change with further documentation updates.
    fn delay_ms(&mut self, ms: u32) {
        // todo: verify the divide factor (/4) (luojia65) 
        // todo: temporarily use u64 before a better solution
        let count: u64 = ms as u64 * (self.clock_frequency.0 / 1000 / 4) as u64;
        let tmp: u64 = self.ctimer.get_value();
        let end = tmp + count as u64;
        while self.ctimer.get_value() < end {}
    }
}
