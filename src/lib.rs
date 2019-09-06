pub use gd32vf103_pac as pac;

pub mod gpio;

pub mod prelude {
    pub use crate::gpio::GpioExt as _gd32vf103_hal_gpio_GpioExt;
}
