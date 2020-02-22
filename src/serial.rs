//! (TODO) Serial Communication (USART)

#![macro_use]

use crate::pac;
use core::fmt::Write;
use pac::{GPIOA, RCU, USART0};

//TODO - use the APB/RCU/GPIO primitives in this crate, rather than unsafe memory poking!

// yay, math functions arn't implemented in core!
fn round(n: f32) -> f32 {
    let int_part: i32 = n as i32; //truncate
    let fraction_part: f32 = n - int_part as f32;
    if fraction_part >= 0.5 {
        return (int_part + 1) as f32;
    } else {
        return int_part as f32;
    }
}

fn init_usart() {
    // enable the peripheral clock
    unsafe {
        &(*USART0::ptr()).ctl0.modify(|r, w| {
            w.bits(r.bits()).uen().clear_bit() //disable while being configured TODO could wait for TC=1?
        });

        &(*RCU::ptr()).apb2en.modify(|r, w| {
            w.bits(r.bits())
                .usart0en()
                .set_bit()
                .afen()
                .set_bit()
                .paen()
                .set_bit()
        });

        &(*GPIOA::ptr()).ctl1.modify(|r, w| {
            w.bits(r.bits())
                .md9()
                .bits(0b11) //output, 50mhz
                .ctl9()
                .bits(0b10) //alternate push pull
                .md10()
                .bits(0b00) //input
                .ctl10()
                .bits(0b01) //floating
        });

        // for 9600 baud rate @ 8mhz clock, USARTDIV = CLK/(16*baud)
        // USARTDIV = 8000000/(16*9600) = 52.0833333
        // integer part = 52, fractional ~= 1/16 -> intdiv=53, fradiv=1
        // can calculate automatically given clk and baud, but note that
        // if fradiv=16, then intdiv++; fradiv=0;

        let _baud = 9600f32;
        let clk_freq = 8_000_000f32;
        let usart_div = clk_freq / (16f32 * 9600f32);
        let mut int_div = usart_div as i32; //note that trunc(), fract(), rount() are not implemented in core...
        let mut fra_div = round(16.0 * (usart_div - int_div as f32)) as i32;
        if fra_div == 16 {
            int_div += 1;
            fra_div = 0;
        }

        &(*USART0::ptr()).baud.modify(|r, w| {
            w.bits(r.bits())
                .intdiv()
                .bits(int_div as u16)
                .fradiv()
                .bits(fra_div as u8)
        });

        &(*USART0::ptr()).ctl2.modify(|r, w| {
            w.bits(r.bits())
                .ctsen()
                .clear_bit() //enable CTS hardware flow control
                .rtsen()
                .clear_bit() //enable RTS hardware flow control
        });

        &(*USART0::ptr()).ctl1.modify(|r, w| {
            w.bits(r.bits())
                .stb()
                .bits(0b00) //set # of stop bits = 1
                .cken()
                .clear_bit()
        });

        &(*USART0::ptr()).ctl0.modify(|r, w| {
            w.bits(r.bits())
                .wl()
                .clear_bit() //set word size to 8
                .ten()
                .set_bit() //enable tx
                .ren()
                .set_bit() //enable rx
                .pcen()
                .clear_bit() //no parity check function plz
                .pm()
                .clear_bit() //0=even parity 1=odd parity
                .uen()
                .set_bit() //enable the uart, yay!
        });
    }
}

/// todo: more proper name
#[doc(hidden)] // experimental, not for practical use
pub struct SerialWrapper;

impl core::fmt::Write for SerialWrapper {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for &byte in s.as_bytes() {
            unsafe {
                &(*USART0::ptr()).data.write(|w| w.data().bits(byte.into()));
                while (*USART0::ptr()).stat.read().tbe().bit_is_clear() {}
            }
        }
        Ok(())
    }
}

// hold things in a static place
static mut STDOUT: Option<SerialWrapper> = None;

#[allow(unused_variables)]
#[doc(hidden)] // experimental, not for practical use
pub fn init_stdout(uart: USART0) {
    init_usart();
    unsafe {
        STDOUT.replace(SerialWrapper {});
    }
}

/// Writes string to stdout
#[doc(hidden)] // experimental, not for practical use
pub fn write_str(s: &str) {
    unsafe {
        if let Some(stdout) = STDOUT.as_mut() {
            let _ = stdout.write_str(s);
        } else {
            panic!("couldn't get stdout!");
        }
    }
}

/// Writes formatted string to stdout
#[doc(hidden)] // experimental, not for practical use
pub fn write_fmt(args: core::fmt::Arguments) {
    unsafe {
        if let Some(stdout) = STDOUT.as_mut() {
            let _ = stdout.write_fmt(args);
        } else {
            panic!("couldn't get stdout!");
        }
    }
}

/// Macro for printing to the serial standard output
#[doc(hidden)] // experimental
#[macro_export]
macro_rules! sprint {
    ($s:expr) => {
        crate::serial::write_str($s)
    };
    ($($tt:tt)*) => {
        crate::serial::write_fmt(format_args!($($tt)*))
    };

}
