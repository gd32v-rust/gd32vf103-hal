//! (TODO) Backup register domain

use crate::rcu::APB1;
use crate::pac::BKP;

// frequent used trait function name:
// - split
// - constrain
// - configure
// this means all the modules should share common power switch or
// unlock process. the function name should depends on how the
// modules effect each other:
// - split: modules do not inherit or depend on each other in common ways
// - constrain: modules have some level of strong inherit in power, clock, 
//   thus should be operated together to be functional
// - configure: modules have weak relation with each other but still
//   should be treated and configured together

pub trait BkpExt {
    fn split(self, apb1: &mut APB1) -> Parts;
}

impl BkpExt for BKP {
    fn split(self, apb1: &mut APB1) -> Parts {
        riscv::interrupt::free(|_| {
            // todo:
            // use apb1 to enable backup domain
        });
        Parts { 
            data_lo: DataLo { _ownership: () },
            data_hi: DataHi { _ownership: () },
            _todo: ()
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
    _ownership: ()
}

pub struct DataHi {
    _ownership: ()
}
