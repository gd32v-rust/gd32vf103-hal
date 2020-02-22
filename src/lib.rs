//! Hardware abstract layer (HAL) for the GD32VF103 microcontroller chip.
//!
//! This is an implementation of `embedded-hal` traits for the GD32VF103, MCU with
//! one RISC-V's RV32IMAC core as well as up to 128 KiB of Flash and 32 KiB of SRAM,
//! produced by GigaDevice Semiconductor Inc.
//!
//! # Usage
//! Add this crate to your dependencies:
//! ```
//! [dependencies]
//! gd32vf103-hal = "0.0"
//! ```
//!
//! # Example
//! ```
//! #![no_std]
//! #![no_main]
//!
//! extern crate panic_halt;
//!
//! use riscv_rt::entry;
//! use gd32vf103_hal as hal;
//! use hal::prelude::*;
//! use hal::pac as pac;
//!
//! #[entry]
//! fn main() -> ! {
//!     // Get ownership of device peripherals
//!     let dp = pac::Peripherals::take().unwrap();
//!     // Constrain RCU register for further use
//!     let mut rcu = dp.RCU.constrain();
//!     // Split GPIOA into separate pins.
//!     // You need a mutable reference of APB2 struct to initialize GPIOA,
//!     // so we offer `&mut rcu.apb2` as a parameter here.
//!     let mut gpioa = dp.GPIOA.split(&mut rcu.apb2);
//!     // Change the state of `pa1` into push-pull output with default speed.
//!     let mut pa1 = gpioa.pa1.into_push_pull_output(&mut gpioa.ctl0);
//!     // Use API offered by `embedded-hal` to set `pa1` low,
//!     // An LED light with cathode connected to PA1 should be lit now.
//!     pa1.set_low().unwrap();
//!     // We just end this program with infinite loop.
//!     // A `wfi` instruction should be also acceptable here.
//!     loop {}
//! }
//! ```

#![no_std]
// #![deny(missing_docs)]

pub use gd32vf103_pac as pac;

pub mod ctimer;
pub mod serial;
pub mod delay;
pub mod gpio;
pub mod rcu;
pub mod spi;
pub mod time;
pub mod timer;

/// Prelude
pub mod prelude {
    pub use crate::gpio::GpioExt as _gd32vf103_hal_gpio_GpioExt;
    pub use crate::gpio::{Unlock as _gd32vf103_hal_gpio_Unlock, UpTo10MHz, UpTo2MHz, UpTo50MHz};
    pub use crate::rcu::RcuExt as _gd32vf103_hal_rcu_RcuExt;
    pub use crate::time::U32Ext as _gd32vf103_hal_time_U32Ext;
    pub use embedded_hal::digital::v2::{
        InputPin as _embedded_hal_digital_v2_InputPin,
        OutputPin as _embedded_hal_digital_v2_OutputPin,
        StatefulOutputPin as _embedded_hal_digital_v2_StatefulOutputPin,
        ToggleableOutputPin as _embedded_hal_digital_v2_ToggleableOutputPin,
    };
}
