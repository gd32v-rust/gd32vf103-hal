//! General Purpose Input / Output

use crate::rcu::APB2;
use core::marker::PhantomData;

/// Extension trait to split a GPIO peripheral into independent pins and registers
pub trait GpioExt {
    /// The type to split the GPIO into
    type Parts;

    /// Splits the GPIO block into independent pins and registers
    fn split(self, apb2: &mut APB2) -> Self::Parts;
}

/// Analog input mode (type state)
pub struct Analog;

/// Input mode (type state)
pub struct Input<MODE> {
    _typestate_mode: PhantomData<MODE>,
}

/// Floating input mode (type state)
pub struct Floating;

/// Pulled down input mode (type state)
pub struct PullDown;

/// Pulled up input mode (type state)
pub struct PullUp;

/// Output mode (type state)
pub struct Output<MODE> {
    _typestate_mode: PhantomData<MODE>,
}

/// Alternate mode (type state)
pub struct Alternate<MODE> {
    _typestate_mode: PhantomData<MODE>,
}

/// Push-pull output or alternate (type state)
pub struct PushPull;

/// Open drain output or alternate (type state)
pub struct OpenDrain;

/// Marker trait for active states
pub trait Active {}

impl Active for Analog {}

impl<MODE> Active for Input<MODE> {}

impl<MODE> Active for Output<MODE> {}

impl<MODE> Active for Alternate<MODE> {}

/// Output speed up to 10 MHz (type param)
pub struct UpTo10MHz;

/// Output speed up to 2 MHz (type param)
pub struct UpTo2MHz;

/// Output speed up to 50 MHz (type param)
pub struct UpTo50MHz;

/// Marker trait for valid output speed
pub trait Speed {
    // The MD\[1:0\] bits this speed is represented into
    #[doc(hidden)]
    const MD_BITS: u32;
}

impl Speed for UpTo50MHz {
    const MD_BITS: u32 = 0b11;
}

impl Speed for UpTo10MHz {
    const MD_BITS: u32 = 0b01;
}

impl Speed for UpTo2MHz {
    const MD_BITS: u32 = 0b10;
}

/// Wraps a pin if this pin is locked
pub struct Locked<T>(T);

// implement all digital input/output traits for locked pins
mod impl_for_locked {
    use super::Locked;
    use embedded_hal::digital::{InputPin, OutputPin, StatefulOutputPin, ToggleableOutputPin};

    impl<T> OutputPin for Locked<T>
    where
        T: OutputPin,
    {
        type Error = T::Error;

        fn try_set_low(&mut self) -> Result<(), Self::Error> {
            self.0.try_set_low()
        }

        fn try_set_high(&mut self) -> Result<(), Self::Error> {
            self.0.try_set_high()
        }
    }

    impl<T> StatefulOutputPin for Locked<T>
    where
        T: StatefulOutputPin,
    {
        fn try_is_set_high(&self) -> Result<bool, Self::Error> {
            self.0.try_is_set_high()
        }

        fn try_is_set_low(&self) -> Result<bool, Self::Error> {
            self.0.try_is_set_low()
        }
    }

    impl<T> ToggleableOutputPin for Locked<T>
    where
        T: ToggleableOutputPin,
    {
        type Error = T::Error;

        fn try_toggle(&mut self) -> Result<(), Self::Error> {
            self.0.try_toggle()
        }
    }

    impl<T> InputPin for Locked<T>
    where
        T: InputPin,
    {
        type Error = T::Error;

        fn try_is_high(&self) -> Result<bool, Self::Error> {
            self.0.try_is_high()
        }

        fn try_is_low(&self) -> Result<bool, Self::Error> {
            self.0.try_is_low()
        }
    }
}

/// Useful unlock methods for lock marked pins
///
/// _Note: We design this trait other than giving all pins an `unlock` method
/// because if we do so, the rust doc of struct `Lock` could be full of `unlock`
/// methods (dozens of them) with full documents for each `unlock` functions, which
/// could be confusing for users and costs much time to read and build. If any
/// questions, please fire an issue to let us know._
pub trait Unlock {
    /// The lock controller register block, typically a `LOCK` struct with temporary
    /// variant bits for each pins.
    type Lock;

    /// Unlock output, typically a `PXi` struct with mode typestate.
    type Output;

