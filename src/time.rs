//! Time units

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
