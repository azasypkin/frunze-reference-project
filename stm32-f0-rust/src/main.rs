#![no_main]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt;
extern crate cortex_m_semihosting;
extern crate panic_semihosting;

#[cfg(feature = "stm32f051")]
#[macro_use(interrupt)]
extern crate stm32f0x1 as stm32f0x;

#[cfg(feature = "stm32f042")]
#[macro_use(interrupt)]
extern crate stm32f0x2 as stm32f0x;

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

use cortex_m_rt::{entry, exception, ExceptionFrame};

use beeper::Beeper;
use button::{Button, PressType};
use rtc::{Time, RTC};

#[derive(Debug)]
enum Mode {
    Alarm,
    Sleep,
    Setup(u8),
}

static CORE_PERIPHERALS: Mutex<RefCell<Option<CorePeripherals>>> = Mutex::new(RefCell::new(None));
static PERIPHERALS: Mutex<RefCell<Option<Peripherals>>> = Mutex::new(RefCell::new(None));
static MODE: Mutex<RefCell<Mode>> = Mutex::new(RefCell::new(Mode::Sleep));

const PRESET_COUNT: u8 = 3;

// Read about interrupt setup sequence at:
// http://www.hertaville.com/external-interrupts-on-the-stm32f0.html
#[entry]
fn main() -> ! {
    //let mut stdout = hio::hstdout().unwrap();
    //writeln!(stdout, "Starting...").unwrap();

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

        configure_standby_mode(&mut cp, p);

        Beeper::acquire(&mut cp, p, |mut beeper| {
            beeper.play_melody();
        });
    });

    loop {
        asm::wfi();
    }
}

fn configure_standby_mode(core_peripherals: &mut CorePeripherals, peripherals: &Peripherals) {
    // Select STANDBY mode.
    peripherals.PWR.cr.modify(|_, w| w.pdds().set_bit());

    // Clear Wakeup flag.
    peripherals.PWR.cr.modify(|_, w| w.cwuf().set_bit());

    // Set SLEEPDEEP bit of Cortex-M0 System Control Register.
    core_peripherals.SCB.set_sleepdeep();
}

interrupt!(EXTI0_1, button_handler);

fn button_handler() {
    interrupt_free(|mut cp, p, mode| {
        let press_type = Button::acquire(&mut cp, p, |mut button| {
            button.get_press_type(PressType::Long)
        });

        match press_type {
            PressType::Short => {
                let mut num_sec = 0;
                if let Mode::Setup(ref s) = mode {
                    num_sec = if *s == PRESET_COUNT { 1 } else { s + 1 };

                    Beeper::acquire(&mut cp, p, |mut beeper| {
                        beeper.beep_n(num_sec);
                    });
                }

                if num_sec != 0 {
                    *mode = Mode::Setup(num_sec);
                }
            }

            PressType::Long => {
                RTC::acquire(&mut cp, p, |mut rtc| {
                    reset_alarm(&mut rtc);
                });

                Beeper::acquire(&mut cp, p, |mut beeper| {
                    beeper.play_setup();
                });

                let press_type = Button::acquire(&mut cp, p, |mut button| {
                    button.get_press_type(PressType::Long)
                });

                match press_type {
                    PressType::Long => {
                        *mode = Mode::Sleep;

                        Beeper::acquire(&mut cp, p, |mut beeper| {
                            beeper.play_reset();
                        });
                    }
                    _ => {
                        *mode = if let Mode::Setup(ref s) = mode {
                            RTC::acquire(&mut cp, p, |mut rtc| {
                                set_alarm(&mut rtc, &(s * 10));
                            });
                            Mode::Alarm
                        } else {
                            Mode::Setup(0)
                        }
                    }
                }
            }

            _ => {}
        }

        Button::acquire(&mut cp, p, |button| button.clear_pending_interrupt());

        // Clear Wakeup flag.
        p.PWR.cr.modify(|_, w| w.cwuf().set_bit());
    });
}

interrupt!(RTC, on_alarm);

fn on_alarm() {
    interrupt_free(|mut cp, p, _| {
        Beeper::acquire(&mut cp, p, |mut beeper| {
            beeper.play_melody();
        });

        // Repeat alarm in 10 seconds until it's reset.
        RTC::acquire(&mut cp, p, |mut rtc| {
            reset_alarm(&mut rtc);
            set_alarm(&mut rtc, &10);
        });

        // Clear Wakeup flag.
        p.PWR.cr.modify(|_, w| w.cwuf().set_bit());
    });
}

fn reset_alarm(rtc: &mut RTC) {
    let reset_time = Time {
        hours: 0,
        minutes: 0,
        seconds: 0,
    };

    rtc.configure_time(&reset_time);
    rtc.configure_alarm(&reset_time);
    rtc.toggle_alarm(false);
    rtc.clear_pending_interrupt();
}

fn set_alarm(rtc: &mut RTC, num_secs: &u8) {
    rtc.configure_time(&Time {
        hours: 1,
        minutes: 0,
        seconds: 0,
    });

    rtc.configure_alarm(&Time {
        hours: 1,
        minutes: 0,
        seconds: *num_secs,
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

#[exception]
fn DefaultHandler(irqn: i16) {
    panic!("unhandled exception (IRQn={})", irqn);
}

#[exception]
fn HardFault(_ef: &ExceptionFrame) -> ! {
    loop {}
}
