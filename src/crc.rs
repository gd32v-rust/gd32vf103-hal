//! CRC calculation unit
//!
//! The hardware cyclic redundancy check (CRC) unit on GD32VF103 has 32-bit data
//! input and 32-bit data output. Calculation period is 4 AHB clock cycles
//! for 32-bit input data size, from data entered to the calculation result
//! available.
//!
//! This unit uses fixed polynomial 0x4C11DB7, which is a common polynomial
//! used in Ethernet.
//!
//! Ref: Section 8, the User Manual; Firmware/Source/gd32vf103_crc.c
//!
//! # Usage
//!
//! ## CRC calculation
//!
//! To use this module, create a [`Crc`] wrapper using [`Crc::crc`] function. This
//! function requires peripheral CRC and ownership of AHB peripheral bus; it turns
//! on the CRC clock and creates an owned `Crc` wrapper.
//! After the wrapper is created, you need to create a [`Digest`] struct.
//! The function [`crc.new_digest()`] will clear the underlying CRC buffer and return
//! an owned Digest struct to get ready for calculation.
//! With the Digest struct, you may keep writing `u32` values with function
//! [`digest.write_u32(value)`]. To read the digest value out, use [`digest.finish()`].
//! Further values can still be written after the digest value is read out.
//! After all the processes, you may stop writing to digests and get the `Crc` wrapper
//! back with function [`digest.free()`].
//! To release and turn off the clock to get CRC peripheral back, use [`crc.release()`].
//!
//! [`Crc`]: struct.Crc.html
//! [`Crc::crc`]: struct.Crc.html#method.crc
//! [`crc.new_digest()`]: struct.Crc.html#method.new_digest
//! [`Digest`]: struct.Digest.html
//! [`digest.write_u32(value)`]: struct.Digest.html#method.write_u32
//! [`digest.finish()`]: struct.Digest.html#method.finish
//! [`digest.free()`]: struct.Digest.html#method.free
//! [`crc.release()`]: struct.Crc.html#method.release
//! 
//! ## Free data register `fdata`
//! 
//! The `fdata` register provides you with additional 8-bit global storage. You may read 
//! from or write to this register using function [`fdata_read`] and [`fdata_write`].
//!
//! [`fdata_read`]: fn.fdata_read.html
//! [`fdata_write`]: fn.fdata_write.html
//! 
//! # Example
//!
//! This example is tested on GD32VF103C-START board. It calculated the CRC value of
//! `0xABCD1234`. The desired result is `0xF7018A40`; if the underlying hardware had
//! calculated correctly, PA7 is set high to turn on the LED with anode connected to
//! the MCU.
//!
//! ```
//! #![no_std]
//! #![no_main]
//!
//! use gd32vf103_hal as hal;
//! use hal::{crc::Crc, pac, prelude::*};
//! use panic_halt as _;
//!
//! #[riscv_rt::entry]
//! fn main() -> ! {
//!     let dp = pac::Peripherals::take().unwrap();
//!     let mut rcu = dp.RCU.constrain();
//!     let mut gpioa = dp.GPIOA.split(&mut rcu.apb2);
//!     let mut pa7 = gpioa.pa7.into_push_pull_output(&mut gpioa.ctl0);
//!
//!     let src: u32 = 0xABCD1234;
//!     let crc = Crc::crc(dp.CRC, &mut rcu.ahb);
//!     let mut digest = crc.new_digest();
//!     digest.write_u32(src);
//!
//!     if digest.finish() == 0xF7018A40 {
//!         pa7.set_high().unwrap();
//!     }
//!
//!     loop {}
//! }
//! ```

use crate::pac::CRC;
use crate::rcu::AHB;

/// Read the value of the free data register `fdata`.
#[inline]
pub fn fdata_read() -> u8 {
    // note(unsafe): separate ownership, volatile read
    unsafe { &*CRC::ptr() }.fdata.read().fdata().bits()
}

/// Write data to the free data register `fdata`.
#[inline]
pub fn fdata_write(byte: u8) {
    // note(unsafe): separate ownership, volatile write
    // for high 24 bits we may keep reset value
    unsafe { &*CRC::ptr() }
        .fdata
        .modify(|_, w| unsafe { w.fdata().bits(byte) });
}

/// CRC module abstraction
///
/// Owns `CRC_DATA` and `CRC_CTL`.
pub struct Crc {
    crc: CRC,
}

impl Crc {
    /// Take ownership of CRC and enable CRC clock.
    /// 
    /// To create struct `Crc`, it's need to pass the peripheral `CRC` and a mutable
    /// reference of `AHB` peripheral bus. Get the ownership of `CRC` from the device 
    /// peripheral struct `pac::Peripherals` (may already exists in your code) as `dp` 
    /// then use `dp.CRC`. To get AHB you need to constrain `RCU` module using 
    /// `dp.RCU.constrain()`, then use `&mut rcu.ahb` to get its mutable reference.
    /// 
    /// # Examples
    /// 
    /// Basic usage:
    /// 
    /// ```no_run
    /// // Prepare CRC peripheral for calculation
    /// let crc = Crc::crc(dp.CRC, &mut rcu.ahb);
    /// ```
    #[inline]
    pub fn crc(crc: CRC, ahb: &mut AHB) -> Self {
        ahb.en().modify(|_, w| w.crcen().set_bit());
        Crc { crc }
    }

