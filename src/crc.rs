//! (TODO) CRC calculation unit
//!
//! The cyclic redundancy check (CRC) unit on GD32VF103 has 32-bit data
//! input and 32-bit data output. Calculation period is 4 AHB clock cycles
//! for 32-bit input data size from data entered to the calculation result
//! available.
//!
//! This unit uses fixed polynomial 0x4C11DB7, which is a common polynomial
//! used in Ethernet.
//!
//! Ref: Section 8, the User Manual; Firmware/Source/gd32vf103_crc.c
//!
//! todo: verify this module

use crate::pac::CRC;
use crate::rcu::AHB;

/// Read the value of the free data register `fdata`.
pub fn fdata_read() -> u8 {
    // note(unsafe): separate ownership, volatile read
    unsafe { &*CRC::ptr() }.fdata.read().fdata().bits()
}

/// Write data to the free data register `fdata`.
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
    pub fn crc(crc: CRC, ahb: &mut AHB) -> Self {
        ahb.en().modify(|_, w| w.crcen().set_bit());
        Crc { crc }
    }

    /// Create new Digest struct for CRC calculation
    pub fn new_digest(self) -> Digest {
        self.crc.ctl.modify(|_, w| w.rst().set_bit());
        // after initialization finished, hardware would set `rst` bit to `false`.
        while self.crc.ctl.read().rst() == true {}
        Digest { crc: self.crc }
    }

    /// Disable the CRC clock and release the peripheral.
    pub fn release(self, ahb: &mut AHB) -> CRC {
        ahb.en().modify(|_, w| w.crcen().clear_bit());
        self.crc
    }
}

/// Digest struct for CRC calculation
pub struct Digest {
    crc: CRC,
}

impl Digest {
    /// Writes a single u32 into this hasher.
    pub fn write_u32(&mut self, i: u32) {
        self.crc.data.write(|w| unsafe { w.data().bits(i) });
    }

    /// Returns the hash value for the values written so far.
    pub fn finish(&self) -> u32 {
        self.crc.data.read().data().bits()
    }

    /// Frees the Digest struct to return the underlying Crc peripheral.
    pub fn free(self) -> Crc {
        Crc { crc: self.crc }
    }
}
