//! Timers

use gd32vf103_pac::CTIMER;

// This right now just gets the systick register, system timer, or mtime.
// I believe system timer is the correct name, as the documentation seems to
// imply mtimer is instruction count, while the system timer increments on
// clock pulses.
/// CTIMER
/// todo: A more proper name?
pub struct SysTick;

impl SysTick {
    pub fn get_systick(ctimer: &CTIMER) -> u64 {
        // Hi is systick1
        let hi: u32 = ctimer.mtime_hi.read().bits();
        let lo: u32 = ctimer.mtime_lo.read().bits();
        if hi == ctimer.mtime_hi.read().bits() {
            return (hi as u64) << 32 | (lo as u64);
        } else {
            return (ctimer.mtime_hi.read().bits() as u64) << 32
                | (ctimer.mtime_lo.read().bits() as u64);
        }
    }
}
