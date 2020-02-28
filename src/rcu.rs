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
            // clocks: Clocks {
            //     // todo: check code here
            //     ck_sys: 8.mhz().into(),
            //     ck_ahb: 8.mhz().into(),
            //     ck_apb1: 8.mhz().into(),
            //     ck_apb2: 8.mhz().into(),
            // },
            cfg: CFG { _ownership: () },
            bdctl: BDCTL { _ownership: () },
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
    /// Constrains `AHBEN`.
    pub ahb: AHB,
    /// Clock configuration registers
    ///
    /// Constrains `CFG0` and `CFG1` and `CTL0`
    pub cfg: CFG,
    // // todo: remove
    // pub clocks: Clocks,
    /// Backup domain control register
    ///
    /// Constrains `BDCTL`.
    pub bdctl: BDCTL,
    // ...
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

/// Clock configuration registers
///
/// Constrains `CFG0` and `CFG1` and `CTL0`
pub struct CFG {
    _ownership: (),
}

impl CFG {
    #[inline]
    pub(crate) fn cfg0(&mut self) -> &rcu::CFG0 {
        unsafe { &(*RCU::ptr()).cfg0 }
    }
    #[inline]
    pub(crate) fn cfg1(&mut self) -> &rcu::CFG1 {
        unsafe { &(*RCU::ptr()).cfg1 }
    }
    #[inline]
    pub(crate) fn ctl(&mut self) -> &rcu::CTL {
        unsafe { &(*RCU::ptr()).ctl }
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
    usb_valid: bool,
}

impl Clocks {
    /// Returns the frequency of the system clock
    pub fn ck_sys(&self) -> Hertz {
        return self.ck_sys;
    }
    
    #[doc(hidden)]
    /// Returns the frequency of the AHB clock
    pub fn ck_ahb(&self) -> Hertz {
        return self.ck_sys; // todo!!!!!!!!
    }

    #[doc(hidden)]
    /// Returns the freqency of the Advanced Peripheral Bus 1 clock
    pub fn ck_apb1(&self) -> Hertz {
        return self.ck_sys; // todo!!!!!!!!
    }

    #[doc(hidden)]
    /// Returns the freqency of the Advanced Peripheral Bus 2 clock
    pub fn ck_apb2(&self) -> Hertz {
        return self.ck_sys; // todo!!!!!!!!
    }
}

use core::num::NonZeroU32;

pub struct Adjust {
    hxtal: Option<NonZeroU32>,
    target_ck_sys: Option<NonZeroU32>,
    target_ck_i2s: Option<NonZeroU32>,
}

impl Adjust {
    pub fn new() -> Self {
        Adjust {
            hxtal: None,
            target_ck_sys: None,
            target_ck_i2s: None,
        }
    }

    pub fn use_hxtal(mut self, freq: impl Into<Hertz>) -> Self {
        let freq_hz = freq.into().0;
        assert!(freq_hz >= 4_000_000 && freq_hz <= 32_000_000); // Figure 5.2, the Manual
        self.hxtal = NonZeroU32::new(freq_hz);
        self
    }

    pub fn ck_sys(mut self, freq: impl Into<Hertz>) -> Self {
        let freq_hz = freq.into().0;
        assert!(freq_hz <= 108_000_000); // Figure 5.2, the Manual
        self.target_ck_sys = NonZeroU32::new(freq_hz);
        self
    }

    pub fn ck_i2s(mut self, freq: impl Into<Hertz>) -> Self {
        let freq_hz = freq.into().0;
        self.target_ck_i2s = NonZeroU32::new(freq_hz);
        self
    }
    
