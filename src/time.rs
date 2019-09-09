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
    /// Hertz
    fn hz(self) -> Hertz;
    /// Kilo hertz
    fn khz(self) -> KiloHertz;
    /// Mega hertz
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