    /// Mark the locked pin as unlocked to allow configurations of pin mode.
    ///
    /// Typically this method uses a mutable borrow of `LOCK` struct of the gpio port.
    /// This function is not an actually unlock; it only clears the corresponding
    /// bit in a temporary variant in `LOCK`. To actually perform and freeze the lock,
    /// use `freeze`; see function `lock` for details.
    ///
    /// The caller of this method must obtain a mutable reference of `LOCK` struct;
    /// if you have called the `freeze` method of that `LOCK` struct, the actually lock
    /// operation would perform and lock state of all pins would be no longer possible to
    /// change - see its documentation for details.
    fn unlock(self, lock: &mut Self::Lock) -> Self::Output;
}

trait PinIndex {
    const OP_LK_INDEX: usize;

    const CTL_MD_INDEX: usize;
}

macro_rules! impl_gpio {
    ($GPIOX:ident,$gpiox:ident,$gpioy:ident,$en:ident,$rst: ident,$PXx:ident, [
        $($PXi:ident:($pxi:ident,$i:expr,$MODE:ty,$CTL:ident,$ctl:ident),)+
    ]) => {
/// GPIO port
pub mod $gpiox {
    use super::{
        Active, Alternate, Analog, Floating, GpioExt, Input, OpenDrain, Output,
        PinIndex, PullDown, PullUp, PushPull, Speed, UpTo50MHz, Locked, Unlock
    };
    use crate::pac::{$gpioy, $GPIOX};
    use crate::rcu::APB2;
    use crate::atomic::{atomic_set_bit, atomic_toggle_bit};
    use core::convert::Infallible;
    use core::marker::PhantomData;
    use core::sync::atomic::AtomicU32;
    use embedded_hal::digital::{InputPin, OutputPin, StatefulOutputPin, ToggleableOutputPin};

    /// GPIO parts
    pub struct Parts {
        /// Opaque CTL0 register
        pub ctl0: CTL0,
        /// Opaque CTL1 register
        pub ctl1: CTL1,
        /// Opaque OCTL register
        pub octl: OCTL,
        /// Opaque LOCK register
        pub lock: LOCK,
        $(
            /// Pin
            pub $pxi: $PXi<$MODE>,
        )+
        #[doc(hidden)]
        _extensible: (),
    }

    impl GpioExt for $GPIOX {
        type Parts = Parts;

        fn split(self, apb2: &mut APB2) -> Self::Parts {
            riscv::interrupt::free(|_| {
                apb2.en().modify(|_,w| w.$en().set_bit());
                apb2.rst().write(|w| w.$rst().set_bit());
                apb2.rst().write(|w| w.$rst().clear_bit());
            });
            Parts {
                ctl0: CTL0 { _ownership: () },
                ctl1: CTL1 { _ownership: () },
                octl: OCTL { _ownership: () },
                lock: LOCK {
                    tmp_bits: unsafe { &(*$GPIOX::ptr()).lock }.read().bits(),
                    _ownership: ()
                },
                $(
                    $pxi: $PXi {
                        _typestate_mode: PhantomData,
                    },
                )+
                _extensible: (),
            }
        }
    }

    /// Opaque CTL0 register
    pub struct CTL0 {
        _ownership: (),
    }

    impl CTL0 {
        pub(crate) fn ctl0(&mut self) -> &$gpioy::CTL0 {
            unsafe { &(*$GPIOX::ptr()).ctl0 }
        }
    }

    /// Opaque CTL1 register
    pub struct CTL1 {
        _ownership: (),
    }

    impl CTL1 {
        pub(crate) fn ctl1(&mut self) -> &$gpioy::CTL1 {
            unsafe { &(*$GPIOX::ptr()).ctl1 }
        }
    }

    /// Opaque OCTL register
    pub struct OCTL {
        _ownership: (),
    }

    impl OCTL {
        pub(crate) fn octl(&mut self) -> &$gpioy::OCTL {
            unsafe { &(*$GPIOX::ptr()).octl }
        }
    }

    /// Opaque LOCK register
    pub struct LOCK {
        tmp_bits: u32,
        _ownership: (),
    }

    impl LOCK {
        pub(crate) fn lock(&mut self) -> &$gpioy::LOCK {
            unsafe { &(*$GPIOX::ptr()).lock }
        }

        /// Freeze pin modes of this GPIO port to forbid furtuer modifications
        /// on pin modes.
        ///
        /// By the time this function succeeds to execute, the program cannot
        /// change CTL0 and CTL1 registers of this port anymore before chip reset.
        /// To perform the real lock process, this operation writes LKy and LKK
        /// registers in a special way, and this configuration cannot be undone
        /// so it consumes the LOCK register struct `self`.
        ///
        /// Instead of returning the LOCK back, this function panics on lock failure.
        /// That's because we consider all lock failures comes from mistakes in
        /// underlying libraries or chip design which may be not proper for users
        /// to handle by themselves. If this design results in mistake, please
        /// fire an issue to let us know.
        pub fn freeze(mut self) {
            let tmp = self.tmp_bits;
            let a = tmp | 0x00010000;
            // write in special ways to lock the register
            let success = riscv::interrupt::free(|_| {
                self.lock().write(|w| unsafe { w.bits(a) });
                self.lock().write(|w| unsafe { w.bits(tmp) });
                self.lock().write(|w| unsafe { w.bits(a) });
                let ans1 = self.lock().read().bits();
                let ans2 = self.lock().read().bits();
                ans1 == 0 && ans2 & 0x00010000 != 0
            });
            // if success, this function returns
            if !success {
                panic!("the LOCK freeze process won't succeed")
            }
        }
    }

    /// Partially erased pin
    pub struct $PXx<MODE> {
        i: u8,
        _typestate_mode: PhantomData<MODE>,
    }

    impl<MODE> InputPin for $PXx<Input<MODE>> {
        type Error = Infallible;

        fn try_is_high(&self) -> Result<bool, Self::Error> {
            let ans =
                (unsafe { &(*$GPIOX::ptr()).istat }.read().bits() & (1 << self.i)) != 0;
            Ok(ans)
        }

        fn try_is_low(&self) -> Result<bool, Self::Error> {
            Ok(!self.try_is_high()?)
        }
    }

    impl<MODE> OutputPin for $PXx<Output<MODE>> {
        type Error = Infallible;

        fn try_set_high(&mut self) -> Result<(), Self::Error> {
            unsafe { &(*$GPIOX::ptr()).bop }.write(|w| unsafe { w.bits(1 << self.i) });
            Ok(())
        }

        fn try_set_low(&mut self) -> Result<(), Self::Error> {
            unsafe { &(*$GPIOX::ptr()).bc }.write(|w| unsafe { w.bits(1 << self.i) });
            Ok(())
        }
    }

    impl<MODE> StatefulOutputPin for $PXx<Output<MODE>> {
        fn try_is_set_high(&self) -> Result<bool, Self::Error> {
            let ans =
                (unsafe { &(*$GPIOX::ptr()).octl }.read().bits() & (1 << self.i)) != 0;
            Ok(ans)
        }

        fn try_is_set_low(&self) -> Result<bool, Self::Error> {
            Ok(!self.try_is_set_high()?)
        }
    }

    impl<MODE> ToggleableOutputPin for $PXx<Output<MODE>> {
        type Error = Infallible;

        fn try_toggle(&mut self) -> Result<(), Self::Error> {
            let r: &AtomicU32 = unsafe { &*(&(*$GPIOX::ptr()).octl as *const _ as *const _) };
            atomic_toggle_bit(r, self.i as usize);
            Ok(())
        }
    }

    impl InputPin for $PXx<Output<OpenDrain>> {
        type Error = Infallible;

        fn try_is_high(&self) -> Result<bool, Self::Error> {
            let ans =
                (unsafe { &(*$GPIOX::ptr()).istat }.read().bits() & (1 << self.i)) != 0;
            Ok(ans)
        }

        fn try_is_low(&self) -> Result<bool, Self::Error> {
            Ok(!self.try_is_high()?)
        }
    }
$(
    /// Pin
    pub struct $PXi<MODE> {
        _typestate_mode: PhantomData<MODE>,
    }

    impl<MODE> PinIndex for $PXi<MODE> {
        const OP_LK_INDEX: usize = $i;

        const CTL_MD_INDEX: usize = (4 * $i) % 32;
    }

    impl<MODE> $PXi<MODE>
    where
        MODE: Active,
    {
        /// Configures the pin to serve as an analog input pin.
        pub fn into_analog(self, $ctl: &mut $CTL) -> $PXi<Analog> {
            self.into_with_ctrl_md($ctl, 0b00_00)
        }

        /// Configures the pin to serve as a floating input pin.
        pub fn into_floating_input(self, $ctl: &mut $CTL) -> $PXi<Input<Floating>> {
            self.into_with_ctrl_md($ctl, 0b01_00)
        }

        /// Configures the pin to serve as a pull down input pin.
        pub fn into_pull_down_input(
            self,
            $ctl: &mut $CTL,
            octl: &mut OCTL,
        ) -> $PXi<Input<PullDown>> {
            let r: &AtomicU32 = unsafe { &*(&octl.octl() as *const _ as *const _) };
            atomic_set_bit(r, false, Self::OP_LK_INDEX);
            self.into_with_ctrl_md($ctl, 0b10_00)
        }

        /// Configures the pin to serve as a pull up input pin.
        pub fn into_pull_up_input(
            self,
            $ctl: &mut $CTL,
            octl: &mut OCTL,
        ) -> $PXi<Input<PullUp>> {
            let r: &AtomicU32 = unsafe { &*(&octl.octl() as *const _ as *const _) };
            atomic_set_bit(r, true, Self::OP_LK_INDEX);
            self.into_with_ctrl_md($ctl, 0b10_00)
        }

        /// Configures the pin to serve as a push pull output pin;
        /// the maximum speed is set to the default value 50MHz.
        pub fn into_push_pull_output(
            self,
            $ctl: &mut $CTL,
        ) -> $PXi<Output<PushPull>> {
            let ctrl_md = 0b00_00 | UpTo50MHz::MD_BITS;
            self.into_with_ctrl_md($ctl, ctrl_md)
        }

        /// Configures the pin to serve as an open drain output pin;
        /// the maximum speed is set to the default value 50MHz.
        pub fn into_open_drain_output(
            self,
            $ctl: &mut $CTL,
        ) -> $PXi<Output<OpenDrain>> {
            let ctrl_md = 0b01_00 | UpTo50MHz::MD_BITS;
            self.into_with_ctrl_md($ctl, ctrl_md)
        }

        /// Configures the pin to serve as a push pull alternate pin;
        /// the maximum speed is set to the default value 50MHz.
        pub fn into_alternate_push_pull(
            self,
            $ctl: &mut $CTL,
        ) -> $PXi<Alternate<PushPull>> {
            let ctrl_md = 0b10_00 | UpTo50MHz::MD_BITS;
            self.into_with_ctrl_md($ctl, ctrl_md)
        }

        /// Configures the pin to serve as an open drain alternate pin;
        /// the maximum speed is set to the default value 50MHz.
        pub fn into_alternate_open_drain(
            self,
            $ctl: &mut $CTL,
        ) -> $PXi<Alternate<OpenDrain>> {
            let ctrl_md = 0b11_00 | UpTo50MHz::MD_BITS;
            self.into_with_ctrl_md($ctl, ctrl_md)
        }

        /// Configures the pin to serve as a push pull output pin with maximum speed given.
        pub fn into_push_pull_output_speed<SPEED: Speed>(
            self,
            $ctl: &mut $CTL,
        ) -> $PXi<Output<PushPull>> {
            let ctrl_md = 0b00_00 | SPEED::MD_BITS;
            self.into_with_ctrl_md($ctl, ctrl_md)
        }

        /// Configures the pin to serve as an open drain output pin with maximum speed given.
        pub fn into_open_drain_output_speed<SPEED: Speed>(
            self,
            $ctl: &mut $CTL,
        ) -> $PXi<Output<OpenDrain>> {
            let ctrl_md = 0b01_00 | SPEED::MD_BITS;
            self.into_with_ctrl_md($ctl, ctrl_md)
        }

        /// Configures the pin to serve as a push pull alternate pin with maximum speed given
        pub fn into_alternate_push_pull_speed<SPEED: Speed>(
            self,
            $ctl: &mut $CTL,
        ) -> $PXi<Alternate<PushPull>> {
            let ctrl_md = 0b10_00 | SPEED::MD_BITS;
            self.into_with_ctrl_md($ctl, ctrl_md)
        }

        /// Configures the pin to serve as an open drain alternate pin with maximum speed given.
        pub fn into_alternate_open_drain_speed<SPEED: Speed>(
            self,
            $ctl: &mut $CTL,
        ) -> $PXi<Alternate<OpenDrain>> {
            let ctrl_md = 0b11_00 | SPEED::MD_BITS;
            self.into_with_ctrl_md($ctl, ctrl_md)
        }

        #[inline]
        fn into_with_ctrl_md<T>(self, $ctl: &mut $CTL, ctl_and_md: u32) -> $PXi<T> {
            $ctl.$ctl().modify(|r, w| unsafe {
                w.bits(
                    (r.bits() & !(0b1111 << Self::CTL_MD_INDEX))
                        | (ctl_and_md << Self::CTL_MD_INDEX),
                )
            });
            $PXi {
                _typestate_mode: PhantomData,
            }
        }

        /// Lock the pin to prevent further configurations on pin mode.
        ///
        /// After this function is called, the pin is not actually locked; it only
        /// sets a marker temporary variant to prepare for the real lock freezing
        /// procedure `freeze`. To actually perform the lock, users are encouraged
        /// to call `freeze` after all pins configured and marked properly for lock.
        ///
        /// The output state of this pin can still be changed. You may unlock locked
        /// pins by using `unlock` method with a mutable reference of `LOCK` struct,
        /// but it will not be possible if `freeze` method of LOCK struct was
        /// called; see its documentation for details.
        #[inline]
        pub fn lock(self, lock: &mut LOCK) -> Locked<$PXi<MODE>> {
            let r: &AtomicU32 = unsafe { &*(&lock.tmp_bits as *const _ as *const _) };
            atomic_set_bit(r, true, Self::OP_LK_INDEX);
            Locked($PXi {
                _typestate_mode: PhantomData,
            })
        }
    }

    impl<MODE> Unlock for Locked<$PXi<MODE>>
    where
        MODE: Active,
    {
        type Lock = LOCK;

        type Output = $PXi<MODE>;

        #[inline]
        fn unlock(self, lock: &mut Self::Lock) -> Self::Output {
            // set temporary bit for this pin in LOCK struct
            let r: &AtomicU32 = unsafe { &*(&lock.tmp_bits as *const _ as *const _) };
            atomic_set_bit(r, false, $i); // PXi::OP_LK_INDEX
            $PXi {
                _typestate_mode: PhantomData,
            }
        }
    }

    impl<MODE> $PXi<MODE>
    where
        MODE: Active
    {
        /// Erases the pin number from the type.
        ///
        /// This is useful when you want to collect the pins into an array
        /// where you need all the elements to have the same type.
        pub fn downgrade(self) -> $PXx<MODE> {
            $PXx {
                i: $i,
                _typestate_mode: PhantomData
            }
        }
    }

    impl<MODE> InputPin for $PXi<Input<MODE>> {
        type Error = Infallible;

        fn try_is_high(&self) -> Result<bool, Self::Error> {
            let ans =
                (unsafe { &(*$GPIOX::ptr()).istat }.read().bits() & (1 << Self::OP_LK_INDEX)) != 0;
            Ok(ans)
        }

        fn try_is_low(&self) -> Result<bool, Self::Error> {
            Ok(!self.try_is_high()?)
        }
    }

    impl<MODE> OutputPin for $PXi<Output<MODE>> {
        type Error = Infallible;

        fn try_set_high(&mut self) -> Result<(), Self::Error> {
            unsafe { &(*$GPIOX::ptr()).bop }.write(|w| unsafe { w.bits(1 << Self::OP_LK_INDEX) });
            Ok(())
        }

        fn try_set_low(&mut self) -> Result<(), Self::Error> {
            unsafe { &(*$GPIOX::ptr()).bc }.write(|w| unsafe { w.bits(1 << Self::OP_LK_INDEX) });
            Ok(())
        }
    }

    impl<MODE> OutputPin for $PXi<Alternate<MODE>> {
        type Error = Infallible;

        fn try_set_high(&mut self) -> Result<(), Self::Error> {
            unsafe { &(*$GPIOX::ptr()).bop }.write(|w| unsafe { w.bits(1 << Self::OP_LK_INDEX) });
            Ok(())
        }

        fn try_set_low(&mut self) -> Result<(), Self::Error> {
            unsafe { &(*$GPIOX::ptr()).bc }.write(|w| unsafe { w.bits(1 << Self::OP_LK_INDEX) });
            Ok(())
        }
    }

    impl<MODE> StatefulOutputPin for $PXi<Output<MODE>> {
        fn try_is_set_high(&self) -> Result<bool, Self::Error> {
            let ans =
                (unsafe { &(*$GPIOX::ptr()).octl }.read().bits() & (1 << Self::OP_LK_INDEX)) != 0;
            Ok(ans)
        }

        fn try_is_set_low(&self) -> Result<bool, Self::Error> {
            Ok(!self.try_is_set_high()?)
        }
    }

    impl<MODE> StatefulOutputPin for $PXi<Alternate<MODE>> {
        fn try_is_set_high(&self) -> Result<bool, Self::Error> {
            let ans =
                (unsafe { &(*$GPIOX::ptr()).octl }.read().bits() & (1 << Self::OP_LK_INDEX)) != 0;
            Ok(ans)
        }

        fn try_is_set_low(&self) -> Result<bool, Self::Error> {
            Ok(!self.try_is_set_high()?)
        }
    }

    impl<MODE> ToggleableOutputPin for $PXi<Output<MODE>> {
        type Error = Infallible;

        fn try_toggle(&mut self) -> Result<(), Self::Error> {
            let r: &AtomicU32 = unsafe { &*(&(*$GPIOX::ptr()).octl as *const _ as *const _) };
            atomic_toggle_bit(r, Self::OP_LK_INDEX);
            Ok(())
        }
    }

    impl<MODE> ToggleableOutputPin for $PXi<Alternate<MODE>> {
        type Error = Infallible;

        fn try_toggle(&mut self) -> Result<(), Self::Error> {
            let r: &AtomicU32 = unsafe { &*(&(*$GPIOX::ptr()).octl as *const _ as *const _) };
            atomic_toggle_bit(r, Self::OP_LK_INDEX);
            Ok(())
        }
    }

    impl InputPin for $PXi<Output<OpenDrain>> {
        type Error = Infallible;

        fn try_is_high(&self) -> Result<bool, Self::Error> {
            let ans =
                (unsafe { &(*$GPIOX::ptr()).istat }.read().bits() & (1 << Self::OP_LK_INDEX)) != 0;
            Ok(ans)
        }

        fn try_is_low(&self) -> Result<bool, Self::Error> {
            Ok(!self.try_is_high()?)
        }
    }
)+
}
    };
}

