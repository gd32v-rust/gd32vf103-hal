//! Reset and Control Unit

use crate::pac::{rcu, RCU};
use crate::time::*;
use core::num::NonZeroU32;

/// Extension trait that constrains the `RCU` peripheral
pub trait RcuExt {
    /// Constrains the `RCU` peripheral so it plays nicely with the other abstractions
    fn constrain(self) -> Rcu;
}

impl RcuExt for RCU {
    fn constrain(self) -> Rcu {
        Rcu {
            apb1: APB1 { _ownership: () },
            apb2: APB2 { _ownership: () },
            ahb: AHB { _ownership: () },
            cfg: CFG { _ownership: () },
            bdctl: BDCTL { _ownership: () },
            // rstsck: RSTSCK
            // dsv: DSV
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
    /// Constrains `AHBEN` and `AHBRST`.
    /// 
    /// Note: only `USBFS` AHB peripheral is able to be reset. (Section 5.3.11)
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
/// Constrains `AHBEN` and `AHBRST`.
/// 
/// Note: only `USBFS` AHB peripheral is able to be reset. (Section 5.3.11)
pub struct AHB {
    _ownership: (),
}

impl AHB {
    #[inline]
    pub(crate) fn en(&mut self) -> &rcu::AHBEN {
        unsafe { &(*RCU::ptr()).ahben }
    }
    // pub(crate) fn rst
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
    ahb_shr: u8, // [0, 9] -> [1, 512]
    apb1_shr: u8, // [0, 4] -> [2, 16]
    apb2_shr: u8, // [0, 4] -> [2, 16]
    usb_valid: bool,
}

impl Clocks {
    /// Returns the frequency of the system clock
    pub const fn ck_sys(&self) -> Hertz {
        self.ck_sys
    }
    
    /// Returns the frequency of the AHB clock
    pub const fn ck_ahb(&self) -> Hertz {
        Hertz(self.ck_sys.0 >> self.ahb_shr)
    }

    /// Returns the freqency of the Advanced Peripheral Bus 1 clock
    pub const fn ck_apb1(&self) -> Hertz {
        Hertz(self.ck_sys.0 >> (self.ahb_shr + self.apb1_shr))
    }

    /// Returns the freqency of the Advanced Peripheral Bus 2 clock
    pub const fn ck_apb2(&self) -> Hertz {
        Hertz(self.ck_sys.0 >> (self.ahb_shr + self.apb2_shr))
    }

    /// Returns the freqency of the CK_TIMERX clock
    pub const fn ck_timerx(&self) -> Hertz {
        // Hertz(self.ck_sys.0 >> (self.ahb_shr + self.apb2_shr 
        //     - if self.apb2_shr == 0 { 0 } else { 1 }))
        Hertz(self.ck_sys.0 >> (self.ahb_shr + self.apb2_shr 
            - [0, 1, 1, 1, 1][self.apb2_shr as usize]))
    }

    /// Returns whether the CK_USBFS clock frequency is valid for the USB peripheral
    pub const fn ck_usbfs_valid(&self) -> bool {
        self.usb_valid
    }
}

/// Strict clock configurator
/// 
/// This configurator only accepts strictly accurate value. If all available frequency 
/// values after configurated does not strictly equal to the desired value, the `freeze`
/// function panics. Users must be careful to ensure that the output frequency values 
/// can be strictly configurated into using input frequency values and internal clock 
/// frequencies.
/// 
/// If you need to get most precise frequenct possible (other than the stictly accutare
/// value only), use configurator `Precise` instead. 
/// 
/// For example if 49.60MHz and 50.20MHz are able to be configurated prefectly, input 
/// 50MHz into `Strict` would result in a panic when performing `freeze`; however input 
/// same 50MHz into `Precise` it would not panic, but would set and freeze into 
/// 50.20MHz as the frequency error is smallest.
pub struct Strict {
    hxtal: Option<NonZeroU32>,
    target_ck_sys: Option<NonZeroU32>,
    target_ck_i2s: Option<NonZeroU32>,
    target_ck_ahb: Option<NonZeroU32>,
    target_ck_apb1: Option<NonZeroU32>,
    target_ck_apb2: Option<NonZeroU32>,
}

impl Strict {
    /// Create a configurator
    pub fn new() -> Self {
        Strict {
            hxtal: None,
            target_ck_sys: None,
            target_ck_i2s: None,
            target_ck_ahb: None,
            target_ck_apb1: None,
            target_ck_apb2: None,
        }
    }

    /// Prefer use HXTAL (external oscillator) as the clock source.
    pub fn use_hxtal(mut self, freq: impl Into<Hertz>) -> Self {
        let freq_hz = freq.into().0;
        assert!(freq_hz >= 4_000_000 && freq_hz <= 32_000_000); // Figure 5.2, the Manual
        self.hxtal = NonZeroU32::new(freq_hz);
        self
    }

    /// Sets the desired frequency for the CK_SYS clock
    pub fn ck_sys(mut self, freq: impl Into<Hertz>) -> Self {
        let freq_hz = freq.into().0;
        assert!(freq_hz <= 108_000_000); // Figure 5.2, the Manual
        self.target_ck_sys = NonZeroU32::new(freq_hz);
        self
    }

    #[doc(hidden)] // todo
    /// Sets the desired frequency for the CK_I2S clock
    pub fn ck_i2s(mut self, freq: impl Into<Hertz>) -> Self {
        let freq_hz = freq.into().0;
        self.target_ck_i2s = NonZeroU32::new(freq_hz);
        self
    }

    /// Sets the desired frequency for the CK_AHB clock
    pub fn ck_ahb(mut self, freq: impl Into<Hertz>) -> Self {
        let freq_hz = freq.into().0;
        self.target_ck_ahb = NonZeroU32::new(freq_hz);
        self
    }

    /// Sets the desired frequency for the CK_APB1 clock
    pub fn ck_apb1(mut self, freq: impl Into<Hertz>) -> Self {
        let freq_hz = freq.into().0;
        self.target_ck_apb1 = NonZeroU32::new(freq_hz);
        self
    }

    /// Sets the desired frequency for the CK_APB2 clock
    pub fn ck_apb2(mut self, freq: impl Into<Hertz>) -> Self {
        let freq_hz = freq.into().0;
        self.target_ck_apb2 = NonZeroU32::new(freq_hz);
        self
    }
    
    /// Calculate and balance clock registers to configure into the given clock value. 
    /// 
    /// # Panics
    /// 
    /// If strictly accurate value of given `ck_sys` etc. is not reachable, this function 
    /// panics.
    pub fn freeze(self, cfg: &mut CFG) -> Clocks {
        const IRC8M: u32 = 8_000_000;
        let mut usb_valid = false;
        let target_ck_sys = self.target_ck_sys.map(|f| f.get()).unwrap_or(IRC8M);
        let target_ck_ahb = self.target_ck_ahb.map(|f| f.get()).unwrap_or(target_ck_sys);
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
                    panic!("invalid frequency")
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
                    panic!("invalid frequency")
                }
            };
        } // use_pll
        let (ahbpsc, ahb_shr) = {
            // 0xxx: /1; 1000: /2; 1001: /4; ... 1111: /512. (skip /32)
            let mut ahb_shr = 0; // log2(1)
            let mut ans = 0b0111u8;
            let mut target_freq = target_ck_ahb;
            while ahb_shr <= 9 { // log2(512)
                if ahb_shr != 5 && target_freq == target_ck_sys {
                    break;
                }
                target_freq *= 2;
                ahb_shr += 1;
                if ahb_shr != 5 { // log2(32)
                    ans += 1;
                }
            }
            if ans > 0b1111 {
                panic!("invalid frequency")
            }
            (ans, ahb_shr)
        };
        let calc_psc_apbx = |target_ck_apbx: u32| {
            let mut ans = 0b011u8;
            let mut target_freq = target_ck_apbx;
            while ans <= 0b111 {
                if target_freq == target_ck_ahb {
                    break;
                }
                target_freq *= 2;
                ans += 1;
            }
            if ans > 0b111 {
                panic!("invalid frequency")
            };
            ans
        };
        let target_ck_apb1 = self.target_ck_apb1.map(|f| f.get()).unwrap_or(target_ck_ahb / 2);
        let apb1psc = calc_psc_apbx(target_ck_apb1);
        let target_ck_apb2 = self.target_ck_apb2.map(|f| f.get()).unwrap_or(target_ck_ahb);
        let apb2psc = calc_psc_apbx(target_ck_apb2);
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
            cfg.cfg0().modify(|_, w| unsafe { w
                // Configure PLL input selector
                .pllsel().bit(use_pll)
                // Configure PLL multiplier
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
            // adjust USB prescaler
            cfg.cfg0().modify(|_, w| unsafe { w.usbfspsc().bits(usbfspsc) });
        }
        // 6. adjust AHB and APB clocks
        // todo: verify if three switches in one modify is okay
        cfg.cfg0().modify(|_, w| unsafe { w
            .ahbpsc().bits(ahbpsc)
            .apb1psc().bits(apb1psc)
            .apb2psc().bits(apb2psc)
        });
        Clocks {
            ck_sys: Hertz(target_ck_sys),
            ahb_shr,
            apb1_shr: apb1psc - 0b011,
            apb2_shr: apb2psc - 0b011,
            usb_valid
        }
    }
}

/// (TODO) Precise clock configurator
/// 
/// This configurator would offer config to get the most precise output value possible 
/// using input values. Errors between desired and actual output would be acceptible; 
/// it would be minimized by the algorithm, thus the output would be as precise as 
/// possible. 
pub struct Precise {
    _todo: ()
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
