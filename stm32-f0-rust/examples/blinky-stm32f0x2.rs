#![feature(used)]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt;
extern crate stm32f0x2;

use cortex_m::asm;

fn main() {
    if let Some(p) = stm32f0x2::Peripherals::take() {
        let rcc = p.RCC;
        let gpio = p.GPIOA;

        /* Enable clock for SYSCFG, else everything will behave funky! */
        rcc.apb2enr.modify(|_, w| w.syscfgen().set_bit());

        /* Enable clock for GPIO Port A */
        rcc.ahbenr.modify(|_, w| w.iopaen().set_bit());

        /* (Re-)configure PA4 as output */
        gpio.moder.modify(|_, w| unsafe { w.moder4().bits(1) });

        loop {
            /* Turn PA4 on a 1000 times in a row */
            for _ in 0..300 {
                gpio.bsrr.write(|w| w.bs4().set_bit());
            }
            /* Then turn PA4 off a 1000 times in a row */
            for _ in 0..300 {
                gpio.bsrr.write(|w| w.br4().set_bit());
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
