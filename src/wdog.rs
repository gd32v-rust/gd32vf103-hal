//! (TODO) Watchdog Timer (WDGT)
//!
//! Ref: Section 13, the User Manual

use crate::pac::{FWDGT, WWDGT};
use crate::unit::MicroSeconds;
use embedded_hal::watchdog::{Watchdog, Enable};
use core::convert::Infallible;

// Ref: Section 13.1.4
const FWDGT_CMD_ENABLE: u16 = 0xCCCC;
const FWDGT_CMD_RELOAD: u16 = 0xAAAA;
const FWDGT_CMD_WRITE_ACCESS_ENABLE: u16 = 0x5555;
const FWDGT_CMD_WRITE_ACCESS_DISABLE: u16 = 0x0000;

// in this library we do not use 0b111 as psc(0b110) equals psc(0b111)
const FWDGT_MAX_PSC: u8 = 0b110;
// RLD == 0 => multiplier = 1, RLD == FFF => multiplier = 4096
const FWDGT_MAX_RLD: u16 = 0xFFF;

/*

    We declare this struct Free<Enabled> and Free<Disabled> and
    implement different traits for them.
    However if existing designs still need old style API, they may
    set two Target associated types to Self. Then these designs could
    implement all trait for (e.g.) a Free struct alone.

*/

/// Type state struct for disabled watchdog timers.
pub struct Disabled;

/// Type state struct for enabled watchdog timers.
pub struct Enabled;

/// Free Watchdog Timer (FWDGT) peripheral
///
/// This watchdog timer cannot be disabled.
///
/// TODO: debug
pub struct Free<STATE> {
    fwdgt: FWDGT,
    state: STATE,
}

impl<STATE> Free<STATE> {
    /// Wrap the watchdog
    pub fn new(fwdgt: FWDGT) -> Free<Disabled> {
        Free { fwdgt, state: Disabled }
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

impl<STATE> Free<STATE> {
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

impl Free<Enabled> {
    // We may set another period when watchdog is enabled
    pub fn set_period(&mut self, period: impl Into<MicroSeconds>) {
        let (psc, rld) = Self::calc_psc_rld(period.into().0);
        while self.rud_or_pud_updating() {}
        riscv::interrupt::free(|_| {
            self.access_protected(|fwdgt| {
                // note(unsafe): valid values ensured by calc_psc_rld
                fwdgt.psc.modify(|_, w| unsafe { w.psc().bits(psc) });
                fwdgt.rld.modify(|_, w| unsafe { w.rld().bits(rld) });
            });
            // note(unsafe): write valid command constant defined in the Manual
            self.fwdgt
                .ctl
                .write(|w| unsafe { w.cmd().bits(FWDGT_CMD_RELOAD) });
        });
    }
}

impl Enable for Free<Disabled> {
    type Error = Infallible;
    type Time = MicroSeconds;
    type Target = Free<Enabled>;

    fn try_start<T>(self, period: T) -> Result<Free<Enabled>, Self::Error>
    where
        T: Into<Self::Time>,
    {
        // prepare configurations
        let (psc, rld) = Self::calc_psc_rld(period.into().0);
        // According to the Manual, if applications need to modify the PSC and
        // RLD registers, they should wait until PUD and RUD bit is cleared to
        // zero by hardware. (Section 13.1.4, FWDGT_RLD)
        while self.rud_or_pud_updating() {}
        // perform config and start process
        riscv::interrupt::free(|_| {
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
        // Change typestate to `Enabled`
        Ok(Free { fwdgt: self.fwdgt, state: Enabled })
    }
}

// We only implement `Watchdog` for a watchdog that is enabled.
// Application developers may not being able to `feed` an `Free<Disabled>`.
// In this case, developers would not forget to `enable` before feed,
// or we would not allow developers to feed a disabled dog.
impl Watchdog for Free<Enabled> {
    type Error = Infallible;

    fn try_feed(&mut self) -> Result<(), Self::Error> {
        // note(unsafe): write valid command constant defined in the Manual
        self.fwdgt
            .ctl
            .write(|w| unsafe { w.cmd().bits(FWDGT_CMD_RELOAD) });
        Ok(())
    }
}

/*
    GD32VF103's Free Watchdog Timer does not support disable operation.
    We keep code here for example to show that, if there is a watchdog timer
    that can be disabled, how could we write code for it.
*/

// impl Disable for Free<Enabled> {
//     type Error = Infallible;
//     type Target = Free<Disabled>;

//     fn try_disable(self) -> Result<Self::Target, Self::Error> {
//         // There should be probable DISABLE command
//         // Change typestate to `Disabled`
//         Ok(Free { fwdgt: self.fwdgt, state: Disabled })
//     }
// }

/// Window Watchdog Timer (WWDGT) peripheral
pub struct Window {
    wwdgt: WWDGT,
}
