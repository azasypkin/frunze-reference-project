#![feature(used)]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt;
extern crate stm32f0x1;

use cortex_m::asm;

fn main() {
    if let Some(p) = stm32f0x1::Peripherals::take() {
        let rcc = p.RCC;
        let gpio = p.GPIOC;

        /* Enable clock for SYSCFG, else everything will behave funky! */
        rcc.apb2enr.modify(|_, w| w.syscfgen().set_bit());

        /* Enable clock for GPIO Port C */
        rcc.ahbenr.modify(|_, w| w.iopcen().set_bit());

        /* (Re-)configure PC8 as output */
        gpio.moder.modify(|_, w| unsafe { w.moder8().bits(1) });

        loop {
            /* Turn PC8 on a 1000 times in a row */
            for _ in 0..1_000 {
                gpio.bsrr.write(|w| w.bs8().set_bit());
            }
            /* Then turn PC8 off a 1000 times in a row */
            for _ in 0..1_000 {
                gpio.bsrr.write(|w| w.br8().set_bit());
            }
        }
    }
}

// As we are not using interrupts, we just register a dummy catch all handler
#[link_section = ".vector_table.interrupts"]
#[used]
static INTERRUPTS: [extern "C" fn(); 240] = [default_handler; 240];

extern "C" fn default_handler() {
    asm::bkpt();
}
