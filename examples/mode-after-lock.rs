#![no_std]
#![no_main]

extern crate panic_halt;

use gd32vf103_hal::pac;

#[riscv_rt::entry]
unsafe fn main() -> ! {
    // Enable clock for GPIO port A
    (*pac::RCU::ptr()).apb2en.write(|w| w.paen().set_bit());
    (*pac::RCU::ptr()).apb2rst.write(|w| w.parst().set_bit());
    (*pac::RCU::ptr()).apb2rst.write(|w| w.parst().clear_bit());
    // Switch PA0 to push-pull output with 50-MHz maximum freq
    (*pac::GPIOA::ptr())
        .ctl0
        .modify(|r, w| w.bits((r.bits() & !(0b1111 << 0)) | (0b00_11 << 0)));
    // Lock port A to prevent mode configurations
    (*pac::GPIOA::ptr())
        .lock
        .write(|w| w.bits(0b0000_0000).bits(0b0001_0000).bits(0b0000_0000));
    let _ans1 = (*pac::GPIOA::ptr()).lock.read().bits();
    let _ans2 = (*pac::GPIOA::ptr()).lock.read().bits();
    // Try to switch mode for PA0 again
    (*pac::GPIOA::ptr())
        .ctl0
        .modify(|r, w| w.bits((r.bits() & !(0b1111 << 0)) | (0b01_11 << 0)));
    loop {}
}
