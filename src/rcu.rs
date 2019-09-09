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
            apb2: APB2 { _ownership: () }
            // ...
        }
    }
}

/// Constrained RCU peripheral
pub struct Rcu {
    /// Advanced Pheripheral Bus 2 (APB2) registers
    pub apb2: APB2,
    // ...
}

/// Advanced Pheripheral Bus 2 (APB2) registers
pub struct APB2 {
    _ownership: ()
}

impl APB2 {
    pub(crate) fn en(&mut self) -> &rcu::APB2EN {
        unsafe { &(*RCU::ptr()).apb2en }
    }

    pub(crate) fn rst(&mut self) -> &rcu::APB2RST {
        unsafe { &(*RCU::ptr()).apb2rst }
    }
}
