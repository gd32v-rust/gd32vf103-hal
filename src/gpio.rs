//! General Purpose Input / Output

use core::marker::PhantomData;
use core::sync::atomic::{AtomicU32, Ordering};
use crate::rcu::APB2;

/// Extension trait to split a GPIO peripheral into independent pins and registers
pub trait GpioExt {
    /// The type to split the GPIO into
    type Parts;

    /// Splits the GPIO block into independent pins and registers
    fn split(self, apb2: &mut APB2) -> Self::Parts;
}

/// Pin is locked (type state)
pub struct Locked;

/// Pin is not locked (type state)
pub struct Unlocked;

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

impl<MODE> Active for Output<MODE>{}

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

#[inline]
fn atomic_set_bit(r: &AtomicU32, is_one: bool, index: usize) {
    let mask = 1 << index;
    match is_one {
        true => r.fetch_or(mask, Ordering::SeqCst),
        false => r.fetch_nand(mask, Ordering::SeqCst),
    };
}

#[inline]
fn atomic_toggle_bit(r: &AtomicU32, index: usize) {
    let mask = 1 << index;
    r.fetch_xor(mask, Ordering::SeqCst);
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
        Active, Alternate, Analog, Floating, GpioExt, Input, Locked, OpenDrain, Output, 
        PinIndex, PullDown, PullUp, PushPull, Speed, Unlocked, UpTo50MHz,
    };
    use crate::pac::{$gpioy, $GPIOX};
    use crate::rcu::APB2;
    use core::convert::Infallible;
    use core::marker::PhantomData;
    use core::sync::atomic::AtomicU32;
    use embedded_hal::digital::v2::{InputPin, OutputPin, StatefulOutputPin, ToggleableOutputPin};

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
            pub $pxi: $PXi<Unlocked, $MODE>,
        )+
        #[doc(hidden)]
        _extensible: (),
    }

    impl GpioExt for $GPIOX {
        type Parts = Parts;

        fn split(self, apb2: &mut APB2) -> Self::Parts {
            apb2.en().write(|w| w.$en().set_bit());
            apb2.rst().write(|w| w.$rst().set_bit());
            apb2.rst().write(|w| w.$rst().clear_bit());
            Parts {
                ctl0: CTL0 { _ownership: () },
                ctl1: CTL1 { _ownership: () },
                octl: OCTL { _ownership: () },
                lock: LOCK { _ownership: () },
                $(
                    $pxi: $PXi {
                        _typestate_locked: PhantomData,
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
        _ownership: (),
    }

    impl LOCK {
        pub(crate) fn lock(&mut self) -> &$gpioy::LOCK {
            unsafe { &(*$GPIOX::ptr()).lock }
        }

        /// Lock all LK lock bits in this GPIO port to prevent furtuer modifications
        /// on pin mode configurations.
        ///
        /// This operation cannot be undone so it consumes the LOCK ownership
        /// handle `self`. By the time this function succeeds to execute, the
        /// program cannot unlock LK bits anymore before chip reset.
        ///
        /// Instead of returning the LOCK back, this function panics on lock failure.
        /// That's because we consider all lock failures comes from mistakes in
        /// underlying libraries or chip design which may be not proper for users
        /// to handle by themselves. If this design results in mistake, please
        /// fire an issue to let us know.
        pub fn lock_all_pins(mut self) {
            let r: &AtomicU32 = unsafe { core::mem::transmute(self.lock()) };
            super::atomic_set_bit(r, true, 16);
            super::atomic_set_bit(r, false, 16);
            super::atomic_set_bit(r, true, 16);
            let ans1 = self.lock().read().bits() & (1 << 16);
            let ans2 = self.lock().read().bits() & (1 << 16);
            if ans1 == 0 && ans2 == 1 {
                return;
            } else {
                panic!("the lock_all_pins process won't succeed")
            }
        }
    }

    /// Partially erased pin
    pub struct $PXx<LOCKED, MODE> {
        i: u8,
        _typestate_mode: PhantomData<MODE>,
        _typestate_locked: PhantomData<LOCKED>,
    }

    impl<LOCKED, MODE> InputPin for $PXx<LOCKED, Input<MODE>> {
        type Error = Infallible;

        fn is_high(&self) -> Result<bool, Self::Error> {
            let ans =
                (unsafe { &(*$GPIOX::ptr()).istat }.read().bits() & (1 << self.i)) != 0;
            Ok(ans)
        }

        fn is_low(&self) -> Result<bool, Self::Error> {
            Ok(!self.is_high()?)
        }
    }

    impl<LOCKED, MODE> OutputPin for $PXx<LOCKED, Output<MODE>> {
        type Error = Infallible;

        fn set_high(&mut self) -> Result<(), Self::Error> {
            unsafe { &(*$GPIOX::ptr()).bop }.write(|w| unsafe { w.bits(1 << self.i) });
            Ok(())
        }

        fn set_low(&mut self) -> Result<(), Self::Error> {
            unsafe { &(*$GPIOX::ptr()).bc }.write(|w| unsafe { w.bits(1 << self.i) });
            Ok(())
        }
    }

    impl<LOCKED, MODE> StatefulOutputPin for $PXx<LOCKED, Output<MODE>> {
        fn is_set_high(&self) -> Result<bool, Self::Error> {
            let ans =
                (unsafe { &(*$GPIOX::ptr()).octl }.read().bits() & (1 << self.i)) != 0;
            Ok(ans)
        }

        fn is_set_low(&self) -> Result<bool, Self::Error> {
            Ok(!self.is_set_high()?)
        }
    }

    impl<LOCKED, MODE> ToggleableOutputPin for $PXx<LOCKED, Output<MODE>> {
        type Error = Infallible;

        fn toggle(&mut self) -> Result<(), Self::Error> {
            let r: &AtomicU32 = unsafe { core::mem::transmute(&(*$GPIOX::ptr()).octl) };
            super::atomic_toggle_bit(r, self.i as usize);
            Ok(())
        }
    }

    impl<LOCKED> InputPin for $PXx<LOCKED, Output<OpenDrain>> {
        type Error = Infallible;

        fn is_high(&self) -> Result<bool, Self::Error> {
            let ans =
                (unsafe { &(*$GPIOX::ptr()).istat }.read().bits() & (1 << self.i)) != 0;
            Ok(ans)
        }

        fn is_low(&self) -> Result<bool, Self::Error> {
            Ok(!self.is_high()?)
        }
    }
$(
    /// Pin
    pub struct $PXi<LOCKED, MODE> {
        _typestate_locked: PhantomData<LOCKED>,
        _typestate_mode: PhantomData<MODE>,
    }

    impl<LOCKED, MODE> PinIndex for $PXi<LOCKED, MODE> {
        const OP_LK_INDEX: usize = $i;

        const CTL_MD_INDEX: usize = (4 * $i) % 32;
    }

    impl<MODE> $PXi<Unlocked, MODE>
    where
        MODE: Active,
    {
        /// Configures the pin to serve as an analog input pin.
        pub fn into_analog(self, $ctl: &mut $CTL) -> $PXi<Unlocked, Analog> {
            self.into_with_ctrl_md($ctl, 0b00_00)
        }

        /// Configures the pin to serve as a floating input pin.
        pub fn into_floating_input(self, $ctl: &mut $CTL) -> $PXi<Unlocked, Input<Floating>> {
            self.into_with_ctrl_md($ctl, 0b01_00)
        }

        /// Configures the pin to serve as a pull down input pin.
        pub fn into_pull_down_input(
            self,
            $ctl: &mut $CTL,
            octl: &mut OCTL,
        ) -> $PXi<Unlocked, Input<PullDown>> {
            let r: &AtomicU32 = unsafe { core::mem::transmute(octl.octl()) };
            super::atomic_set_bit(r, false, Self::OP_LK_INDEX);
            self.into_with_ctrl_md($ctl, 0b10_00)
        }

        /// Configures the pin to serve as a pull up input pin.
        pub fn into_pull_up_input(
            self,
            $ctl: &mut $CTL,
            octl: &mut OCTL,
        ) -> $PXi<Unlocked, Input<PullUp>> {
            let r: &AtomicU32 = unsafe { core::mem::transmute(octl.octl()) };
            super::atomic_set_bit(r, true, Self::OP_LK_INDEX);
            self.into_with_ctrl_md($ctl, 0b10_00)
        }

        /// Configures the pin to serve as a push pull output pin;
        /// the maximum speed is set to the default value 50MHz.
        pub fn into_push_pull_output(
            self,
            $ctl: &mut $CTL,
        ) -> $PXi<Unlocked, Output<PushPull>> {
            let ctrl_md = 0b00_00 | UpTo50MHz::MD_BITS;
            self.into_with_ctrl_md($ctl, ctrl_md)
        }

        /// Configures the pin to serve as an open drain output pin;
        /// the maximum speed is set to the default value 50MHz.
        pub fn into_open_drain_output(
            self,
            $ctl: &mut $CTL,
        ) -> $PXi<Unlocked, Output<OpenDrain>> {
            let ctrl_md = 0b01_00 | UpTo50MHz::MD_BITS;
            self.into_with_ctrl_md($ctl, ctrl_md)
        }

        /// Configures the pin to serve as a push pull alternate pin;
        /// the maximum speed is set to the default value 50MHz.
        pub fn into_push_pull_alternate(
            self,
            $ctl: &mut $CTL,
        ) -> $PXi<Unlocked, Alternate<PushPull>> {
            let ctrl_md = 0b10_00 | UpTo50MHz::MD_BITS;
            self.into_with_ctrl_md($ctl, ctrl_md)
        }

        /// Configures the pin to serve as an open drain alternate pin;
        /// the maximum speed is set to the default value 50MHz.
        pub fn into_open_drain_alternate(
            self,
            $ctl: &mut $CTL,
        ) -> $PXi<Unlocked, Alternate<OpenDrain>> {
            let ctrl_md = 0b11_00 | UpTo50MHz::MD_BITS;
            self.into_with_ctrl_md($ctl, ctrl_md)
        }

        /// Configures the pin to serve as a push pull output pin with maximum speed given.
        pub fn into_push_pull_output_speed<SPEED: Speed>(
            self,
            $ctl: &mut $CTL,
        ) -> $PXi<Unlocked, Output<PushPull>> {
            let ctrl_md = 0b00_00 | SPEED::MD_BITS;
            self.into_with_ctrl_md($ctl, ctrl_md)
        }

        /// Configures the pin to serve as an open drain output pin with maximum speed given.
        pub fn into_open_drain_output_speed<SPEED: Speed>(
            self,
            $ctl: &mut $CTL,
        ) -> $PXi<Unlocked, Output<OpenDrain>> {
            let ctrl_md = 0b01_00 | SPEED::MD_BITS;
            self.into_with_ctrl_md($ctl, ctrl_md)
        }

        /// Configures the pin to serve as a push pull alternate pin with maximum speed given
        pub fn into_push_pull_alternate_speed<SPEED: Speed>(
            self,
            $ctl: &mut $CTL,
        ) -> $PXi<Unlocked, Alternate<PushPull>> {
            let ctrl_md = 0b10_00 | SPEED::MD_BITS;
            self.into_with_ctrl_md($ctl, ctrl_md)
        }

        /// Configures the pin to serve as an open drain alternate pin with maximum speed given.
        pub fn into_open_drain_alternate_speed<SPEED: Speed>(
            self,
            $ctl: &mut $CTL,
        ) -> $PXi<Unlocked, Alternate<OpenDrain>> {
            let ctrl_md = 0b11_00 | SPEED::MD_BITS;
            self.into_with_ctrl_md($ctl, ctrl_md)
        }

        #[inline]
        fn into_with_ctrl_md<T>(self, $ctl: &mut $CTL, ctl_and_md: u32) -> $PXi<Unlocked, T> {
            $ctl.$ctl().modify(|r, w| unsafe {
                w.bits(
                    (r.bits() & !(0b1111 << Self::CTL_MD_INDEX))
                        | (ctl_and_md << Self::CTL_MD_INDEX),
                )
            });
            $PXi {
                _typestate_locked: PhantomData,
                _typestate_mode: PhantomData,
            }
        }

        /// Lock the pin to prevent further configurations on pin mode.
        ///
        /// The output state of this pin can still be changed. You may unlock locked
        /// pins by using `unlock` method with a mutable reference of `LOCK` struct,
        /// but it will not be possible if `lock_all_pins` method of LOCK struct was
        /// called; see its documentation for details.
        pub fn lock(self, lock: &mut LOCK) -> $PXi<Locked, MODE> {
            let r: &AtomicU32 = unsafe { core::mem::transmute(lock.lock()) };
            super::atomic_set_bit(r, true, Self::OP_LK_INDEX);
            $PXi {
                _typestate_locked: PhantomData,
                _typestate_mode: PhantomData,
            }
        }
    }

    impl<MODE> $PXi<Locked, MODE>
    where
        MODE: Active,
    {
        /// Unlock this locked pin to allow configurations of pin mode.
        ///
        /// You don't need to unlock pins if you only want to change output state
        /// other than reconfigurate the pin mode. The caller of this method must
        /// obtain a mutable reference of `LOCK` struct; if you have called the
        /// `lock_all_pins` method of that struct, you would be no longer possible
        /// to change lock state or unlock any locked pins - see its documentation
        ///  for details.
        pub fn unlock(self, lock: &mut LOCK) -> $PXi<Unlocked, MODE> {
            let r: &AtomicU32 = unsafe { core::mem::transmute(lock.lock()) };
            super::atomic_set_bit(r, false, Self::OP_LK_INDEX);
            $PXi {
                _typestate_locked: PhantomData,
                _typestate_mode: PhantomData,
            }
        }
    }

    impl<LOCKED, MODE> $PXi<LOCKED, MODE> 
    where 
        MODE: Active 
    {
        /// Erases the pin number from the type.
        /// 
        /// This is useful when you want to collect the pins into an array 
        /// where you need all the elements to have the same type.
        pub fn downgrade(self) -> $PXx<LOCKED, MODE> {
            $PXx {
                i: $i,
                _typestate_locked: PhantomData,
                _typestate_mode: PhantomData
            }
        }
    }

    impl<LOCKED, MODE> InputPin for $PXi<LOCKED, Input<MODE>> {
        type Error = Infallible;

        fn is_high(&self) -> Result<bool, Self::Error> {
            let ans =
                (unsafe { &(*$GPIOX::ptr()).istat }.read().bits() & (1 << Self::OP_LK_INDEX)) != 0;
            Ok(ans)
        }

        fn is_low(&self) -> Result<bool, Self::Error> {
            Ok(!self.is_high()?)
        }
    }

    impl<LOCKED, MODE> OutputPin for $PXi<LOCKED, Output<MODE>> {
        type Error = Infallible;

        fn set_high(&mut self) -> Result<(), Self::Error> {
            unsafe { &(*$GPIOX::ptr()).bop }.write(|w| unsafe { w.bits(1 << Self::OP_LK_INDEX) });
            Ok(())
        }

        fn set_low(&mut self) -> Result<(), Self::Error> {
            unsafe { &(*$GPIOX::ptr()).bc }.write(|w| unsafe { w.bits(1 << Self::OP_LK_INDEX) });
            Ok(())
        }
    }

    impl<LOCKED, MODE> OutputPin for $PXi<LOCKED, Alternate<MODE>> {
        type Error = Infallible;

        fn set_high(&mut self) -> Result<(), Self::Error> {
            unsafe { &(*$GPIOX::ptr()).bop }.write(|w| unsafe { w.bits(1 << Self::OP_LK_INDEX) });
            Ok(())
        }

        fn set_low(&mut self) -> Result<(), Self::Error> {
            unsafe { &(*$GPIOX::ptr()).bc }.write(|w| unsafe { w.bits(1 << Self::OP_LK_INDEX) });
            Ok(())
        }
    }

    impl<LOCKED, MODE> StatefulOutputPin for $PXi<LOCKED, Output<MODE>> {
        fn is_set_high(&self) -> Result<bool, Self::Error> {
            let ans =
                (unsafe { &(*$GPIOX::ptr()).octl }.read().bits() & (1 << Self::OP_LK_INDEX)) != 0;
            Ok(ans)
        }

        fn is_set_low(&self) -> Result<bool, Self::Error> {
            Ok(!self.is_set_high()?)
        }
    }

    impl<LOCKED, MODE> StatefulOutputPin for $PXi<LOCKED, Alternate<MODE>> {
        fn is_set_high(&self) -> Result<bool, Self::Error> {
            let ans =
                (unsafe { &(*$GPIOX::ptr()).octl }.read().bits() & (1 << Self::OP_LK_INDEX)) != 0;
            Ok(ans)
        }

        fn is_set_low(&self) -> Result<bool, Self::Error> {
            Ok(!self.is_set_high()?)
        }
    }

    impl<LOCKED, MODE> ToggleableOutputPin for $PXi<LOCKED, Output<MODE>> {
        type Error = Infallible;

        fn toggle(&mut self) -> Result<(), Self::Error> {
            let r: &AtomicU32 = unsafe { core::mem::transmute(&(*$GPIOX::ptr()).octl) };
            super::atomic_toggle_bit(r, Self::OP_LK_INDEX);
            Ok(())
        }
    }

    impl<LOCKED, MODE> ToggleableOutputPin for $PXi<LOCKED, Alternate<MODE>> {
        type Error = Infallible;

        fn toggle(&mut self) -> Result<(), Self::Error> {
            let r: &AtomicU32 = unsafe { core::mem::transmute(&(*$GPIOX::ptr()).octl) };
            super::atomic_toggle_bit(r, Self::OP_LK_INDEX);
            Ok(())
        }
    }

    impl<LOCKED> InputPin for $PXi<LOCKED, Output<OpenDrain>> {
        type Error = Infallible;

        fn is_high(&self) -> Result<bool, Self::Error> {
            let ans =
                (unsafe { &(*$GPIOX::ptr()).istat }.read().bits() & (1 << Self::OP_LK_INDEX)) != 0;
            Ok(ans)
        }

        fn is_low(&self) -> Result<bool, Self::Error> {
            Ok(!self.is_high()?)
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
