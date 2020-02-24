//! (TODO) Backup register domain

use crate::pac::{BKP, PMU};
use crate::rcu::APB1;
use core::marker::PhantomData;

pub trait BkpExt {
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
            apb1.en().write(|w| w.pmuen().set_bit().bkpien().set_bit());
            // 2. use pmuctl to enbale write access
            // todo: should PMU be designed as a separate module?
            pmu.ctl.write(|w| w.bkpwen().set_bit());
        });
        Parts {
            data: Data { _owned_ptr: PhantomData },
            _todo: (),
        }
    }
}

pub struct Parts {
    pub data: Data,
    _todo: (),
    // pub octl: OCTL,
    // pub tpctl: TPCTL,
    // pub tpcs: TPCS,
}

// BKP + 0x04 to 0x28; BKP + 0x40 to 0xBC
pub struct Data {
    _owned_ptr: PhantomData<*const u16>,
}

impl Data {
    /// Read a 16-bit value from `BKP_DATA` backup data register. 
    /// Parameter `idx` must be a valid register index (in `[0, 41]`)
    /// for there are 42 registers in total; otherwise this function panics.
    pub fn read(&mut self, idx: usize) -> u16 {
        unsafe { *Self::get_ptr(idx) }
    }

    /// Write a 16-bit value into `BKP_DATA` backup data register. 
    /// Parameter `idx` must be a valid register index (in `[0, 41]`)
    /// for there are 42 registers in total; otherwise this function panics.
    pub fn write(&mut self, idx: usize, data: u16) {
        unsafe { *Self::get_ptr(idx) = data }
    }

    // todo: verify this function
    fn get_ptr(idx: usize) -> *mut u16 {
        if idx <= 9 { // data0 ..= data9
            unsafe { (BKP::ptr() as *mut u16).add(idx + 2) }
        } else if idx <= 41 {  // data10 ..= data41
            unsafe { (BKP::ptr() as *mut u16).add(idx + 8) }
        } else {
            panic!("invalid index")
        }
    }
}
