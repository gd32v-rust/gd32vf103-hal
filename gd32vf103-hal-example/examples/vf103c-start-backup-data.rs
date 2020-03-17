// vf103c-start-backup-data.rs

#![no_std]
#![no_main]

use gd32vf103_hal as hal;
use hal::{pac, prelude::*, backup::*, rcu, serial::{Serial, Config}};
use panic_halt as _;
use core::fmt::Write;
use nb::block;

#[riscv_rt::entry]
fn main() -> ! {
    let mut dp = pac::Peripherals::take().unwrap();
    let mut rcu = dp.RCU.constrain();
    let mut afio = dp.AFIO.split(&mut rcu.apb2);
    let mut bkp = dp.BKP.split(&mut rcu.apb1, &mut dp.PMU);

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

    bkp.tamper.set_pin_active_high();
    bkp.tamper.clear_event_bit();
    bkp.tamper.enable();

    let mut idx: usize = 0;
    let mut value: u32 = 0;
    let mut state = 0;
    let mut err = 0;
    
    write_help(&mut serial);
    
    loop {
        if state == 0 {
            write!(serial, "> ").ok();
        }
        let ch = block!(serial.read()).unwrap();
        block!(serial.write(ch)).ok();
        match (state, ch) {
            (0, b'x') => state = 10,
            (10, b'\r') | (10, b'\n') => {
                execute_query_all(&mut serial, &mut bkp);
                state = 0;
            },
            (10, _) => {},
            (0, b's') => {
                state = 30;
            },
            (0, b'g') => {
                state = 40;
            },
            (0, b'c') => {
                state = 50;
            },
            (0, _) => state = 20,
            (20, b'\r') | (20, b'\n') => {
                write!(serial, "Invalid input!\r\n").ok();
                state = 0
            },
            (30, b' ') => {
                state = 1;
                idx = 0;
                value = 0;
                err = 0;
            },
            (30, _) => {
                state = 0;
            }
            (40, b'\r') | (40, b'\n') => {
                for i in 0..42 { 
                    bkp.data.write(i, 0x2333 + i as u16);
                }
                write!(serial, "All register values set!\r\n").ok();
                state = 0;
            },
            (40, _) => {
                state = 0;
            }
            (50, b'\r') | (50, b'\n') => {
                bkp.tamper.clear_event_bit();
                write!(serial, "Tamper event flag cleared!\r\n").ok();
                state = 0;
            },
            (50, _) => {
                state = 0;
            }
            (1, ch @ b'0'..=b'9') => {
                idx *= 10;
                idx += (ch - b'0') as usize;
                if idx >= 42 {
                    err = 1;
                    idx = 42;
                }
            },
            (1, b' ') => state = 2,
            (1, _) => {
                err = 2;
            },
            (2, ch @ b'0'..=b'9') => {
                value *= 10;
                value += (ch - b'0') as u32;
                if value >= 0x10000 {
                    err = 3;
                    value = 0x10000;
                }
            },
            (2, b'\r') | (2, b'\n') => {
                match err {
                    0 => {
                        bkp.data.write(idx, value as u16);
                        write!(serial, "Value of register #{} is set to {}!\r\n", idx, value).unwrap();
                    },
                    1 => write!(serial, "Error: Invalid index; must within 0..42\r\n").unwrap(),
                    2 => write!(serial, "Error: Invalid input; only digits are allowed\r\n").unwrap(),
                    3 => write!(serial, "Error: Invalid value; only u16 values are valid\r\n").unwrap(),
                    _ => unreachable!()
                }
                state = 0;
            },
            (2, _) => {
                err = 2;
            },
            _ => unreachable!()
        }
    }
}

fn write_help<T: Write>(out: &mut T) {
    write!(out, "\r
Welcome to GD32VF103C-START BKP Data example!\r
Type 'x' to list all BKP data register values;\r
Type 's <id> <u16 value>' to set register value;\r
Type 'g' to set all BKP data register to 0x2333 + id;\r
Type 'c' to clear tamper event flag.\r
\r
Note: Tamper pin would come into effect. If you connect\r
Pin 2 (PC13) to Pin 48, all BKP data would be cleared to 0.\r
\r\n"
    ).ok();
}

fn execute_query_all<T: Write>(out: &mut T, bkp: &mut Parts) {
    write!(out, "All BKP data registers: \r\n").ok();
    for i in 0..42 { 
        if i % 8 == 0 {
            write!(out, "#{}-{}:\t", i, i + 7).ok();
        }
        write!(out, "{:04X} ", bkp.data.read(i)).ok();
        if i % 8 == 7 {
            write!(out, "\r\n").ok();
        }
    }
    write!(out, "\r\n").ok();
}
