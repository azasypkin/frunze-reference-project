#![feature(used)]
#![feature(const_fn)]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt;
extern crate cortex_m_semihosting;
extern crate panic_semihosting;

/*#[cfg(feature = "stm32f051")]
#[macro_use(interrupt)]
extern crate stm32f0x1 as stm32f0x;

#[cfg(feature = "stm32f042")]
#[macro_use(interrupt)]
extern crate stm32f0x2 as stm32f0x;*/

#[macro_use(interrupt)]
extern crate stm32f0x1 as stm32f0x;

mod beeper;
mod button;
mod config;
mod rtc;
mod systick;

use core::cell::RefCell;
use core::fmt::Write;

use cortex_m::asm;
use cortex_m::interrupt::{self, Mutex};
use cortex_m_semihosting::hio;

use cortex_m::Peripherals as CorePeripherals;
use stm32f0x::Peripherals;

use beeper::Beeper;
use button::{Button, PressType};
use rtc::{Time, RTC};

#[derive(Debug)]
enum Mode {
    Sleep,
    Setup,
}

static CORE_PERIPHERALS: Mutex<RefCell<Option<CorePeripherals>>> = Mutex::new(RefCell::new(None));
static PERIPHERALS: Mutex<RefCell<Option<Peripherals>>> = Mutex::new(RefCell::new(None));
static MODE: Mutex<RefCell<Mode>> = Mutex::new(RefCell::new(Mode::Sleep));

// Read about interrupt setup sequence at:
// http://www.hertaville.com/external-interrupts-on-the-stm32f0.html
fn main() {
    let mut stdout = hio::hstdout().unwrap();
    writeln!(stdout, "Starting...").unwrap();

    interrupt::free(|cs| {
        *PERIPHERALS.borrow(cs).borrow_mut() = Some(Peripherals::take().unwrap());
        *CORE_PERIPHERALS.borrow(cs).borrow_mut() = Some(cortex_m::Peripherals::take().unwrap());
    });

    interrupt_free(|mut cp, p, _| {
        Beeper::configure(&p);
        Button::configure(&p, &mut cp);
        RTC::configure(cp, p);

        RTC::acquire(cp, p, |mut rtc| {
            rtc.toggle_alarm(false);
        });

        configure_standby_mode(&cp, p);
    });

    loop {
        writeln!(stdout, "Sleep").unwrap();
        asm::wfi();
        writeln!(stdout, "Wake").unwrap();
    }
}

fn configure_standby_mode(core_peripherals: &CorePeripherals, peripherals: &Peripherals) {
    // Select STANDBY mode.
    peripherals.PWR.cr.modify(|_, w| w.pdds().set_bit());

    // Clear Wakeup flag.
    peripherals.PWR.cr.modify(|_, w| w.cwuf().set_bit());

    // Set SLEEPDEEP bit of Cortex-M0 System Control Register.
    unsafe { core_peripherals.SCB.scr.modify(|w| w | 0b100) }
}

interrupt!(EXTI0_1, button_handler);

fn button_handler() {
    interrupt_free(|mut cp, p, mode| {
        let press_type = Button::acquire(&mut cp, p, |mut button| {
            button.get_press_type(PressType::Long)
        });

        if let PressType::Long = press_type {
            RTC::acquire(&mut cp, p, reset_alarm);

            Beeper::acquire(&mut cp, p, |mut beeper| {
                beeper.beep_n(2);
            });

            let press_type = Button::acquire(&mut cp, p, |mut button| {
                button.get_press_type(PressType::Long)
            });

            match press_type {
                PressType::Long => {
                    *mode = Mode::Sleep;

                    Beeper::acquire(&mut cp, p, |mut beeper| {
                        beeper.beep_n(3);
                    });
                }
                _ => {
                    *mode = Mode::Setup;

                    RTC::acquire(&mut cp, p, |rtc| {
                        set_alarm(rtc);
                    });
                }
            }
        }

        Button::acquire(&mut cp, p, |button| button.clear_pending_interrupt());
    });
}

interrupt!(RTC, on_alarm);

fn on_alarm() {
    interrupt_free(|mut cp, p, _| {
        Beeper::acquire(&mut cp, p, |mut beeper| {
            beeper.beep();
        });

        RTC::acquire(&mut cp, p, |mut rtc| {
            let mut current_time = rtc.get_time();
            current_time.add_seconds(15);

            rtc.configure_alarm(&current_time);

            rtc.clear_pending_interrupt();
        });
    });
}

fn reset_alarm(mut rtc: RTC) {
    let reset_time = Time {
        hours: 0,
        minutes: 0,
        seconds: 0,
    };

    rtc.configure_time(&reset_time);
    rtc.configure_alarm(&reset_time);
    rtc.toggle_alarm(false);
}

fn set_alarm(mut rtc: RTC) {
    rtc.configure_time(&Time {
        hours: 1,
        minutes: 1,
        seconds: 1,
    });

    rtc.configure_alarm(&Time {
        hours: 1,
        minutes: 1,
        seconds: 15,
    });
}

fn interrupt_free<F>(f: F) -> ()
where
    F: FnOnce(&mut CorePeripherals, &Peripherals, &mut Mode),
{
    interrupt::free(|cs| {
        if let (Some(cp), Some(p), mut m) = (
            CORE_PERIPHERALS.borrow(cs).borrow_mut().as_mut(),
            PERIPHERALS.borrow(cs).borrow_mut().as_mut(),
            MODE.borrow(cs).borrow_mut(),
        ) {
            f(cp, p, &mut m);
        } else {
            panic!("Can not borrow peripherals!");
        }
    });
}