impl_gpio! { GPIOA, gpioa, gpioa, paen, parst, PAx, [
    PA0: (pa0, 0, Input<Floating>, CTL0, ctl0),
    PA1: (pa1, 1, Input<Floating>, CTL0, ctl0),
    PA2: (pa2, 2, Input<Floating>, CTL0, ctl0),
    PA3: (pa3, 3, Input<Floating>, CTL0, ctl0),
    PA4: (pa4, 4, Input<Floating>, CTL0, ctl0),
    PA5: (pa5, 5, Input<Floating>, CTL0, ctl0),
    PA6: (pa6, 6, Input<Floating>, CTL0, ctl0),
    PA7: (pa7, 7, Input<Floating>, CTL0, ctl0),
    PA8: (pa8, 8, Input<Floating>, CTL1, ctl1),
    PA9: (pa9, 9, Input<Floating>, CTL1, ctl1),
    PA10: (pa10, 10, Input<Floating>, CTL1, ctl1),
    PA11: (pa11, 11, Input<Floating>, CTL1, ctl1),
    PA12: (pa12, 12, Input<Floating>, CTL1, ctl1),
    PA13: (pa13, 13, Input<PullUp>, CTL1, ctl1),
    PA14: (pa14, 14, Input<PullDown>, CTL1, ctl1),
    PA15: (pa15, 15, Input<PullUp>, CTL1, ctl1),
] }

