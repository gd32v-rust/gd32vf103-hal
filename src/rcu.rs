//! Reset and Control Unit

use crate::pac::{rcu, RCU};
use crate::time::*;

/// Extension trait that constrains the `RCU` peripheral
pub trait RcuExt {
    /// Constrains the `RCU` peripheral so it plays nicely with the other abstractions
    fn constrain(self) -> Rcu;
}

impl RcuExt for RCU {
    fn constrain(self) -> Rcu {
        Rcu {
            // ahb: AHB { _ownership: () },
            apb1: APB1 { _ownership: () },
            apb2: APB2 { _ownership: () },
            ahb: AHB { _ownership: () },
            clocks: Clocks {
                ck_sys: 8.mhz().into(),
                ck_ahb: 8.mhz().into(),
                ck_apb1: 8.mhz().into(),
                ck_apb2: 8.mhz().into(),
            },
            // ...
            _todo: (),
        }
    }
}

/// Constrained RCU peripheral
pub struct Rcu {
    // pub ahb: AHB,
    /// Advanced Pheripheral Bus 1 (APB1) registers
    ///
    /// Constrains `APB1EN` and `ABR1RST`.
    pub apb1: APB1,
    /// Advanced Pheripheral Bus 2 (APB2) registers
    ///
    /// Constrains `APB2EN` and `ABR2RST`.
    pub apb2: APB2,
    /// AHB registers
    ///
    /// Constrains `AHBEN`
    pub ahb: AHB,
    /// Clock configuration registers
    ///
    /// Constrains `CFG0` and `CFG1` and `CTL0`
    pub clocks: Clocks,
    // ...
    #[doc(hidden)]
    _todo: (),
}

/// AMBA High-performance Bus (AHB) registers
///
/// Constrains `AHBEN`.
pub struct AHB {
    _ownership: (),
}

impl AHB {
    #[inline]
    pub(crate) fn en(&mut self) -> &rcu::AHBEN {
        unsafe { &(*RCU::ptr()).ahben }
    }
}

/// Advanced Pheripheral Bus 1 (APB1) registers
///
/// Constrains `APB1EN` and `ABR1RST`.
pub struct APB1 {
    _ownership: (),
}

impl APB1 {
    #[inline]
    pub(crate) fn en(&mut self) -> &rcu::APB1EN {
        unsafe { &(*RCU::ptr()).apb1en }
    }

    #[inline]
    pub(crate) fn rst(&mut self) -> &rcu::APB1RST {
        unsafe { &(*RCU::ptr()).apb1rst }
    }
}

/// Advanced Pheripheral Bus 2 (APB2) registers
///
/// Constrains `APB2EN` and `ABR2RST`.
pub struct APB2 {
    _ownership: (),
}

impl APB2 {
    #[inline]
    pub(crate) fn en(&mut self) -> &rcu::APB2EN {
        unsafe { &(*RCU::ptr()).apb2en }
    }

    #[inline]
    pub(crate) fn rst(&mut self) -> &rcu::APB2RST {
        unsafe { &(*RCU::ptr()).apb2rst }
    }
}

//TODO read the registers and store in struct, rather than hardcode defaults
//TODO actually freeze these somehow...
/// Frozen clock freqencies
///
/// The existence of this value indicates that the core clock
/// configuration can no longer be changed
#[derive(Clone, Copy)]
pub struct Clocks {
    ck_sys: Hertz,
    ck_ahb: Hertz,
    ck_apb1: Hertz,
    ck_apb2: Hertz,
}

impl Clocks {
    /// Returns the frequency of the system clock
    pub fn ck_sys(&self) -> Hertz {
        return self.ck_sys;
    }

    /// Returns the frequency of the system clock (alias for ck_sys)
    pub fn sysclk(&self) -> Hertz {
        return self.ck_sys;
    }

    /// Returns the frequency of the AHB clock
    pub fn ck_ahb(&self) -> Hertz {
        return self.ck_ahb;
    }

    /// Returns the freqency of the Advanced Peripheral Bus 1 clock
    pub fn ck_apb1(&self) -> Hertz {
        return self.ck_apb1;
    }

    /// Returns the freqency of the Advanced Peripheral Bus 2 clock
    pub fn ck_apb2(&self) -> Hertz {
        return self.ck_apb2;
    }

    /// Returns the freqency of the PCLK1 clock used for apb1 peripherals
    pub fn pclk1(&self) -> Hertz {
        return self.ck_apb1;
    }

    /// Returns the freqency of the PCLK2 clock used for apb2 peripherals
    pub fn pclk2(&self) -> Hertz {
        return self.ck_apb2;
    }
}
