use config;
use cortex_m::Peripherals as CorePeripherals;
use stm32f0x::Interrupt;
use stm32f0x::Peripherals;
use systick::SysTick;

pub struct Button<'a> {
    core_peripherals: &'a mut CorePeripherals,
    peripherals: &'a Peripherals,
}

impl<'a> Button<'a> {
    pub fn new(
        core_peripherals: &'a mut CorePeripherals,
        peripherals: &'a Peripherals,
    ) -> Button<'a> {
        Button {
            core_peripherals,
            peripherals,
        }
    }

    pub fn configure(peripherals: &Peripherals, core_peripherals: &mut CorePeripherals) {
        // Enable EXTI0 interrupt line for PA0.
        peripherals
            .SYSCFG_COMP
            .syscfg_exticr1
            .write(|w| unsafe { w.exti0().bits(0) });

        // Configure PA0 to trigger an interrupt event on the EXTI0 line on a rising edge.
        peripherals.EXTI.rtsr.modify(|_, w| w.tr0().set_bit());

        // Unmask the external interrupt line EXTI0 by setting the bit corresponding to the
        // EXTI0 "bit 0" in the EXT_IMR register.
        peripherals.EXTI.imr.modify(|_, w| w.mr0().set_bit());

        // Enable clock for GPIO Port A.
        peripherals.RCC.ahbenr.modify(|_, w| w.iopaen().set_bit());

        // Switch PA0 to alternate function mode.
        peripherals
            .GPIOA
            .moder
            .modify(|_, w| unsafe { w.moder0().bits(0b10) });

        // Set alternative function #2.
        peripherals
            .GPIOA
            .afrl
            .modify(|_, w| unsafe { w.afrl0().bits(0b0010) });

        // Set priority for the `EXTI0` line to `1`.
        unsafe {
            core_peripherals.NVIC.set_priority(Interrupt::EXTI0_1, 1);
        }
        // Enable the interrupt in the NVIC.
        core_peripherals.NVIC.enable(Interrupt::EXTI0_1);

        // Enable waker
        peripherals.PWR.csr.modify(|_, w| w.ewup1().set_bit());
    }
}