impl_gpio! { GPIOB, gpiob, gpioa, pben, pbrst, PBx, [
    PB0: (pb0, 0, Input<Floating>, CTL0, ctl0),
    PB1: (pb1, 1, Input<Floating>, CTL0, ctl0),
    PB2: (pb2, 2, Input<Floating>, CTL0, ctl0),
    PB3: (pb3, 3, Input<Floating>, CTL0, ctl0),
    PB4: (pb4, 4, Input<PullUp>, CTL0, ctl0),
    PB5: (pb5, 5, Input<Floating>, CTL0, ctl0),
    PB6: (pb6, 6, Input<Floating>, CTL0, ctl0),
    PB7: (pb7, 7, Input<Floating>, CTL0, ctl0),
    PB8: (pb8, 8, Input<Floating>, CTL1, ctl1),
    PB9: (pb9, 9, Input<Floating>, CTL1, ctl1),
    PB10: (pb10, 10, Input<Floating>, CTL1, ctl1),
    PB11: (pb11, 11, Input<Floating>, CTL1, ctl1),
    PB12: (pb12, 12, Input<Floating>, CTL1, ctl1),
    PB13: (pb13, 13, Input<Floating>, CTL1, ctl1),
    PB14: (pb14, 14, Input<Floating>, CTL1, ctl1),
    PB15: (pb15, 15, Input<Floating>, CTL1, ctl1),
] }

