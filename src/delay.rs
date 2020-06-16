//! Delays
use crate::ctimer::CoreTimer;
use crate::rcu::Clocks;
use crate::unit::*;
use embedded_hal::blocking::delay::DelayMs;
use core::convert::Infallible;

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
            clock_frequency: clocks.ck_sys(), // SystemCoreClock
        }
    }

    /// Release and return the ownership of the core timer resource
    pub fn free(self) -> CoreTimer {
        self.ctimer
    }
}

impl DelayMs<u32> for Delay {
    type Error = Infallible;
    // This doesn't wait for a systick tick, so may be off by a few ns. Sorry
    // The divide by two may be incorrect for other dividors. It should be 8
    // according to the clock diagram, but 2 is accurate. I suspect
    // this will need to change with further documentation updates.
    // -----
    // @luojia65: Ref: Examples/ADC/ADC0_TIMER1_trigger_inserted_channel/systick.c
    //      the divide factor is 4000.0
    fn try_delay_ms(&mut self, ms: u32) -> Result<(), Self::Error> {
        // factor 4000 is verified from official example files
        // leave u64 here
        let count: u64 = ms as u64 * (self.clock_frequency.0 / 4000) as u64;
        let tmp: u64 = self.ctimer.get_value();
        let mut start: u64 = self.ctimer.get_value();
        while start == tmp {
            start = self.ctimer.get_value();
        }
        // prevent u64 overflow
        while u64::wrapping_sub(self.ctimer.get_value(), start) < count {}
        Ok(())
    }
}
