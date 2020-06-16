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
        (int_part + 1) as f32
    } else {
        int_part as f32
    }
}

fn init_usart() {
    // enable the peripheral clock
    unsafe {
        (*USART0::ptr()).ctl0.modify(|r, w| {
            w.bits(r.bits()).uen().clear_bit() //disable while being configured TODO could wait for TC=1?
        });

        (*RCU::ptr()).apb2en.modify(|r, w| {
            w.bits(r.bits())
                .usart0en()
                .set_bit()
                .afen()
                .set_bit()
                .paen()
                .set_bit()
        });

        (*GPIOA::ptr()).ctl1.modify(|r, w| {
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

        (*USART0::ptr()).baud.modify(|r, w| {
            w.bits(r.bits())
                .intdiv()
                .bits(int_div as u16)
                .fradiv()
                .bits(fra_div as u8)
        });

        (*USART0::ptr()).ctl2.modify(|r, w| {
            w.bits(r.bits())
                .ctsen()
                .clear_bit() //enable CTS hardware flow control
                .rtsen()
                .clear_bit() //enable RTS hardware flow control
        });

        (*USART0::ptr()).ctl1.modify(|r, w| {
            w.bits(r.bits())
                .stb()
                .bits(0b00) //set # of stop bits = 1
                .cken()
                .clear_bit()
        });

        (*USART0::ptr()).ctl0.modify(|r, w| {
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
                (*USART0::ptr()).data.write(|w| w.data().bits(byte.into()));
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

// --- //

// use crate::pac::USART0;
use crate::afio::PCF0;
use crate::gpio::gpioa::{PA10, PA11, PA12, PA8, PA9};
use crate::gpio::{Alternate, Floating, Input, PushPull};
use crate::rcu::{Clocks, APB2};
use crate::unit::{Bps, U32Ext};

/// Serial config
pub struct Config {
    pub baudrate: Bps,
    pub parity: Parity,
    pub stop_bits: StopBits,
    // pub flow_control
}

impl Default for Config {
    fn default() -> Self {
        Config {
            baudrate: 115200u32.bps(),
            parity: Parity::ParityNone,
            stop_bits: StopBits::STOP1,
        }
    }
}

impl Config {
    pub fn baudrate(mut self, baudrate: Bps) -> Config {
        self.baudrate = baudrate;
        self
    }
    
    pub fn parity(mut self, parity: Parity) -> Config {
        self.parity = parity;
        self
    }

    pub fn stop_bits(mut self, stop_bits: StopBits) -> Config {
        self.stop_bits = stop_bits;
        self
    }
}

/// Serial parity
pub enum Parity {
    /// Disable parity check
    ParityNone,
    /// Enable even parity check
    ParityEven,
    /// Enable odd parity check
    ParityOdd,
}

impl Parity {
    // (word_length, parity_enable, parity_config)
    // word_length: 0 => 8 bits; 1 => 9 bits
    // parity_enable: 0 => disable; 1 => enable
    // parity_config: 0 => odd; 1 => even
    #[inline]
    fn config(&self) -> (bool, bool, bool) {
        match *self {
            Parity::ParityNone => (false, false, false),
            Parity::ParityEven => (true, true, false),
            Parity::ParityOdd => (true, true, true),
        }
    }
}

/// Serial stop bits
pub enum StopBits {
    /// 1 stop bit
    STOP1,
    /// 0.5 stop bit
    STOP0P5,
    /// 2 stop bits
    STOP2,
    /// 1.5 stop bit
    STOP1P5,
}

impl StopBits {
    #[inline]
    fn config(&self) -> u8 {
        match *self {
            StopBits::STOP1 => 0b00,
            StopBits::STOP0P5 => 0b01,
            StopBits::STOP2 => 0b10,
            StopBits::STOP1P5 => 0b11,
        }
    }
}

/// Serial abstraction
pub struct Serial<USART, PINS> {
    usart: USART,
    pins: PINS,
}

impl<PINS> Serial<USART0, PINS> {
    /// Power on and create serial instance
    pub fn usart0(
        usart0: USART0,
        pins: PINS,
        pcf0: &mut PCF0,
        config: Config,
        clocks: Clocks,
        apb2: &mut APB2,
    ) -> Self
    where
        PINS: Bundle<USART0>,
    {
        // calculate baudrate divisor fractor
        let baud_div = {
            // use apb2 or apb1 may vary
            // round the value to get most accurate one (without float point)
            let baud_div = (clocks.ck_apb2().0 + config.baudrate.0 / 2) / config.baudrate.0;
            assert!(baud_div >= 0x0010 && baud_div <= 0xFFFF, "impossible baudrate");
            baud_div
        };
        // get parity config
        let (wl, pcen, pm) = config.parity.config();
        // get stop bit config
        let stb = config.stop_bits.config();
        riscv::interrupt::free(|_| {
            // enable and reset usart peripheral
            apb2.en().modify(|_, w| w.usart0en().set_bit());
            apb2.rst().modify(|_, w| w.usart0rst().set_bit());
            apb2.rst().modify(|_, w| w.usart0rst().clear_bit());
            // set serial remap
            pcf0.pcf0()
                .modify(|_, w| w.usart0_remap().bit(PINS::REMAP == 1));
            // does not enable DMA in this section; DMA is enabled separately
            // set baudrate
            usart0
                .baud
                .write(|w| unsafe { w.bits(baud_div) });
            // configure stop bits
            usart0.ctl1.modify(|_, w| unsafe { w.stb().bits(stb) });
            usart0.ctl0.modify(|_, w| {
                // set parity check settings
                w.wl().bit(wl).pcen().bit(pcen).pm().bit(pm);
                // enable the peripheral
                // todo: split receive and transmit
                w.uen().set_bit().ren().set_bit().ten().set_bit()
            });
        });
        Serial {
            usart: usart0,
            pins,
        }
    }

    /// Power down and return ownership of owned registers
    pub fn release(self, apb2: &mut APB2) -> (USART0, PINS) {
        // disable the peripheral
        self.usart
            .ctl0
            .modify(|_, w| w.uen().clear_bit().ren().clear_bit().ten().clear_bit());
        // disable the clock
        apb2.en().modify(|_, w| w.usart0en().clear_bit());

        // return the ownership
        (self.usart, self.pins)
    }
}

/// Serial error
#[derive(Debug)]
pub enum Error {
    /// New data frame received while read buffer is not empty. (ORERR)
    Overrun,
    /// Noise detected on the RX pin when receiving a frame. (NERR)
    Noise,
    /// RX pin is detected low during the stop bits of a receive frame. (FERR)
    Framing,
    /// Parity bit of the receive frame does not match the expected parity value. (PERR)
    Parity,
}

impl<PINS> embedded_hal::serial::Read<u8> for Serial<USART0, PINS> {
    type Error = Error;

    fn try_read(&mut self) -> nb::Result<u8, Self::Error> {
        let stat = self.usart.stat.read();
        // the chip has already filled data buffer with input data
        // check for errors present
        let err = if stat.orerr().bit_is_set() {
            Some(Error::Overrun)
        } else if stat.nerr().bit_is_set() {
            Some(Error::Noise)
        } else if stat.ferr().bit_is_set() {
            Some(Error::Framing)
        } else if stat.perr().bit_is_set() {
            Some(Error::Parity)
        } else {
            None
        };

        if let Some(err) = err {
            // error occurred, no data is read. clean the data buffer and error flags
            // note(unsafe): stateless register read
            unsafe {
                core::ptr::read_volatile(&self.usart.stat as *const _ as *const _);
                core::ptr::read_volatile(&self.usart.data as *const _ as *const _);
            }
            // returns error; no data is returned
            Err(nb::Error::Other(err))
        } else {
            // if a byte is available, return the byte; or the upstream should wait
            // until a byte is ready
            if stat.rbne().bit_is_set() {
                // read buffer non empty, return this byte
                Ok(unsafe { core::ptr::read_volatile(&self.usart.data as *const _ as *const _) })
            } else {
                // byte is not ready
                Err(nb::Error::WouldBlock)
            }
        }
    }
}

impl<PINS> embedded_hal::serial::Write<u8> for Serial<USART0, PINS> {
    type Error = core::convert::Infallible; // !

    fn try_write(&mut self, byte: u8) -> nb::Result<(), Self::Error> {
        let stat = self.usart.stat.read();

        if stat.tbe().bit_is_set() {
            // NOTE(unsafe) atomic write to stateless register
            // impossible using PAC only to write u8 value
            unsafe {
                // compiles into `lui a?, %hi(USART_DATA); sb a??, %lo(USART_DATA)(a?)`
                core::ptr::write_volatile(&self.usart.data as *const _ as *mut _, byte)
            }
            Ok(())
        } else {
            // upstream should wait until end of transmit
            Err(nb::Error::WouldBlock)
        }
    }

    fn try_flush(&mut self) -> nb::Result<(), Self::Error> {
        // if translate completed, do not wait
        if self.usart.stat.read().tc().bit_is_set() {
            Ok(())
        } else {
            // otherwise upstream should wait
            Err(nb::Error::WouldBlock)
        }
    }
}

impl<PINS> core::fmt::Write for Serial<USART0, PINS> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        use embedded_hal::serial::Write;
        s.as_bytes()
            .iter()
            .try_for_each(|c| nb::block!(self.try_write(*c)))
            .map_err(|_| core::fmt::Error) // no write error is possible
    }
}

// /// IrDA Config
// pub struct IrConfig {
//     /// If IrDA low power mode should be enabled
//     pub low_power: bool,
//     /// Serial baudrate
//     pub baudrate: Bps,
//     /// Serial parity
//     pub parity: Parity,
// }

// /// Infrared Data Association (IrDA) communication abstraction
// pub struct IrDA<USART, PINS> {
//     usart: USART,
//     pins: PINS,
// }

// impl<PINS> IrDA<USART0, PINS> {
//     /// Power on and create IrDA instance
//     #[doc(hidden)]
//     pub fn usart0(
//         usart0: USART0,
//         pins: PINS,
//         pcf0: &mut PCF0,
//         clocks: Clocks,
//         apb2: &mut APB2,
//     ) -> Self
//     where
//         PINS: Bundle<USART0>,
//     {
//         todo!("actual power up process");
//         Self {
//             usart: usart0,
//             pins,
//         }
//     }

//     /// Power down and return ownership of owned registers
//     #[doc(hidden)]
//     pub fn release(self) -> (USART0, PINS) {
//         todo!("actual power down");
//         (self.usart, self.pins)
//     }
// }

pub trait Pins {
    // private::Sealed; internal use only
    #[doc(hidden)]
    const REMAP: u8;

    type TX;

    type RX;

    type RTS;

    type CTS;

    type CK;
}

impl Pins for USART0 {
    const REMAP: u8 = 0;

    type TX = PA9<Alternate<PushPull>>;
    type RX = PA10<Input<Floating>>;
    // todo: mode of PA12, PA11 and P8
    type RTS = PA12<Alternate<PushPull>>;
    type CTS = PA11<Alternate<PushPull>>;
    type CK = PA8<Alternate<PushPull>>;
}

// TX, RX, RTS, CTS

pub trait Bundle<USART: Pins> {
    #[doc(hidden)]
    const REMAP: u8 = USART::REMAP;
    #[doc(hidden)]
    fn enable_ctl0();
    #[doc(hidden)]
    fn enable_ctl2();
}

impl<USART: Pins> Bundle<USART> for (USART::TX, USART::RX) {
    #[inline]
    fn enable_ctl0() {
        // w.ren().set_bit().ten().set_bit()
    }
    #[inline]
    fn enable_ctl2() {
        // w.rtsen().clear_bit().ctsen().clear_bit()
    }
}

impl<USART: Pins> Bundle<USART> for (USART::TX, USART::RX, USART::RTS, USART::CTS) {
    #[inline]
    fn enable_ctl0() {
        // w.ren().set_bit().ten().set_bit()
    }
    #[inline]
    fn enable_ctl2() {
        // w.rtsen().set_bit().ctsen().set_bit()
    }
}

//todo