    /// Create new Digest struct for CRC calculation.
    /// 
    /// The underlying CRC buffer is cleaned to prepare for incoming values. Write
    /// operations could be later performed using functions in `Digest`. You may
    /// refer to [`digest.write_u32(value)`] for how to write the value for CRC
    /// calculation.
    /// 
    /// [`digest.write_u32(value)`]: struct.Digest.html#method.write_u32
    /// 
    /// # Examples
    /// 
    /// Basic usage:
    /// 
    /// ```no_run
    /// // Prepare CRC peripheral for calculation
    /// let crc = Crc::crc(dp.CRC, &mut rcu.ahb);
    /// // Create a Digest instance to write values for calculation
    /// let mut digest = crc.new_digest();
    /// ```
    #[inline]
    pub fn new_digest(self) -> Digest {
        self.crc.ctl.modify(|_, w| w.rst().set_bit());
        // after initialization finished, hardware would set `rst` bit to `false`.
        while self.crc.ctl.read().rst() == true {}
        Digest { crc: self.crc }
    }

    /// Disable the CRC clock and release the peripheral.
    /// 
    /// The clock is switched off using AHB; you must provide a mutable referrence
    /// of AHB to release the CRC peripheral. After release, the peripheral is freed
    /// for further use.
    /// 
    /// # Examples
    /// 
    /// Basic usage:
    /// 
    /// ```no_run
    /// // Prepare CRC peripheral for calculation
    /// let crc = Crc::crc(dp.CRC, &mut rcu.ahb);
    /// // ... actual calculations with `crc`
    /// // Release the wrapper and get CRC peripheral back
    /// let crc = crc.release(&mut rcu.ahb);
    /// ```
    #[inline]
    pub fn release(self, ahb: &mut AHB) -> CRC {
        ahb.en().modify(|_, w| w.crcen().clear_bit());
        self.crc
    }
}

/// Digest struct for CRC calculation.
/// 
/// This struct is created using [`Crc::new_digest`] function. Use [`write_u32`]
/// function to write data for calculation; use [`finish`] to read the result.
/// After calculation, use [`free`] to get the Crc wrapper back.
/// 
/// [`Crc::new_digest`]: ./struct.Crc.html#method.new_digest
/// [`write_u32`]: #method.write_u32
/// [`finish`]: #method.finish
/// [`free`]: #method.free
/// 
/// # Examples
/// 
/// Calculate CRC result of single value:
/// 
/// ```no_run
/// // Write a single value
/// digest.write_u32(0x12345678);
/// // Read its CRC calculation result
/// let ans = digest.finish();
/// ```
/// 
/// Calculate CRC reuslt of an array of values:
/// 
/// ```no_run
/// // Write all values of an array
/// for value in array {
///     digest.write_u32(value);
/// }
/// // Read the CRC calculation result of this array
/// let ans = digest.finish();
/// ```
pub struct Digest {
    crc: CRC,
}

impl Digest {
    /// Writes a single u32 into this hasher.
    /// 
    /// Multiple u32 values may be written one by one using this function. 
    /// After all values written for calculation, you may read the CRC result 
    /// using function [`finish`].
    /// 
    /// [`finish`]: #method.finish
    /// 
    /// # Examples
    /// 
    /// Write a single value:
    /// 
    /// ```no_run
    /// // Write a single value
    /// digest.write_u32(0x12345678);
    /// ```
    /// 
    /// Write an array of values:
    /// 
    /// ```no_run
    /// // Write all values of an array
    /// for value in array {
    ///     digest.write_u32(value);
    /// }
    /// ```
    #[inline]
    pub fn write_u32(&mut self, i: u32) {
        self.crc.data.write(|w| unsafe { w.data().bits(i) });
    }

    /// Returns the hash value for the values written so far.
    /// 
    /// # Examples
    /// 
    /// Get CRC reuslt of a single value:
    /// 
    /// ```no_run
    /// // Write a single value
    /// digest.write_u32(0x12345678);
    /// // Read its CRC calculation result
    /// let ans = digest.finish();
    /// ```
    /// 
    /// Get CRC reuslt of an array of values:
    /// 
    /// ```no_run
    /// // Write all values of an array
    /// for value in array {
    ///     digest.write_u32(value);
    /// }
    /// // Read the CRC calculation result of this array
    /// let ans = digest.finish();
    /// ```
    #[inline]
    pub fn finish(&self) -> u32 {
        self.crc.data.read().data().bits()
    }

    /// Frees the Digest struct to return the underlying Crc peripheral.
    /// 
    /// # Examples
    /// 
    /// Basic usage:
    /// 
    /// ```no_run
    /// // Calculate CRC of a single value
    /// digest.write_u32(0x12345678);
    /// let ans = digest.finish();
    /// // Free the Digest to get the CRC wrapper back
    /// let crc = digest.free();
    /// ```
    #[inline]
    pub fn free(self) -> Crc {
        Crc { crc: self.crc }
    }
}
