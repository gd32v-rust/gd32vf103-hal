//! Hardware abstract layer (HAL) for the GD32VF103 microcontroller chip.
#![no_std]

#![deny(missing_docs)]

pub use gd32vf103_pac as pac;

pub mod gpio;
pub mod spi;

/// Prelude
pub mod prelude {
    pub use embedded_hal::prelude::*;
    pub use crate::gpio::{UpTo2MHz, UpTo10MHz, UpTo50MHz};
    pub use crate::gpio::GpioExt as _gd32vf103_hal_gpio_GpioExt;
}
