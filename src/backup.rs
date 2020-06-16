//! (TODO) Backup register domain

use crate::pac::{bkp, BKP, PMU};
use crate::rcu::APB1;
use core::marker::PhantomData;

// todo: constrain an alternate PC13 pin?

/// Extension trait that constrains the `BKP` peripheral
pub trait BkpExt {
    /// Split the `BKP` peripheral into stand alone backup domain modules
    fn split(self, apb1: &mut APB1, pmu: &mut PMU) -> Parts;
}

impl BkpExt for BKP {
    fn split(self, apb1: &mut APB1, pmu: &mut PMU) -> Parts {
        // After chip reset, all write operation to backup domain (e.g.
        // registers and RTC) are forbidden. To enable write access to all
        // backup domain, first enable APB1EN's PMUEN for PMU clock, BKPIEN
        // for BKP clock; then enable PMU_CTL's BKPWEN bit for write access
        // to registers and RTC.
        riscv::interrupt::free(|_| {
            // 1. use apb1 to enable backup domain clock
            apb1.en()
                .modify(|_, w| w.pmuen().set_bit().bkpien().set_bit());
            // 2. use pmuctl to enbale write access
            // todo: should PMU be designed as a separate module?
            pmu.ctl.write(|w| w.bkpwen().set_bit());
        });
        Parts {
            data: Data {
                _owned_incontinuous_storage: PhantomData,
            },
            tamper: Tamper { _ownership: () },
            octl: OCTL { _ownership: () },
        }
    }
}

/// `BKP` Parts
pub struct Parts {
    /// Backup data register
    ///
    /// Constrains all `BKP_DATAx` (x in 0..=41).
    pub data: Data,
    /// Tamper event monitor
    ///
    /// Constrains `BKP_TPCTL` and `BKP_TPCS`.
    pub tamper: Tamper,
    /// RTC signal output control register
    ///
    /// Constrains `BKP_OCTL`.
    pub octl: OCTL,
}

// verified on GD32VF103C-START board; 2020-03-16
/// Backup data register
///
/// Constrains all `BKP_DATAx` registers, totally 42 * `u16` _incontinuous_
/// storages which adds up to 84 bytes. These storages may be used to save
/// user defined application data, and will not be reset after wake from
/// standby mode or power reset.
///
/// This struct is considered as an owned incontinuous storage, thus could be
/// shared with and sent between different contexts.
///
/// Ref: Section 4.1 & 4.4.1, the User Manual
pub struct Data {
    _owned_incontinuous_storage: PhantomData<[u16; 42]>,
}

impl Data {
    /// Read a 16-bit value from `BKP_DATA` backup data register.
    /// Parameter `idx` must be a valid register index (in `[0, 41]`)
    /// for there are 42 registers in total; otherwise this function panics.
    #[inline]
    pub fn read(&self, idx: usize) -> u16 {
        unsafe { *Self::get_ptr(idx) }
    }

    /// Write a 16-bit value into `BKP_DATA` backup data register.
    /// Parameter `idx` must be a valid register index (in `[0, 41]`)
    /// for there are 42 registers in total; otherwise this function panics.
    #[inline]
    pub fn write(&mut self, idx: usize, data: u16) {
        unsafe { *Self::get_ptr(idx) = data }
    }

    // address verified
    #[inline]
    fn get_ptr(idx: usize) -> *mut u16 {
        if idx <= 9 {
            // data0 ..= data9
            unsafe { (BKP::ptr() as *mut u8).add(idx * 0x04 + 0x04) as *mut u16 }
        } else if idx <= 41 {
            // data10 ..= data41
            unsafe { (BKP::ptr() as *mut u8).add((idx - 10) * 0x04 + 0x40) as *mut u16 }
        } else {
            panic!("invalid index")
        }
    }
}

/// Tamper event monitor
///
/// todo: detailed doc & module verify
pub struct Tamper {
    _ownership: (),
}

impl Tamper {
    /// Enable temper detection.
    ///
    /// After enabled the TAMPER pin is dedicated for Backup Reset function.
    /// The active level on the TAMPER pin resets all data of the BKP_DATAx
    /// registers.
    ///
    /// Ref: Section 4.4.3, the User Manual
    pub fn enable(&mut self) {
        unsafe { &*BKP::ptr() }
            .tpctl
            .modify(|_, w| w.tpen().set_bit());
    }

    /// Disable temper detection.
    ///
    /// After disabled, the TAMPER pin is free for GPIO functions.
    pub fn disable(&mut self) {
        unsafe { &*BKP::ptr() }
            .tpctl
            .modify(|_, w| w.tpen().clear_bit());
    }

    /// Set the TAMPER pin to active high. The TAMPER pin defaults to active
    /// high after reset.
    ///
    /// Ref: Section 4.4.3, the User Manual
    pub fn set_pin_active_high(&mut self) {
        unsafe { &*BKP::ptr() }
            .tpctl
            .modify(|_, w| w.tpal().clear_bit());
    }

    /// Set the TAMPER pin to active low. The TAMPER pin defaults to active
    /// high after reset.
    ///
    /// Ref: Section 4.4.3, the User Manual
    pub fn set_pin_active_low(&mut self) {
        unsafe { &*BKP::ptr() }
            .tpctl
            .modify(|_, w| w.tpal().set_bit());
    }

    /// Check the tamper event flag by reading from `TEF` register bit.
    pub fn check_event(&self) -> bool {
        unsafe { &*BKP::ptr() }.tpcs.read().tef().bit()
    }

    /// Clear the tamper interrupt flag bit by writing 1 to `TER` register bit.
    pub fn clear_event_bit(&mut self) {
        unsafe { &*BKP::ptr() }
            .tpcs
            .modify(|_, w| w.ter().set_bit());
    }

    /// Enable the tamper interrupt by setting the _Tamper interrupt enable 
    /// (TPIE)_ register bit.
    pub fn enable_interrupt(&mut self) {
        unsafe { &*BKP::ptr() }
            .tpcs
            .modify(|_, w| w.tpie().set_bit());
    }
    
    /// Disable the tamper interrupt by clearing the _Tamper interrupt enable 
    /// (TPIE)_ register bit.
    pub fn disable_interrupt(&mut self) {
        unsafe { &*BKP::ptr() }
            .tpcs
            .modify(|_, w| w.tpie().clear_bit());
    }

    /// Check the tamper interrupt flag by reading from `TIF` register bit.
    pub fn check_interrupt(&self) -> bool {
        unsafe { &*BKP::ptr() }.tpcs.read().tif().bit()
    }

    /// Clear the tamper interrupt flag bit by writing 1 to `TIR` register bit.
    pub fn clear_interrupt_bit(&mut self) {
        unsafe { &*BKP::ptr() }
            .tpcs
            .modify(|_, w| w.tir().set_bit());
    }
}

/// RTC signal output control register (BKP_OCTL)
pub struct OCTL {
    _ownership: (),
}

impl OCTL {
    // todo: use this register
    // pub(crate) fn octl(&mut self) -> &bkp::OCTL {
    //     unsafe { &(*BKP::ptr()).octl }
    // }
}
