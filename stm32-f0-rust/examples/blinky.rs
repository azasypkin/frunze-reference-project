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
//use cortex_m::asm;
use cortex_m_semihosting::hio;
use stm32f0x1::Interrupt;

static EXTI: Mutex<RefCell<Option<stm32f0x1::EXTI>>> =
    Mutex::new(RefCell::new(None));
static GPIO: Mutex<RefCell<Option<stm32f0x1::GPIOC>>> =
    Mutex::new(RefCell::new(None));

fn main() {
    if let (Some(cp), Some(p)) = (
        cortex_m::Peripherals::take(),
        stm32f0x1::Peripherals::take(),
    ) {
        let mut nvic = cp.NVIC;
        let rcc = p.RCC;
        let gpio = p.GPIOC;

        let sys_cfg = p.SYSCFG_COMP;
        let exti = p.EXTI;

        sys_cfg.syscfg_exticr1.write(|w| unsafe { w.exti0().bits(0) });
        exti.rtsr.write(|w| w.tr0().set_bit());
        exti.imr.write(|w| w.mr0().set_bit());

        unsafe { nvic.set_priority(Interrupt::EXTI0_1, 1); }
        nvic.enable(Interrupt::EXTI0_1);

        /* Enable clock for SYSCFG, else everything will behave funky! */
        rcc.apb2enr.modify(|_, w| w.syscfgen().set_bit());

        /* Enable clock for GPIO Port C */
        rcc.ahbenr.modify(|_, w| w.iopcen().set_bit());

        /* (Re-)configure PC8 as output */
        gpio.moder.modify(|_, w| unsafe { w.moder8().bits(1) });

        interrupt::free(|cs| {
            *EXTI.borrow(cs).borrow_mut() = Some(exti);
            *GPIO.borrow(cs).borrow_mut() = Some(gpio);
        });

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
        if let Some(gpio) = GPIO.borrow(cs).borrow_mut().as_mut() {
            if locals.tick {
                // Turn PC8 on
                gpio.bsrr.write(|w| w.bs8().set_bit());
            } else {
                // Turn PC8 off
                gpio.bsrr.write(|w| w.br8().set_bit());
            }
        }

        if let Some(exti) = EXTI.borrow(cs).borrow_mut().as_mut() {
            writeln!(stdout, "Clearing....").unwrap();
            exti.pr.modify(|_, w| w.pr0().set_bit());
        }
    });
    //asm::bkpt();
}
