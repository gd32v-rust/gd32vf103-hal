//! (TODO) Watchdog Timer (WDGT)
//!
//! Ref: Section 13, the User Manual

use crate::pac::{FWDGT, WWDGT};
use crate::time::MicroSeconds;
use embedded_hal::watchdog::{Watchdog, WatchdogEnable};

// Ref: Section 13.1.4
const FWDGT_CMD_ENABLE: u16 = 0xCCCC;
const FWDGT_CMD_RELOAD: u16 = 0xAAAA;
const FWDGT_CMD_WRITE_ACCESS_ENABLE: u16 = 0x5555;
const FWDGT_CMD_WRITE_ACCESS_DISABLE: u16 = 0x0000;

// in this library we do not use 0b111 as psc(0b110) equals psc(0b111)
const FWDGT_MAX_PSC: u8 = 0b110;
// RLD == 0 => multiplier = 1, RLD == FFF => multiplier = 4096
const FWDGT_MAX_RLD: u16 = 0xFFF;

/// Free Watchdog Timer (FWDGT) peripheral
///
/// This watchdog timer cannot be disabled.
///
/// TODO: debug
pub struct Free {
    fwdgt: FWDGT,
}

impl Free {
    /// Wrap the watchdog
    pub fn new(fwdgt: FWDGT) -> Self {
        Free { fwdgt }
    }

    /// Returns the interval in microseconds
    pub fn interval(&self) -> MicroSeconds {
        while self.rud_or_pud_updating() {}
        let psc: u32 = self.fwdgt.psc.read().psc().bits().into();
        let rld: u32 = self.fwdgt.rld.read().rld().bits().into();
        let time_us = (rld + 1) * (1 << psc) * 100;
        MicroSeconds(time_us)
    }

    // todo: stop_on_debug function (DBG peripheral)
}

impl Free {
    #[inline]
    fn rud_or_pud_updating(&self) -> bool {
        let stat = self.fwdgt.stat.read();
        stat.rud().bits() || stat.pud().bits()
    }
    #[inline]
    fn calc_psc_rld(time_us: u32) -> (u8, u16) {
        let mut psc = 0;
        let mut max_time = 409_600; // 4096 * 1/(40KHz * 1/4) = 409.6ms = 409600us
        while psc < FWDGT_MAX_PSC && max_time < time_us {
            psc += 1;
            max_time *= 2;
        }
        let mut rld = u32::saturating_sub(time_us, 1) / (100 * (1 << psc));
        if rld > FWDGT_MAX_RLD as u32 {
            rld = FWDGT_MAX_RLD as u32;
        }
        (psc, rld as u16)
    }
    // ref: stm32f4xx_hal
    fn access_protected<A, F: FnMut(&FWDGT) -> A>(&self, mut f: F) -> A {
        // Unprotect write access to registers
        self.fwdgt
            .ctl
            .write(|w| unsafe { w.cmd().bits(FWDGT_CMD_WRITE_ACCESS_ENABLE) });
        let ans = f(&self.fwdgt);

        // Protect again
        self.fwdgt
            .ctl
            .write(|w| unsafe { w.cmd().bits(FWDGT_CMD_WRITE_ACCESS_DISABLE) });
        ans
    }
}

// todo: if WatchdogEnable start returns an independent Enabled<Free>,
// that struct could impl `configure` function to modify the period value;
// at that time PUD and RUD bits in FWDGT_STAT register could be used.

impl WatchdogEnable for Free {
    type Time = MicroSeconds;

    fn start<T>(&mut self, period: T)
    where
        T: Into<Self::Time>,
    {
        // prepare configurations
        let (psc, rld) = Self::calc_psc_rld(period.into().0);
        // perform config and start process
        riscv::interrupt::free(|_| {
            // According to the Manual, if applications need to modify the PSC and
            // RLD registers, they should wait until PUD and RUD bit is cleared to
            // zero by hardware. However in watchdog start process we assume that
            // these two registers are not modified before thus always are zero,
            // we didn't check these two bits. In future designs if there could be
            // a chance to modify the configuration after watchdog started, we
            // should write PUD and RUD checks into the code to prevent issues.
            self.access_protected(|fwdgt| {
                // note(unsafe): valid values ensured by calc_psc_rld
                fwdgt.psc.modify(|_, w| unsafe { w.psc().bits(psc) });
                fwdgt.rld.modify(|_, w| unsafe { w.rld().bits(rld) });
            });
            // note(unsafe): write valid command constant defined in the Manual
            self.fwdgt
                .ctl
                .write(|w| unsafe { w.cmd().bits(FWDGT_CMD_RELOAD) });
            self.fwdgt
                .ctl
                .write(|w| unsafe { w.cmd().bits(FWDGT_CMD_ENABLE) });
        });
    }
}

impl Watchdog for Free {
    fn feed(&mut self) {
        // note(unsafe): write valid command constant defined in the Manual
        self.fwdgt
            .ctl
            .write(|w| unsafe { w.cmd().bits(FWDGT_CMD_RELOAD) });
    }
}

/// Window Watchdog Timer (WWDGT) peripheral
pub struct Window {
    wwdgt: WWDGT,
}
