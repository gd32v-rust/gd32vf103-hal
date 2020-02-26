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
//! Ref: Section 8, the User Manual
//! 
//! todo: verify this module

use crate::pac::CRC;
use crate::rcu::AHB;

/// CRC module abstraction.
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

pub struct Digest {
    crc: CRC,
}

impl Digest {
    pub fn write_u32(&mut self, i: u32) {
        self.crc.data.write(|w| unsafe { w.data().bits(i) });
        // todo: wait for 4 AHB cycles?
    }

    pub fn finish(self) -> (u32, Crc) {
        let ans = self.crc.data.read().data().bits();
        (ans, Crc { crc: self.crc })
    }
}
