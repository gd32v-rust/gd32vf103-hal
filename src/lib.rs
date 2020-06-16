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
//! // choose a panic handler crate
//! extern crate panic_halt;
//! // include this library
//! use gd32vf103_hal::{pac, prelude::*};
//! // use the `riscv_rt` runtime to define entry
//! #[riscv_rt::entry]
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

pub mod adc;
pub mod afio;
pub mod backup;
pub mod crc;
pub mod ctimer;
pub mod debug;
pub mod delay;
pub mod esig;
pub mod fmc;
pub mod gpio;
pub mod rcu;
pub mod serial;
pub mod spi;
pub mod timer;
pub mod unit;
pub mod wdog;

/// Prelude
pub mod prelude {
    pub use embedded_hal::prelude::*;
    pub use crate::afio::AfioExt as _gd32vf103_hal_afio_AfioExt;
    pub use crate::gpio::GpioExt as _gd32vf103_hal_gpio_GpioExt;
    pub use crate::gpio::{Unlock as _gd32vf103_hal_gpio_Unlock, UpTo10MHz, UpTo2MHz, UpTo50MHz};
    pub use crate::rcu::RcuExt as _gd32vf103_hal_rcu_RcuExt;
    pub use crate::unit::U32Ext as _gd32vf103_hal_unit_U32Ext;
}

mod atomic {
    use core::sync::atomic::{AtomicU32, Ordering};
    // This function uses AtomicU32, compiles into atomic instructions to prevent data race
    // and optimize for speed.
    //
    // If we don't do like this, we would need to go into critical section, where additional
    // interrupt disabling and enabling operations are required, which needs lots of CSR
    // read/write instructions and costs lot of time.
    //
    // For all `is_one: true` params, the core feature of this function compiles into
    // only one atomic instruction `amoor.w` to set the target register.
    // (For `is_one: false` params, it compiles into ont `amoand.w`).
    // Additional instructions to set the mask may differ between actual applications,
    // this part may cost additional one to two instructions (mainly `lui` and `addi`).
    //
    // Note: we uses `fetch_and(!mask, ...)` instead of `fetch_nand(mask, ...)`; that's
    // because RISC-V's RV32A does not provide an atomic nand instruction, thus `rustc`
    // may compile code into very long binary output.
    #[inline(always)]
    pub(crate) fn atomic_set_bit(r: &AtomicU32, is_one: bool, index: usize) {
        let mask = 1 << index;
        if is_one {
            r.fetch_or(mask, Ordering::Relaxed);
        } else {
            r.fetch_and(!mask, Ordering::Relaxed);
        }
    }

    // This function compiles into RV32A's `amoxor.w` instruction to prevent data
    // race as well as optimize for speed.
    #[inline(always)]
    pub(crate) fn atomic_toggle_bit(r: &AtomicU32, index: usize) {
        let mask = 1 << index;
        r.fetch_xor(mask, Ordering::Relaxed);
    }
}

// == Notes on prelude trait function naming:
//
// If we wrap some register modules into one Rust `mod`, we infer that
// all the modules share common switches, clocks or unlock process.
//
// To take apart whole module into functional module register groups,
// we use traits with one function. Function name can be arbitrary in
// theory but we prefer following frequent function names:
// - split
// - constrain
// - configure
//
// The function name should depends on how the modules logically effect
// each other:
//
// If logical state of module registers do not depend on each other,
// the trait function name could be `split`.
//
// If logical states of module registers is under inherit or hierarchy
// relation thus may depend on each other, name could be `constain`
// or `configure`. If all combination of register states are valid,
// use `configure`; otherwise if some combinations are invalid, use
// `constrain`.
