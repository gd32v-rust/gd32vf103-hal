//! Device electronic signature
//!
//! Ref: Section 1.5, GD32VF103 User Manual
//!
//! TODO: verify unique_id in this module
// should this module be named `signature`? this name may be too long

const UNIQUE_ID: *const [u32; 3] = 0x1FFF_F7E8 as *const _;
const MEMORY_DENSITY: *const u16 = 0x1FFF_F7E0 as *const _;

/// Factory programed unique device id.
/// 
/// This field if 96 bits wide. It may be only read using 32-bit load
/// procedures.
///
/// According to section 1.5.2 of the Manual, this value
/// can never be altered by user.
#[inline]
pub fn unique_id() -> &'static [u32; 3] {
    // note(unsafe): static read-only value
    unsafe { &*UNIQUE_ID }
}

/// Flash memory density in KBytes.
///
/// This value indicates the flash memory density of the device in KBytes.
/// For example, `0x0020` means 32 KBytes.
///
/// Ref: Section 1.5.1, the Manual
#[inline]
pub fn flash_density() -> u16 {
    // note(unsafe): static read-only value
    unsafe { *MEMORY_DENSITY } // read bits [15:0]
}

/// On-chip SRAM density in KBytes.
///
/// This value indicates the on-chip SRAM density of the device in KBytes.
/// For example, `0x0008` means 8 KBytes.
///
/// Ref: Section 1.5.1, the Manual
#[inline]
pub fn sram_density() -> u16 {
    // note(unsafe): static read-only value
    unsafe { *(MEMORY_DENSITY.add(1)) } // read bits [31:16]
}
