//! (TODO) Alternate Function I/O

use crate::pac::AFIO;
use crate::rcu::APB2;

pub trait AfioExt {
    fn split(self, apb2: &mut APB2) -> Parts;
}

impl AfioExt for AFIO {
    fn split(self, apb2: &mut APB2) -> Parts {
        riscv::interrupt::free(|_| {
            apb2.en().modify(|_, w| w.afen().set_bit());
            apb2.rst().modify(|_, w| w.afrst().set_bit());
            apb2.rst().modify(|_, w| w.afrst().clear_bit());
        });
        Parts {
            ec: EC { _ownership: () },
            pcf0: PCF0 { _ownership: () },
            pcf1: PCF1 { _ownership: () },
            _todo: (),
        }
    }
}

pub struct Parts {
    pub ec: EC, // pub ec: EventOutCtrl
    // pub extiss: ExtiSelect
    pub pcf0: PCF0,
    pub pcf1: PCF1,
    _todo: (),
}

pub struct EC {
    _ownership: (),
}

// todo: impl EC

/// Opaque PCF0 register
pub struct PCF0 {
    _ownership: (),
}

/// Opaque PCF1 register
pub struct PCF1 {
    _ownership: (),
}
