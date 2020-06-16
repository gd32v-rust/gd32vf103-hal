//! Timers
use crate::pac::TIMER6;
use crate::rcu::{Clocks, APB1};
use crate::unit::Hertz;
use embedded_hal::blocking::delay::DelayMs;
use embedded_hal::timer::CountDown;
use core::convert::Infallible;

// I'd prefer using Timer<TIMERx> for convenience
/// Timer object
pub struct Timer<TIMER> {
    timer: TIMER,
    clock_scaler: u16,
    clock_frequency: Hertz,
}

impl Timer<TIMER6> {
    /// Initialize the timer.
    ///
    /// An enable and reset procedure is procceed to peripheral to clean its state.
    pub fn timer6(timer: TIMER6, clock: Clocks, apb1: &mut APB1) -> Self {
        riscv::interrupt::free(|_| {
            apb1.en().modify(|_, w| w.timer6en().set_bit());
            apb1.rst().write(|w| w.timer6rst().set_bit());
            apb1.rst().write(|w| w.timer6rst().clear_bit());
        });
        Timer {
            timer,
            clock_scaler: 1000,
            clock_frequency: clock.ck_apb1(),
        }
    }
}

impl<TIMER> Timer<TIMER> {
    // in future designs we do not stop timer in this function
    // but prefer using Timer<TIMER>::start(self, ...) -> SomeTimer
    // when SomeTimer should be stopped, it has function returns timer back
    // as SomeTimer::stop(self) -> Timer<TIMER>.
    /// Release the timer, return its ownership.
    pub fn release(self) -> TIMER {
        self.timer
    }
}

impl<T: Into<u32>> DelayMs<T> for Timer<TIMER6> {
    type Error = Infallible;
    fn try_delay_ms(&mut self, ms: T) -> Result<(), Self::Error> {
        let count = (ms.into() * self.clock_frequency.0) / (self.clock_scaler as u32 * 1000);
        if count > u16::max_value() as u32 {
            panic!("can not delay that long");
        }
        self.try_start(count as u16).ok();
        nb::block!(self.try_wait()).ok();
        Ok(())
    }
}

impl CountDown for Timer<TIMER6> {
    type Error = Infallible;
    type Time = u16;

    fn try_start<T>(&mut self, count: T) -> Result<(), Self::Error>
    where
        T: Into<Self::Time>,
    {
        let c = count.into();
        riscv::interrupt::free(|_| {
            self.timer
                .psc
                .write(|w| unsafe { w.psc().bits(self.clock_scaler) });
            self.timer.intf.write(|w| w.upif().clear_bit());
            self.timer.swevg.write(|w| w.upg().set_bit());
            self.timer.intf.write(|w| w.upif().clear_bit());
            self.timer.car.modify(|_, w| unsafe { w.carl().bits(c) });
            self.timer.ctl0.modify(|_, w| w.cen().set_bit());
        });
        Ok(())
    }

    // this signature changes in a future version, so we don'ot need the void crate.
    // --- 
    // changed to Self::Error, removing void (luojia65)
    fn try_wait(&mut self) -> nb::Result<(), Self::Error> {
        let flag = self.timer.intf.read().upif().bit_is_set();
        if flag {
            Ok(())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }
}

// impl Periodic for Timer<TIMER2> {}
