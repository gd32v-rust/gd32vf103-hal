//! Measurement units

/// Hertz
#[derive(Clone, Copy)]
pub struct Hertz(pub u32);

/// Kilo hertz
#[derive(Clone, Copy)]
pub struct KiloHertz(pub u32);

/// Mega hertz
#[derive(Clone, Copy)]
pub struct MegaHertz(pub u32);

/// Extension trait that add convenient methods to the `u32` type
pub trait U32Ext {
    /// Wrap in `Hertz`
    fn hz(self) -> Hertz;
    /// Wrap in `KiloHertz`
    fn khz(self) -> KiloHertz;
    /// Wrap in `MegaHertz`
    fn mhz(self) -> MegaHertz;
    /// Wrap in `MilliSeconds`
    fn ms(self) -> MilliSeconds;
    /// Wrap in `MicroSeconds`
    fn us(self) -> MicroSeconds;
    /// Wrap in `Bps`
    fn bps(self) -> Bps;
}

impl U32Ext for u32 {
    fn hz(self) -> Hertz {
        Hertz(self)
    }

    fn khz(self) -> KiloHertz {
        KiloHertz(self)
    }

    fn mhz(self) -> MegaHertz {
        MegaHertz(self)
    }

    fn ms(self) -> MilliSeconds {
        MilliSeconds(self)
    }

    fn us(self) -> MicroSeconds {
        MicroSeconds(self)
    }

    fn bps(self) -> Bps {
        Bps(self)
    }
}

impl Into<Hertz> for KiloHertz {
    fn into(self) -> Hertz {
        Hertz(self.0 * 1_000)
    }
}

impl Into<Hertz> for MegaHertz {
    fn into(self) -> Hertz {
        Hertz(self.0 * 1_000_000)
    }
}

impl Into<KiloHertz> for MegaHertz {
    fn into(self) -> KiloHertz {
        KiloHertz(self.0 * 1_000)
    }
}

/// Milliseconds
pub struct MilliSeconds(pub u32);

// todo: there's no need for accurate time units by now
/// Microseconds
pub struct MicroSeconds(pub u32);

impl Into<MicroSeconds> for MilliSeconds {
    fn into(self) -> MicroSeconds {
        MicroSeconds(self.0 * 1_000)
    }
}

/// Bits per second
pub struct Bps(pub u32);
