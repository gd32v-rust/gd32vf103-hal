//! Serial Peripheral Interface (SPI) bus

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
