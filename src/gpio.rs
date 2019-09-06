use core::marker::PhantomData;
use core::sync::atomic::{AtomicU32, Ordering};

pub trait GpioExt {
    type Parts;

    fn split(self) -> Self::Parts;
}

pub struct Locked;

pub struct Unlocked;

pub struct Input<MODE> {
    _typestate_mode: PhantomData<MODE>,
}
pub struct Analog;

pub struct Floating;

pub struct PullDown;

pub struct PullUp;

pub struct Output<MODE, SPEED> {
    _typestate_mode: PhantomData<MODE>,
    _typestate_speed: PhantomData<SPEED>,
}

pub struct Alternate<MODE, SPEED> {
    _typestate_mode: PhantomData<MODE>,
    _typestate_speed: PhantomData<SPEED>,
}

pub struct PushPull;

pub struct OpenDrain;

pub trait InputMode {}

impl InputMode for Analog {}

impl InputMode for Floating {}

impl InputMode for PullDown {}

impl InputMode for PullUp {}

pub trait OutputMode {}

impl OutputMode for PushPull {}

impl OutputMode for OpenDrain {}

pub trait AlternateMode {}

impl AlternateMode for PushPull {}

impl AlternateMode for OpenDrain {}

pub trait Active {}

impl<MODE> Active for Input<MODE> where MODE: InputMode {}

impl<MODE, SPEED> Active for Output<MODE, SPEED>
where
    MODE: OutputMode,
    SPEED: Speed,
{
}

impl<MODE, SPEED> Active for Alternate<MODE, SPEED>
where
    MODE: AlternateMode,
    SPEED: Speed,
{
}

pub struct UpTo10MHz;

pub struct UpTo2MHz;

pub struct UpTo50MHz;

pub trait Speed {
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
    let mask = 1 << (index & 31);
    match is_one {
        true => r.fetch_or(mask, Ordering::SeqCst),
        false => r.fetch_nand(mask, Ordering::SeqCst),
    };
}

pub mod gpioa {
    use super::{
        Active, Alternate, AlternateMode, Analog, Floating, Input, OpenDrain, Output, OutputMode,
        PullDown, PullUp, PushPull, Speed, Unlocked,
    };
    use crate::pac::{gpioa, GPIOA};
    use core::marker::PhantomData;
    use core::sync::atomic::AtomicU32;

    pub struct Parts {
        pub ctl0: CTL0,
        //ctl1
        pub octl: OCTL,
        pub pa0: PA0<Unlocked, Input<Floating>>,
        //pa1, ..
    }

    pub struct CTL0 {
        _ownership: (),
    }

    impl CTL0 {
        pub(crate) fn ctl0(&mut self) -> &gpioa::CTL0 {
            unsafe { &(*GPIOA::ptr()).ctl0 }
        }
    }

    pub struct OCTL {
        _ownership: (),
    }

    impl OCTL {
        pub(crate) fn octl(&mut self) -> &gpioa::OCTL {
            unsafe { &(*GPIOA::ptr()).octl }
        }
    }

    pub struct PA0<LOCKED, MODE> {
        _typestate_locked: PhantomData<LOCKED>,
        _typestate_mode: PhantomData<MODE>,
    }