    /// Balance clock registers to get the most accurate clock frequency possible
    pub fn freeze(self, cfg: &mut CFG) -> Clocks {
        const IRC8M: u32 = 8_000_000;
        let mut usb_valid = false;
        let target_ck_sys = self.target_ck_sys.map(|f| f.get()).unwrap_or(IRC8M);
        let (scs, use_pll) = match (self.hxtal, target_ck_sys) {
            (Some(hxtal), sys) if hxtal.get() == sys => (0b01, false),
            (None, sys) if IRC8M == sys => (0b00, false),
            _ => (0b10, true),
        };
        let mut pllmf = 0;
        if use_pll {
            pllmf = if let Some(hxtal) = self.hxtal {
                let hxtal = hxtal.get();
                let calc_pllmf = || {
                    for div in 1..=16 {
                        if target_ck_sys == hxtal * 13 / 2 { // 6.5
                            return 0b01101;
                        }
                        let mul = target_ck_sys / (div * hxtal);
                        if mul < 2 || mul > 32 || mul == 15 {
                            continue;
                        }
                        let out_ck_sys = hxtal * mul / div;
                        if out_ck_sys == target_ck_sys {
                            return if mul <= 14 { mul - 2 } else { mul - 1 };
                        }
                    };
                    panic!("invalid system frequency")
                };
                calc_pllmf() as u8
            } else { // does not use HXTAL
                let pllsel0_src = IRC8M / 2;
                let mul_pllmf = target_ck_sys / pllsel0_src;
                // pllmf: 00000 => 2, 00001 => 3, ..., 01100 => 14; 01101 => 6.5;
                // 01111 => 16, 10000 => 17, ..., 11111 => 32; 
                // may use 6.5 here
                let mul_pllmf = u32::max(2, u32::min(mul_pllmf, 32));
                if target_ck_sys == mul_pllmf * pllsel0_src { 
                    // use 2..=14 | 16..=32
                    (if mul_pllmf <= 14 { mul_pllmf - 2 } else { mul_pllmf - 1 }) as u8
                } else if target_ck_sys == pllsel0_src * 13 / 2 {
                    0b01101 as u8// use special 6.5 multiplier
                } else {
                    panic!("invalid system frequency")
                }
            };
        } // use_pll
        // 1. enable IRC8M 
        if self.hxtal.is_none() {
            // enable IRC8M
            cfg.ctl().modify(|_, w| w.irc8men().set_bit()); 
            // Wait for oscillator to stabilize
            while cfg.ctl().read().irc8mstb().bit_is_clear() {} 
        }
        // 2. enable hxtal
        if let Some(_) = self.hxtal {
            // enable hxtal
            cfg.ctl().modify(|_, w| w.hxtalen().set_bit());
            // wait before stable
            while cfg.ctl().read().hxtalstb().bit_is_clear() {}
        }
        // 3. enable pll
        if use_pll {
            // Configure PLL input selector
            cfg.cfg0().modify(|_, w| w.pllsel().bit(use_pll));
            // Configure PLL multiplier
            cfg.cfg0().modify(|_, w| unsafe { w
                .pllmf_4().bit(pllmf & 0x10 != 0)
                .pllmf_3_0().bits(pllmf & 0xf)
            });
            // Enable PLL
            cfg.ctl().modify(|_, w| w.pllen().set_bit());
            // Wait for PLL to stabilize
            while cfg.ctl().read().pllstb().bit_is_clear() {}
        } else {
            // or we disable PLL
            cfg.ctl().modify(|_, w| w.pllen().clear_bit());
        }
        // 4. check SCS selector
        cfg.cfg0().modify(|_, w| unsafe { w.scs().bits(scs) });
        // 5. check and enable usb
        if let Some(_) = self.hxtal {
            let ck_pll = target_ck_sys;
            let (usb_freq_okay, usbfspsc) = match ck_pll {
                48_000_000 => (true, 0b01), // ck_pll / 1
                72_000_000 => (true, 0b00), // ck_pll / 1.5
                96_000_000 => (true, 0b11), // ck_pll / 2
                // 0b10 (ck_pll / 2.5) is impossible in this algorithm
                _ => (false, 0),
            };
            usb_valid = usb_freq_okay;

            cfg.cfg0().modify(|_, w| unsafe { w.usbfspsc().bits(usbfspsc) });
        }
        Clocks {
            ck_sys: Hertz(target_ck_sys),
            usb_valid
        }
    }
}

/// Opaque `BDCTL` register
pub struct BDCTL {
    _ownership: (),
}

impl BDCTL {
    #[inline]
    pub(crate) fn bdctl(&mut self) -> &rcu::BDCTL {
        unsafe { &(*RCU::ptr()).bdctl }
    }
}
