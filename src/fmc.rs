//! (TODO) Flash Memory Controller (FMC)

// this module maybe used in on-air firmware update programs (OTA)
// or store user defined important data
use crate::pac::FMC;

#[doc(hidden)] // not finished
pub struct Fmc {
    fmc: FMC,
}

impl Fmc {
    pub fn new(fmc: FMC) -> Self {
        Fmc { fmc }
    }

    pub fn free(self) -> FMC {
        self.fmc
    }
}
