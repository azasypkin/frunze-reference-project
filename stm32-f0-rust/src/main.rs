#![feature(used)]
#![feature(const_fn)]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt;
extern crate cortex_m_semihosting;
extern crate panic_semihosting;

#[macro_use(interrupt)]
extern crate stm32f0x1;

mod rtc;

use core::cell::RefCell;
use core::fmt::Write;

use cortex_m::interrupt::{self, Mutex};
use cortex_m_semihosting::hio;

use rtc::rtc::RTC;

static CORE_PERIPHERALS: Mutex<RefCell<Option<cortex_m::Peripherals>>> =
    Mutex::new(RefCell::new(None));
static PERIPHERALS: Mutex<RefCell<Option<stm32f0x1::Peripherals>>> = Mutex::new(RefCell::new(None));

// Read about interrupt setup sequence at:
// http://www.hertaville.com/external-interrupts-on-the-stm32f0.html
fn main() {
    interrupt::free(|cs| {
        *PERIPHERALS.borrow(cs).borrow_mut() = Some(stm32f0x1::Peripherals::take().unwrap());
        *CORE_PERIPHERALS.borrow(cs).borrow_mut() = Some(cortex_m::Peripherals::take().unwrap());
    });

    interrupt::free(|cs| {
        if let (Some(mut cp), Some(p)) = (
            CORE_PERIPHERALS.borrow(cs).borrow_mut().as_mut(),
            PERIPHERALS.borrow(cs).borrow_mut().as_mut(),
        ) {
            let mut stdout = hio::hstdout().unwrap();
            writeln!(stdout, "Before RTC").unwrap();

            let mut rtc = RTC::new(&mut cp, &p);
            rtc.configure();

            let mut stdout = hio::hstdout().unwrap();
            writeln!(stdout, "After RTC").unwrap();
        }
    });

    loop {}
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

            let mut rtc = RTC::new(&mut cp, &p);

            // Check alarm A flag.
            if rtc.is_alarm_interrupt() {
                let mut current_time = rtc.get_time();
                writeln!(stdout, "Clear pending... {:?}", current_time).unwrap();

                current_time.add_seconds(5);

                rtc.configure_alarm(&current_time);

                rtc.clear_pending_interrupt();
            } else {
                writeln!(stdout, "Disabling...").unwrap();
                rtc.disable_interrupt();
            }
        }
    });
}
