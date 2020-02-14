//! Timers

use gd32vf103_pac::SYSTICK;
// Somehow, weneed totake in theclocks modle so we know howfast theAHB clock is going. 
// Things we know about the systick peripheral:
//
/// Hardware timers

pub struct Systick{
}
impl Systick {
    pub fn get_systick(syst: &SYSTICK) -> u64
    {
        let hi : u32 = syst.systick1.read().bits();
        let lo : u32 = syst.systick0.read().bits();
        if hi == syst.systick1.read().bits(){
            return (hi as u64) << 32 | (lo as u64);
        } else {
            return (syst.systick1.read().bits() as u64) << 32 | (syst.systick0.read().bits() as u64);

        }
    }
}
