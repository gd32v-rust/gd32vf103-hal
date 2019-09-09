//! Hardware abstract layer (HAL) for the GD32VF103 microcontroller chip.
#![no_std]
#![deny(missing_docs)]

pub use gd32vf103_pac as pac;

pub mod gpio;
pub mod rcu;
pub mod spi;
pub mod time;
pub mod timer;

/// Prelude
pub mod prelude {
    pub use crate::gpio::GpioExt as _gd32vf103_hal_gpio_GpioExt;
    pub use crate::rcu::RcuExt as _gd32vf103_hal_rcu_RcuExt;
    pub use crate::time::U32Ext as _gd32vf103_hal_time_U32Ext;
    pub use crate::gpio::{UpTo10MHz, UpTo2MHz, UpTo50MHz};
    pub use embedded_hal::prelude::*;
    pub use embedded_hal::digital::v2::*;
}
