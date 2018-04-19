#![feature(used)]
#![feature(const_fn)]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt;
extern crate cortex_m_semihosting;
extern crate panic_semihosting;

#[cfg(feature="stm32f051")]
#[macro_use(interrupt)]
extern crate stm32f0x1 as stm32f0x;

#[cfg(feature="stm32f042")]
#[macro_use(interrupt)]
extern crate stm32f0x2 as stm32f0x;

mod beeper;
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
use rtc::RTC;

static CORE_PERIPHERALS: Mutex<RefCell<Option<CorePeripherals>>> =
    Mutex::new(RefCell::new(None));
static PERIPHERALS: Mutex<RefCell<Option<Peripherals>>> = Mutex::new(RefCell::new(None));

// Read about interrupt setup sequence at:
// http://www.hertaville.com/external-interrupts-on-the-stm32f0.html
fn main() {
    let mut stdout = hio::hstdout().unwrap();
    writeln!(stdout, "Starting...").unwrap();

    interrupt::free(|cs| {
        *PERIPHERALS.borrow(cs).borrow_mut() = Some(Peripherals::take().unwrap());
        *CORE_PERIPHERALS.borrow(cs).borrow_mut() = Some(cortex_m::Peripherals::take().unwrap());
    });

    interrupt::free(|cs| {
        if let (Some(mut cp), Some(p)) = (
            CORE_PERIPHERALS.borrow(cs).borrow_mut().as_mut(),
            PERIPHERALS.borrow(cs).borrow_mut().as_mut(),
        ) {

            Beeper::configure(&p);
            play_melody(Beeper::new(&mut cp, p));

            writeln!(stdout, "Before RTC").unwrap();
            let current_time;
            {
                let mut rtc = RTC::new(&mut cp, &p);
                rtc.configure();
                writeln!(stdout, "RTC configured").unwrap();
                current_time = rtc.get_time();
            }

            writeln!(stdout, "After RTC: {:?}", current_time).unwrap();

            enter_standby_mode(&cp, p);

            writeln!(stdout, "After StandBy").unwrap();
        }
    });

    loop {}
}

fn enter_standby_mode(
    core_peripherals: &CorePeripherals,
    peripherals: &Peripherals,
) {
    // Select STANDBY mode.
    peripherals.PWR.cr.modify(|_, w| w.pdds().set_bit());

    // Clear Wakeup flag.
    peripherals.PWR.cr.modify(|_, w| w.cwuf().set_bit());

    // Set SLEEPDEEP bit of Cortex-M0 System Control Register.
    unsafe { core_peripherals.SCB.scr.modify(|w| w | w | 0b100) }

    asm::wfi();
}

interrupt!(RTC, on_alarm);

fn on_alarm() {
    let mut stdout = hio::hstdout().unwrap();
    writeln!(stdout, "Alarm interrupt!").unwrap();

    interrupt::free(|cs| {
        if let (Some(mut cp), Some(p)) = (
            CORE_PERIPHERALS.borrow(cs).borrow_mut().as_mut(),
            PERIPHERALS.borrow(cs).borrow_mut().as_mut(),
        ) {
            play_melody(Beeper::new(&mut cp, p));

           let mut rtc = RTC::new(&mut cp, &p);

            // Check alarm A flag.
            if rtc.is_alarm_interrupt() {
                let mut current_time = rtc.get_time();

                writeln!(stdout, "Clear pending... {:?}", current_time).unwrap();

                current_time.add_seconds(10);

                rtc.configure_alarm(&current_time);

                rtc.clear_pending_interrupt();
            } else {
                writeln!(stdout, "Disabling...").unwrap();
                rtc.disable_interrupt();
            }
        }
    });
}

fn play_melody(mut beeper: Beeper) {
    beeper.play_melody();
}
