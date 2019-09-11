//! Timers

/// Hardware timers
pub struct Timer<TIM> {
    tim: TIM,
}

use crate::rcu;
use crate::pac::TIMER2;

impl Timer<TIMER2> {
    pub fn timer2<T>(timer2: TIMER2, timeout: T, apb1: &mut rcu::APB1) -> Self {
        unimplemented!()
    }
}
