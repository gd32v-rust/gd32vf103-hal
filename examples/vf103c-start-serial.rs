// vf103c-start-serial.rs

#![no_std]
#![no_main]

use gd32vf103_hal as hal;
use hal::{pac, prelude::*, rcu, serial::{Serial, Config}};
use panic_halt as _;
use nb::block;

#[riscv_rt::entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let mut rcu = dp.RCU.constrain();
    let mut afio = dp.AFIO.split(&mut rcu.apb2);

    let mut gpioa = dp.GPIOA.split(&mut rcu.apb2);
    let pa9 = gpioa.pa9.into_alternate_push_pull(&mut gpioa.ctl1);
    let pa10 = gpioa.pa10.into_floating_input(&mut gpioa.ctl1);

    let clocks = rcu::Strict::new()
        .use_hxtal(8.mhz())
        .ck_sys(8.mhz())
        .freeze(&mut rcu.cfg);

    let mut serial = Serial::usart0(
        dp.USART0,
        (pa9, pa10),
        &mut afio.pcf0,
        Config::default().baudrate(9600.bps()),
        clocks,
        &mut rcu.apb2,
    );
    
    loop {
        // Receive a byte from the USART and store it in "received"
        let mut received = block!(serial.read()).unwrap();  
        if received >= b'a' && received <= b'z' {
            received -= b'a' - b'A';
        } else if received >= b'A' && received <= b'Z' {
            received += b'a' - b'A';
        } 
        // Write to the USART
        block!(serial.write(received)).ok();
    }
}