impl_gpio! { GPIOC, gpioc, gpioa, pcen, pcrst, PCx, [
    PC0: (pc0, 0, Input<Floating>, CTL0, ctl0),
    PC1: (pc1, 1, Input<Floating>, CTL0, ctl0),
    PC2: (pc2, 2, Input<Floating>, CTL0, ctl0),
    PC3: (pc3, 3, Input<Floating>, CTL0, ctl0),
    PC4: (pc4, 4, Input<Floating>, CTL0, ctl0),
    PC5: (pc5, 5, Input<Floating>, CTL0, ctl0),
    PC6: (pc6, 6, Input<Floating>, CTL0, ctl0),
    PC7: (pc7, 7, Input<Floating>, CTL0, ctl0),
    PC8: (pc8, 8, Input<Floating>, CTL1, ctl1),
    PC9: (pc9, 9, Input<Floating>, CTL1, ctl1),
    PC10: (pc10, 10, Input<Floating>, CTL1, ctl1),
    PC11: (pc11, 11, Input<Floating>, CTL1, ctl1),
    PC12: (pc12, 12, Input<Floating>, CTL1, ctl1),
    PC13: (pc13, 13, Input<Floating>, CTL1, ctl1),
    PC14: (pc14, 14, Input<Floating>, CTL1, ctl1),
    PC15: (pc15, 15, Input<Floating>, CTL1, ctl1),
] }

