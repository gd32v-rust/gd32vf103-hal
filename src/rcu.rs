//! Reset and Control Unit

use crate::pac::{rcu, RCU};

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
    // ...
    #[doc(hidden)]
    _todo: (),
}

// pub struct AHB {
//     _ownership: ()
// }

/// Advanced Pheripheral Bus 1 (APB1) registers
/// 
/// Constrains `APB1EN` and `ABR1RST`.
pub struct APB1 {
    _ownership: ()
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
    _ownership: ()
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


pub struct Clocks {

}

