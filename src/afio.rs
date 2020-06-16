//! (TODO) Alternate Function I/O

use crate::pac::{afio, AFIO};
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

impl PCF0 {
    #[inline]
    pub(crate) fn pcf0(&mut self) -> &afio::PCF0 {
        unsafe { &(*AFIO::ptr()).pcf0 }
    }
}

/// Opaque PCF1 register
pub struct PCF1 {
    _ownership: (),
}
