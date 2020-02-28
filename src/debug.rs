//! Debug features

// todo: design mode hold register (instead of `pub use`)
use crate::pac::DBG;

/// Read the DBG ID code register
pub fn debug_id() -> u32 {
    // note(unsafe): read read-only register
    unsafe { &*DBG::ptr() }.id.read().bits()
}
