use cortex_m::Peripherals as CorePeripherals;
use stm32f0x::Interrupt;
use stm32f0x::Peripherals;

#[derive(Debug)]
pub struct Time {
    pub hours: u8,
    pub minutes: u8,
    pub seconds: u8,
}

impl Time {
    pub fn add_seconds(&mut self, seconds: u8) {
        if self.seconds + seconds < 60 {
            self.seconds = self.seconds + seconds;
        } else {
            self.seconds = self.seconds + seconds - 60;
            self.add_minutes(1);
        }
    }

    pub fn add_minutes(&mut self, minutes: u8) {
        if self.minutes + minutes < 60 {
            self.minutes = self.minutes + minutes;
        } else {
            self.minutes = self.minutes + minutes - 60;
            self.add_hours(1);
        }
    }

    pub fn add_hours(&mut self, hours: u8) {
        if self.hours + hours < 24 {
            self.hours = self.hours + hours;
        } else {
            self.hours = self.hours + hours - 24;
        }
    }
}

pub struct RTC<'a> {
    core_peripherals: &'a mut CorePeripherals,
    peripherals: &'a Peripherals,
}

impl<'a> RTC<'a> {
    pub fn new(core_peripherals: &'a mut CorePeripherals, peripherals: &'a Peripherals) -> RTC<'a> {
        RTC {
            core_peripherals,
            peripherals,
        }
    }

    pub fn configure(&mut self) {
        // Enable the peripheral clock RTC.
        self.configure_clock();

        // Configure EXTI and NVIC for RTC IT.
        self.configure_interrupts();
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
        self.peripherals
            .RTC
            .isr
            .modify(|_, w| w.alraf().clear_bit());

        // Clear exti line 17 flag.
        self.peripherals.EXTI.pr.modify(|_, w| {
            #[cfg(feature = "stm32f051")]
            return w.pr17().set_bit();

            #[cfg(feature = "stm32f042")]
            return w.pif17().set_bit();
        });
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

    pub fn configure_alarm(&self, time: &Time) {
        // Disable the write protection for RTC registers.
        self.peripherals.RTC.wpr.write(|w| unsafe { w.bits(0xCA) });
        self.peripherals.RTC.wpr.write(|w| unsafe { w.bits(0x53) });

        // Disable alarm A to modify it.
        self.toggle_alarm(false);

        // Wait until it is allowed to modify alarm A value.
        while self.peripherals.RTC.isr.read().alrawf().bit_is_clear() {}

        // Modify alarm A mask to have an interrupt each minute.
        self.peripherals.RTC.alrmar.modify(|_, w| {
            unsafe {
                w.ht().bits(time.hours / 10);
                w.hu().bits(time.hours % 10);
                w.mnt().bits(time.minutes / 10);
                w.mnu().bits(time.minutes % 10);
                w.st().bits(time.seconds / 10);
                w.su().bits(time.seconds % 10);
            }

            w.msk1().clear_bit();
            w.msk2().set_bit();
            w.msk3().set_bit();
            w.msk4().set_bit()
        });

        // Enable alarm A and alarm A interrupt.
        self.toggle_alarm(true);

        // Enable the write protection for RTC registers.
        self.peripherals.RTC.wpr.write(|w| unsafe { w.bits(0xFE) });
        self.peripherals.RTC.wpr.write(|w| unsafe { w.bits(0x64) });
    }

    pub fn toggle_alarm(&self, enable: bool) {
        self.peripherals.RTC.cr.modify(|_, w| {
            if enable {
                w.alraie().set_bit();
                w.alrae().set_bit()
            } else {
                w.alraie().clear_bit();
                w.alrae().clear_bit()
            }
        });
    }

    pub fn configure_time(&self, time: &Time) {
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
                w.ht().bits(time.hours / 10);
                w.hu().bits(time.hours % 10);
                w.mnt().bits(time.minutes / 10);
                w.mnu().bits(time.minutes % 10);
                w.st().bits(time.seconds / 10);
                w.su().bits(time.seconds % 10);
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
            seconds: tr.st().bits() * 10 + tr.su().bits(),
        }
    }

    fn configure_interrupts(&mut self) {
        // Unmask line 17, EXTI line 17 is connected to the RTC Alarm event.
        self.peripherals.EXTI.imr.modify(|_, w| w.mr17().set_bit());
        // Rising edge for line 17.
        self.peripherals.EXTI.rtsr.modify(|_, w| w.tr17().set_bit());
        // Set priority.
        unsafe {
            self.core_peripherals.NVIC.set_priority(Interrupt::RTC, 0);
        }
        // Enable RTC_IRQn in the NVIC.
        self.core_peripherals.NVIC.enable(Interrupt::RTC);
    }
}