impl_gpio! { GPIOD, gpiod, gpioa, pden, pdrst, PDx, [
    PD0: (pd0, 0, Input<Floating>, CTL0, ctl0),
    PD1: (pd1, 1, Input<Floating>, CTL0, ctl0),
    PD2: (pd2, 2, Input<Floating>, CTL0, ctl0),
    PD3: (pd3, 3, Input<Floating>, CTL0, ctl0),
    PD4: (pd4, 4, Input<Floating>, CTL0, ctl0),
    PD5: (pd5, 5, Input<Floating>, CTL0, ctl0),
    PD6: (pd6, 6, Input<Floating>, CTL0, ctl0),
    PD7: (pd7, 7, Input<Floating>, CTL0, ctl0),
    PD8: (pd8, 8, Input<Floating>, CTL1, ctl1),
    PD9: (pd9, 9, Input<Floating>, CTL1, ctl1),
    PD10: (pd10, 10, Input<Floating>, CTL1, ctl1),
    PD11: (pd11, 11, Input<Floating>, CTL1, ctl1),
    PD12: (pd12, 12, Input<Floating>, CTL1, ctl1),
    PD13: (pd13, 13, Input<Floating>, CTL1, ctl1),
    PD14: (pd14, 14, Input<Floating>, CTL1, ctl1),
    PD15: (pd15, 15, Input<Floating>, CTL1, ctl1),
] }

