/*
#![feature(used)]
#![feature(const_fn)]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt;
extern crate cortex_m_semihosting;
extern crate panic_semihosting;

#[macro_use(interrupt)]
extern crate stm32f0x1;

use core::cell::RefCell;
use core::fmt::Write;

use cortex_m::asm;
use cortex_m::interrupt::{self, Mutex};
use cortex_m_semihosting::hio;
use stm32f0x1::Interrupt;

static CORE_PERIPHERALS: Mutex<RefCell<Option<cortex_m::Peripherals>>> =
    Mutex::new(RefCell::new(None));
static PERIPHERALS: Mutex<RefCell<Option<stm32f0x1::Peripherals>>> = Mutex::new(RefCell::new(None));


// Read about interrupt setup sequence at:
// http://www.hertaville.com/external-interrupts-on-the-stm32f0.html
fn main() {
    interrupt::free(|cs| {
        let p = stm32f0x1::Peripherals::take().unwrap();

        // Enable EXTI0 interrupt line for PA0.
        p.SYSCFG_COMP
            .syscfg_exticr1
            .write(|w| unsafe { w.exti0().bits(0) });

        // Configure PA0 to trigger an interrupt event on the EXTI0 line on a rising edge.
        p.EXTI.rtsr.write(|w| w.tr0().set_bit());
        // Unmask the external interrupt line EXTI0 by setting the bit corresponding to the
        // EXTI0 "bit 0" in the EXT_IMR register.
        p.EXTI.imr.write(|w| w.mr0().set_bit());

        // Enable clock for SYSCFG, else everything will behave funky.
        p.RCC.apb2enr.modify(|_, w| w.syscfgen().set_bit());

        // Enable clock for GPIO Port C.
        p.RCC.ahbenr.modify(|_, w| w.iopcen().set_bit());

        // (Re-)configure PC8 as output.
        p.GPIOC.moder.modify(|_, w| unsafe { w.moder8().bits(1) });

        let mut cp = cortex_m::Peripherals::take().unwrap();
        // Set priority for the `EXTI0` line to `1`.
        unsafe {
            cp.NVIC.set_priority(Interrupt::EXTI0_1, 1);
        }
        // Enable the interrupt in the NVIC.
        cp.NVIC.enable(Interrupt::EXTI0_1);

        {
            let _rtc = RTC::new(&cp, &p);
            _rtc.configure();
        }

        *PERIPHERALS.borrow(cs).borrow_mut() = Some(p);
        *CORE_PERIPHERALS.borrow(cs).borrow_mut() = Some(cp);
    });

    //configure_rtc();

    let mut stdout = hio::hstdout().unwrap();
    writeln!(stdout, "Before sleep").unwrap();

    toggle(true);

    *//*let mut counter = 0;
    while counter < 100000 {
        counter = counter + 1;
    }
    toggle(false);
    enter_standby_mode();

    // configure_pwm();
    // play_melody();*//*
    asm::wfi();

    loop {}
}

interrupt!(EXTI0_1, tick, locals: {
   tick: bool = false;
});

fn toggle(on: bool) {
    interrupt::free(|cs| {
        if let Some(p) = PERIPHERALS.borrow(cs).borrow_mut().as_mut() {
            // Toggle PC8.
            p.GPIOC.bsrr.write(|w| {
                if on {
                    w.bs8().set_bit()
                } else {
                    w.br8().set_bit()
                }
            });
        }
    });
}

fn tick(locals: &mut EXTI0_1::Locals) {
    locals.tick = !locals.tick;

    let mut stdout = hio::hstdout().unwrap();
    writeln!(stdout, "I am in the interrupt! {}", locals.tick).unwrap();

    interrupt::free(|cs| {
        if let Some(p) = PERIPHERALS.borrow(cs).borrow_mut().as_mut() {
            // Toggle PC8.
            p.GPIOC.bsrr.write(|w| {
                if locals.tick {
                    w.bs8().set_bit()
                } else {
                    w.br8().set_bit()
                }
            });

            // Set pending register to mark interrupt as handled.
            p.EXTI.pr.modify(|_, w| w.pr0().set_bit());
        }
    });
    //asm::bkpt();
}
*/
