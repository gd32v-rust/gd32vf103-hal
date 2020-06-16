//! Serial Peripheral Interface (SPI) bus
use crate::gpio::gpioa::*;
use crate::gpio::gpiob::*;
use crate::gpio::{Alternate, Floating, Input, Output, PushPull};
use crate::pac::{SPI0, SPI1, SPI2};
use crate::rcu::{Clocks, APB1, APB2};
use crate::unit::Hertz;
use embedded_hal::blocking::spi::*;
use embedded_hal::spi::{FullDuplex, Mode, Phase, Polarity};

/// SPI error
#[derive(Debug)]
pub enum Error {
    /// Configuration fault error
    ConfigFault,
    /// Rx overrun error
    ReceiveOverrun,
    /// TI mdode format error
    Format,
    /// CRC error
    Crc,
}

/// SPI object that can be used to make FullDuplex SPI peripherals
pub struct Spi<SPI, PINS> {
    spi: SPI,
    pins: PINS,
}

#[doc(hidden)]
mod private {
    pub trait Sealed {}
}

pub trait SckPin<SPI>: private::Sealed {}
pub trait MisoPin<SPI>: private::Sealed {}
pub trait MosiPin<SPI>: private::Sealed {}
pub trait NssPin<SPI>: private::Sealed {}

macro_rules! pins {
    ($spi:ident, SCK: [$($sck:ident),*], MISO: [$($miso:ident),*], MOSI: [$($mosi:ident),*], NSS: [$($nss:ident),*]) => {
        $(
            impl private::Sealed for $sck<Alternate<PushPull>> {}
            impl SckPin<$spi> for $sck<Alternate<PushPull>> {}
        )*
        $(
            impl private::Sealed for $miso<Input<Floating>> {}
            impl MisoPin<$spi> for $miso<Input<Floating>> {}
        )*
        $(
            impl private::Sealed for $mosi<Alternate<PushPull>> {}
            impl MosiPin<$spi> for $mosi<Alternate<PushPull>> {}
        )*
        $(
            impl private::Sealed for $nss<Alternate<PushPull>> {}
            impl NssPin<$spi> for $nss<Alternate<PushPull>> {}
            impl private::Sealed for $nss<Output<PushPull>> {}
            impl NssPin<$spi> for $nss<Output<PushPull>> {}
        )*
    }

}

macro_rules! spi {
    ($($SPIX:ident: ($spiX:ident, $APBX:ident, $spiXen:ident, $spiXrst:ident, $pclkX:ident),)+) => {
        $(
            impl<SCK, MISO, MOSI, NSS> Spi<$SPIX, (SCK, MISO, MOSI, NSS)> {
                /// Configures the SPI peripheral to operate in full duplex master mode
                pub fn $spiX<F>(
                    spi: $SPIX,
                    pins: (SCK, MISO, MOSI, NSS),
                    mode: Mode,
                    freq: F,
                    clocks: Clocks,
                    apb: &mut $APBX,
                ) -> Self
                where
                    F: Into<Hertz>,
                    SCK: SckPin<$SPIX>,
                    MISO: MisoPin<$SPIX>,
                    MOSI: MosiPin<$SPIX>,
                    NSS: NssPin<$SPIX>
                {

                    let prescaler_bits = match clocks.$pclkX().0 / freq.into().0 {
                        0 => unreachable!(),
                        2..=2 => 0b000,
                        4..=5 => 0b001,
                        8..=11 => 0b010,
                        16..=23 => 0b011,
                        32..=39 => 0b100,
                        64..=95 => 0b101,
                        128..=191 => 0b110,
                        _ => 0b111,
                    };

                    apb.en().modify(|_,w| w.$spiXen().set_bit());
                    //apb.rst().write(|w| w.$spiXrst().set_bit());
                    //apb.rst().write(|w| w.$spiXrst().clear_bit());

                    spi.ctl0.write(|w| w.spien().clear_bit()); //disable while configuring...
                    spi.ctl1.modify(|_,w| w.nssdrv().clear_bit()); //let application drive the nss pin.
                    unsafe { //unsafe because of call to psc().bits(...)
                        spi.ctl0.modify(|_,w| {
                            w
                                .bden().clear_bit() //bidirectional
                                .ff16().clear_bit() // 8 bit word size
                                .ro().clear_bit() //not read-only
                                .psc().bits(prescaler_bits)
                                .swnssen().clear_bit() // use hardware nss mode. ??
                                .swnss().clear_bit()
                                .lf().clear_bit() //MSB first
                                .mstmod().set_bit() //master mode
                                .ckpl().bit(mode.polarity == Polarity::IdleHigh)
                                .ckph().bit(mode.phase == Phase::CaptureOnSecondTransition)
                                .spien().set_bit()
                        });
                    }


                    Spi { spi, pins }
                }

                /// Releases the SPI peripheral and associated pins
                pub fn free(self) -> ($SPIX, (SCK, MISO, MOSI, NSS)) {
                    (self.spi, self.pins)
                }
            }

            impl<PINS> FullDuplex<u8> for Spi<$SPIX, PINS> {
                type Error = Error;

                fn try_read(&mut self) -> nb::Result<u8, Error> {
                    if self.spi.stat.read().rbne().bit_is_clear() {
                        Err(nb::Error::WouldBlock)
                    } else {
                        let rx_byte = self.spi.data.read().spi_data().bits();
                        Ok(rx_byte as u8)
                    }
                }

                fn try_send(&mut self, byte: u8) -> nb::Result<(), Error> {
                    if self.spi.stat.read().tbe().bit_is_clear() {
                        Err(nb::Error::WouldBlock)
                    } else {
                        self.spi.data.write(|w|{
                            unsafe{
                                w.spi_data().bits(byte.into())
                            }
                        });
                        Ok(())
                    }
                }
            }

            impl<PINS> transfer::Default<u8> for Spi<$SPIX, PINS> {}
            impl<PINS> write::Default<u8> for Spi<$SPIX, PINS> {}
        )+
    }
}

pins! {SPI0,
    SCK: [PA5], //TODO leaving off alternate AFIO pins, due to conflicting Sealed trait impls
    MISO: [PA6],
    MOSI: [PA7],
    NSS: [PA4]
}

pins! {SPI1,
    SCK: [PB13],
    MISO: [PB14],
    MOSI: [PB15],
    NSS: [PB12]
}

pins! {SPI2,
    SCK: [PB3],
    MISO: [PB4],
    MOSI: [PB5],
    NSS: [PA15]
}

spi! {
    SPI0: (spi0, APB2, spi0en, spi0rst, ck_apb2),
    SPI1: (spi1, APB1, spi1en, spi1rst, ck_apb1),
    SPI2: (spi2, APB1, spi2en, spi2rst, ck_apb1),
}