impl_gpio! { GPIOE, gpioe, gpioa, peen, perst, PEx, [
    PE0: (pe0, 0, Input<Floating>, CTL0, ctl0),
    PE1: (pe1, 1, Input<Floating>, CTL0, ctl0),
    PE2: (pe2, 2, Input<Floating>, CTL0, ctl0),
    PE3: (pe3, 3, Input<Floating>, CTL0, ctl0),
    PE4: (pe4, 4, Input<Floating>, CTL0, ctl0),
    PE5: (pe5, 5, Input<Floating>, CTL0, ctl0),
    PE6: (pe6, 6, Input<Floating>, CTL0, ctl0),
    PE7: (pe7, 7, Input<Floating>, CTL0, ctl0),
    PE8: (pe8, 8, Input<Floating>, CTL1, ctl1),
    PE9: (pe9, 9, Input<Floating>, CTL1, ctl1),
    PE10: (pe10, 10, Input<Floating>, CTL1, ctl1),
    PE11: (pe11, 11, Input<Floating>, CTL1, ctl1),
    PE12: (pe12, 12, Input<Floating>, CTL1, ctl1),
    PE13: (pe13, 13, Input<Floating>, CTL1, ctl1),
    PE14: (pe14, 14, Input<Floating>, CTL1, ctl1),
    PE15: (pe15, 15, Input<Floating>, CTL1, ctl1),
] }
