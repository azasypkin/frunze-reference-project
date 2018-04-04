#![feature(used)]
#![feature(const_fn)]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt;
extern crate cortex_m_semihosting;
#[macro_use(interrupt)]
extern crate stm32f0x1;

use core::cell::RefCell;
use core::fmt::Write;

use cortex_m::interrupt::{self, Mutex};
use cortex_m::asm;
use cortex_m_semihosting::hio;
use stm32f0x1::Interrupt;

static EXTI: Mutex<RefCell<Option<stm32f0x1::EXTI>>> =
    Mutex::new(RefCell::new(None));
static GPIO: Mutex<RefCell<Option<stm32f0x1::GPIOC>>> =
    Mutex::new(RefCell::new(None));

// Read about interrupt setup sequence at:
// http://www.hertaville.com/external-interrupts-on-the-stm32f0.html
fn main() {
    if let (Some(cp), Some(p)) = (
        cortex_m::Peripherals::take(),
        stm32f0x1::Peripherals::take(),
    ) {
        let rcc = p.RCC;
        let gpio = p.GPIOC;

        // Enable EXTI0 interrupt line for PA0.
        let sys_cfg = p.SYSCFG_COMP;
        sys_cfg.syscfg_exticr1.write(|w| unsafe { w.exti0().bits(0) });

        let exti = p.EXTI;
        // Configure PA0 to trigger an interrupt event on the EXTI0 line on a rising edge.
        exti.rtsr.write(|w| w.tr0().set_bit());
        // Unmask the external interrupt line EXTI0 by setting the bit corresponding to the
        // EXTI0 "bit 0" in the EXT_IMR register.
        exti.imr.write(|w| w.mr0().set_bit());

        let mut nvic = cp.NVIC;
        // Set priority for the `EXTI0` line to `1`.
        unsafe { nvic.set_priority(Interrupt::EXTI0_1, 1); }
        // Enable the interrupt in the NVIC.
        nvic.enable(Interrupt::EXTI0_1);

        // Enable clock for SYSCFG, else everything will behave funky.
        rcc.apb2enr.modify(|_, w| w.syscfgen().set_bit());

        // Enable clock for GPIO Port C.
        rcc.ahbenr.modify(|_, w| w.iopcen().set_bit());

        // (Re-)configure PC8 as output.
        gpio.moder.modify(|_, w| unsafe { w.moder8().bits(1) });

        interrupt::free(|cs| {
            *EXTI.borrow(cs).borrow_mut() = Some(exti);
            *GPIO.borrow(cs).borrow_mut() = Some(gpio);
        });

        let mut stdout = hio::hstdout().unwrap();
        writeln!(stdout, "Before sleep").unwrap();

        asm::wfi();

        writeln!(stdout, "After sleep").unwrap();

        loop {
        }
    }
}

interrupt!(EXTI0_1, tick, locals: {
   tick: bool = false;
});

fn tick(locals: &mut EXTI0_1::Locals) {
    locals.tick = !locals.tick;

    let mut stdout = hio::hstdout().unwrap();
    writeln!(stdout, "I am in the interrupt! {}", locals.tick).unwrap();

    interrupt::free(|cs| {
        if let (Some(gpio), Some(exti)) = (
            GPIO.borrow(cs).borrow_mut().as_mut(),
            EXTI.borrow(cs).borrow_mut().as_mut(),
        ) {
            // Toggle PC8.
            gpio.bsrr.write(|w| {
                if locals.tick {
                    w.bs8().set_bit()
                } else {
                    w.br8().set_bit()
                }
            });

            // Set pending register to mark interrupt as handled.
            exti.pr.modify(|_, w| w.pr0().set_bit());
        }
    });
    //asm::bkpt();
}
