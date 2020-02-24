//! (TODO) Backup register domain

use crate::pac::BKP;
use crate::rcu::APB1;

pub trait BkpExt {
    fn split(self, apb1: &mut APB1) -> Parts;
}

impl BkpExt for BKP {
    fn split(self, apb1: &mut APB1) -> Parts {
        // After chip reset, all write operation to backup domain (e.g. 
        // registers and RTC) are forbidden. To enable write access to 
        // backup domain, first enable APB1EN's PMUEN for power and BKPIEN 
        // for clock; then enable PMU_CTL's BKPWEN bit for write access
        // to registers and RTC.
        riscv::interrupt::free(|_| {
            // todo:
            // 1. use apb1 to enable backup domain (power & clock)
            // 2. use pmuctl to enbale write access
        });
        Parts {
            data_lo: DataLo { _ownership: () },
            data_hi: DataHi { _ownership: () },
            _todo: (),
        }
    }
}

pub struct Parts {
    pub data_lo: DataLo,
    pub data_hi: DataHi,
    _todo: (),
    // pub octl: OCTL,
    // pub tpctl: TPCTL,
    // pub tpcs: TPCS,
}

pub struct DataLo {
    _ownership: (),
}

pub struct DataHi {
    _ownership: (),
}