    impl<MODE> PA0<Unlocked, MODE>
    where
        MODE: Active,
    {
        pub fn into_analog_input(self, ctl0: &mut CTL0) -> PA0<Unlocked, Input<Analog>> {
            self.into_with_ctrl_md(ctl0, 0b00_00)
        }

        pub fn into_floating_input(self, ctl0: &mut CTL0) -> PA0<Unlocked, Input<Floating>> {
            self.into_with_ctrl_md(ctl0, 0b01_00)
        }

        pub fn into_pull_down_input(
            self,
            ctl0: &mut CTL0,
            octl: &mut OCTL,
        ) -> PA0<Unlocked, Input<PullDown>> {
            let r: &AtomicU32 = unsafe { core::mem::transmute(octl.octl()) };
            super::atomic_set_bit(r, false, 0); //
            self.into_with_ctrl_md(ctl0, 0b10_00)
        }

        pub fn into_pull_up_input(
            self,
            ctl0: &mut CTL0,
            octl: &mut OCTL,
        ) -> PA0<Unlocked, Input<PullUp>> {
            let r: &AtomicU32 = unsafe { core::mem::transmute(octl.octl()) };
            super::atomic_set_bit(r, true, 0); //
            self.into_with_ctrl_md(ctl0, 0b10_00)
        }

        pub fn into_push_pull_output_speed<SPEED: Speed>(
            self,
            ctl0: &mut CTL0,
        ) -> PA0<Unlocked, Output<PushPull, SPEED>> {
            let ctrl_md = 0b00_00 | SPEED::MD_BITS;
            self.into_with_ctrl_md(ctl0, ctrl_md)
        }

        pub fn into_open_drain_output_speed<SPEED: Speed>(
            self,
            ctl0: &mut CTL0,
        ) -> PA0<Unlocked, Output<OpenDrain, SPEED>> {
            let ctrl_md = 0b01_00 | SPEED::MD_BITS;
            self.into_with_ctrl_md(ctl0, ctrl_md)
        }

        pub fn into_push_pull_alternate_speed<SPEED: Speed>(
            self,
            ctl0: &mut CTL0,
        ) -> PA0<Unlocked, Alternate<PushPull, SPEED>> {
            let ctrl_md = 0b10_00 | SPEED::MD_BITS;
            self.into_with_ctrl_md(ctl0, ctrl_md)
        }

        pub fn into_open_drain_alternate_speed<SPEED: Speed>(
            self,
            ctl0: &mut CTL0,
        ) -> PA0<Unlocked, Alternate<OpenDrain, SPEED>> {
            let ctrl_md = 011_00 | SPEED::MD_BITS;
            self.into_with_ctrl_md(ctl0, ctrl_md)
        }

        #[inline]
        fn into_with_ctrl_md<T>(self, ctl0: &mut CTL0, ctl_and_md: u32) -> PA0<Unlocked, T> {
            ctl0.ctl0().modify(|r, w| unsafe {
                w.bits((r.bits() & !(0b1111 << 0)) | (ctl_and_md << 0)) //
            });
            PA0 {
                _typestate_locked: PhantomData,
                _typestate_mode: PhantomData,
            }
        }
    }

    impl<MODE, SPEED> PA0<Unlocked, Output<MODE, SPEED>>
    where
        MODE: OutputMode,
        SPEED: Speed,
    {
        pub fn into_push_pull_output(
            self,
            ctl0: &mut CTL0,
        ) -> PA0<Unlocked, Output<PushPull, SPEED>> {
            let r: &AtomicU32 = unsafe { core::mem::transmute(ctl0.ctl0()) };
            super::atomic_set_bit(r, false, 0 << 4); // $i << 4
            PA0 {
                _typestate_locked: PhantomData,
                _typestate_mode: PhantomData,
            }
        }

        pub fn into_open_drain_output(
            self,
            ctl0: &mut CTL0,
        ) -> PA0<Unlocked, Output<OpenDrain, SPEED>> {
            let r: &AtomicU32 = unsafe { core::mem::transmute(ctl0.ctl0()) };
            super::atomic_set_bit(r, true, 0 << 4); //
            PA0 {
                _typestate_locked: PhantomData,
                _typestate_mode: PhantomData,
            }
        }
    }

    impl<MODE, SPEED> PA0<Unlocked, Alternate<MODE, SPEED>>
    where
        MODE: AlternateMode,
        SPEED: Speed,
    {
        pub fn into_push_pull_alternate(
            self,
            ctl0: &mut CTL0,
        ) -> PA0<Unlocked, Alternate<PushPull, SPEED>> {
            let r: &AtomicU32 = unsafe { core::mem::transmute(ctl0.ctl0()) };
            super::atomic_set_bit(r, false, 0 << 4); //
            PA0 {
                _typestate_locked: PhantomData,
                _typestate_mode: PhantomData,
            }
        }

        pub fn into_open_drain_alternate(
            self,
            ctl0: &mut CTL0,
        ) -> PA0<Unlocked, Alternate<OpenDrain, SPEED>> {
            let r: &AtomicU32 = unsafe { core::mem::transmute(ctl0.ctl0()) };
            super::atomic_set_bit(r, true, 0 << 4); //
            PA0 {
                _typestate_locked: PhantomData,
                _typestate_mode: PhantomData,
            }
        }
    }

    
}
