//! Timers
use crate::time::Hertz;
use embedded_hal::timer::{CountDown, Periodic};
use void::Void;

/// Hardware timers
pub struct Timer<TIM> {
    clocks: rcu::Clocks,
    timer: TIM,
}

use crate::rcu;
use crate::pac::TIMER2;

impl Timer<TIMER2> {
    pub fn timer2<T>(timer2: TIMER2, timeout: T, clocks: rcu::Clocks, apb1: &mut rcu::APB1) -> Self 
    where 
        T: Into<Hertz>
    {
        apb1.en().write(|w| w.timer2en().set_bit());
        apb1.rst().write(|w| w.timer2rst().set_bit());
        apb1.rst().write(|w| w.timer2rst().clear_bit());
        let mut timer = Timer { clocks, timer: timer2 };
        timer.start(timeout);
        timer
    }
}

impl CountDown for Timer<TIMER2> {
    type Time = Hertz;

    fn start<T>(&mut self, count: T)
    where
        T: Into<Self::Time>
    {
        unimplemented!()
    }
        
    fn wait(&mut self) -> nb::Result<(), Void> {
        unimplemented!()
    }
}

impl Periodic for Timer<TIMER2> {}
