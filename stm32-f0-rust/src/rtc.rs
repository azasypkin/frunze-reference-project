pub mod rtc {
    use cortex_m::Peripherals as CorePeripherals;
    use stm32f0x1::Interrupt;
    use stm32f0x1::Peripherals;

    #[derive(Debug)]
    pub struct Time {
        hours: u8,
        minutes: u8,
        seconds: u8
    }


    pub struct RTC<'a> {
        core_peripherals: &'a mut CorePeripherals,
        peripherals: &'a Peripherals,
    }

    impl<'a> RTC<'a> {
        pub fn new(
            core_peripherals: &'a mut CorePeripherals,
            peripherals: &'a Peripherals,
        ) -> RTC<'a> {
            RTC {
                core_peripherals,
                peripherals,
            }
        }

        pub fn configure(&mut self) {
            // Enable the peripheral clock RTC.
            self.configure_clock();

            // Configure alarm.
            self.configure_alarm();

            // Configure EXTI and NVIC for RTC IT.
            self.configure_interrupts();

            // Set time.
            self.configure_time();
        }

        pub fn is_alarm_interrupt(&self) -> bool {
            self.peripherals.RTC.isr.read().alraf().bit_is_set()
        }

        pub fn disable_interrupt(&mut self) {
            // Disable RTC_IRQn in the NVIC.
            self.core_peripherals.NVIC.disable(Interrupt::RTC);
        }

        pub fn clear_pending_interrupt(&self) {
            // Clear Alarm A flag.
            self.peripherals.RTC.isr.modify(|_, w| w.alraf().clear_bit());
            // Clear exti line 17 flag.
            self.peripherals.EXTI.pr.write(|w| w.pr17().set_bit());
        }

        fn configure_clock(&self) {
            // Enable the LSI.
            self.peripherals.RCC.csr.modify(|_, w| w.lsion().set_bit());

            // Wait while it is not ready.
            while self.peripherals.RCC.csr.read().lsirdy().bit_is_clear() {}

            // Enable PWR clock.
            self.peripherals
                .RCC
                .apb1enr
                .modify(|_, w| w.pwren().set_bit());

            // Enable write in RTC domain control register.
            self.peripherals.PWR.cr.modify(|_, w| w.dbp().set_bit());

            // LSI for RTC clock.
            self.peripherals.RCC.bdcr.modify(|_, w| {
                w.rtcen().set_bit();
                unsafe { w.rtcsel().bits(0b10) }
            });

            // Disable PWR clock.
            self.peripherals
                .RCC
                .apb1enr
                .modify(|_, w| w.pwren().clear_bit());
        }

        fn configure_alarm(&self) {
            // Disable the write protection for RTC registers.
            self.peripherals.RTC.wpr.write(|w| unsafe { w.bits(0xCA) });
            self.peripherals.RTC.wpr.write(|w| unsafe { w.bits(0x53) });

            // Disable alarm A to modify it.
            self.peripherals.RTC.cr.modify(|_, w| w.alrae().clear_bit());

            // Wait until it is allowed to modify alarm A value.
            while self.peripherals.RTC.isr.read().alrawf().bit_is_clear() {}

            // Modify alarm A mask to have an interrupt each minute.
            self.peripherals.RTC.alrmar.modify(|_, w| {
                unsafe {
                    w.bits(0);
                    w.st().bits(0x3);
                    w.su().bits(0x2);
                }

                w.msk2().set_bit();
                w.msk3().set_bit();
                w.msk4().set_bit()
            });

            // Enable alarm A and alarm A interrupt.
            self.peripherals.RTC.cr.modify(|_, w| {
                w.alraie().set_bit();
                w.alrae().set_bit()
            });

            // Enable the write protection for RTC registers.
            self.peripherals.RTC.wpr.write(|w| unsafe { w.bits(0xFE) });
            self.peripherals.RTC.wpr.write(|w| unsafe { w.bits(0x64) });
        }

        fn configure_time(&self) {
            // Disable the write protection for RTC registers.
            self.peripherals.RTC.wpr.write(|w| unsafe { w.bits(0xCA) });
            self.peripherals.RTC.wpr.write(|w| unsafe { w.bits(0x53) });

            // Enable init phase.
            self.peripherals.RTC.isr.modify(|_, w| w.init().set_bit());

            // Wait until it is allowed to modify RTC register values.
            while self.peripherals.RTC.isr.read().initf().bit_is_clear() {}

            // Set prescaler, 40kHz/128 (0x7F + 1) => 312 Hz, 312Hz/312 (0x137 + 1) => 1Hz.
            self.peripherals
                .RTC
                .prer
                .modify(|_, w| unsafe { w.prediv_s().bits(0x137) });

            self.peripherals
                .RTC
                .prer
                .modify(|_, w| unsafe { w.prediv_a().bits(0x7F) });

            // Configure Time register.
            self.peripherals.RTC.tr.modify(|_, w| {
                unsafe {
                    w.hu().bits(0x01);
                    w.mnt().bits(0);
                    w.mnu().bits(0);
                    w.su().bits(0);
                }

                w
            });

            // Disable init phase.
            self.peripherals.RTC.isr.modify(|_, w| w.init().clear_bit());

            // Enable the write protection for RTC registers.
            self.peripherals.RTC.wpr.write(|w| unsafe { w.bits(0xFE) });
            self.peripherals.RTC.wpr.write(|w| unsafe { w.bits(0x64) });
        }

        pub fn get_time(&self) -> Time {
            let tr = self.peripherals.RTC.tr.read();

            Time {
                hours: tr.ht().bits() * 10 + tr.hu().bits(),
                minutes: tr.mnt().bits() * 10 + tr.mnu().bits(),
                seconds: tr.st().bits() * 10 + tr.su().bits()
            }
        }

        fn configure_interrupts(&mut self) {
            // Unmask line 17, EXTI line 17 is connected to the RTC Alarm event.
            self.peripherals.EXTI.imr.write(|w| w.mr17().set_bit());
            // Rising edge for line 17.
            self.peripherals.EXTI.rtsr.write(|w| w.tr17().set_bit());
            // Set priority.
            unsafe {
                self.core_peripherals.NVIC.set_priority(Interrupt::RTC, 0);
            }
            // Enable RTC_IRQn in the NVIC.
            self.core_peripherals.NVIC.enable(Interrupt::RTC);
        }
    }
}
