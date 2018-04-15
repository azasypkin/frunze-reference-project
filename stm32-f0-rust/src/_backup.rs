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

mod rtc;

use core::cell::RefCell;
use core::fmt::Write;

use cortex_m::asm;
use cortex_m::interrupt::{self, Mutex};
use cortex_m_semihosting::hio;
use stm32f0x1::Interrupt;

use rtc::rtc::RTC;

static CORE_PERIPHERALS: Mutex<RefCell<Option<cortex_m::Peripherals>>> =
    Mutex::new(RefCell::new(None));
static PERIPHERALS: Mutex<RefCell<Option<stm32f0x1::Peripherals>>> = Mutex::new(RefCell::new(None));

const SYNCHRONISATION_TIMEOUT: u32 = 0x0000_8000;
const INIT_TIMEOUT: u32 = 0x0000_4000;

const CLOCK_SPEED: u32 = 8_000_000;
const QUARTER_DOT_NOTE: u32 = 450;
const QUARTER_NOTE: u32 = 300;
const EIGHTH_NOTE: u32 = 150;

static SCALES: &'static [u32] = &[523, 554, 587, 622, 659, 698, 740, 784, 831, 880, 932, 988];

fn configure_pwm() {
    // Compute the value to be set in ARR register to generate signal frequency at 17.57 Khz.
    let timer_period: u32 = (CLOCK_SPEED / 17_570) - 1;
    // Compute CCR1 value to generate a duty cycle at 50% for channel 1 and 1N.
    let channel_one_pulse: u32 = (5 * (timer_period - 1)) / 10;
    interrupt::free(|cs| {
        if let Some(peripherals) = PERIPHERALS.borrow(cs).borrow_mut().as_mut() {
            let rcc = &peripherals.RCC;

            // Enable clock for GPIO Port A.
            rcc.ahbenr.modify(|_, w| w.iopaen().set_bit());
            // Enable TIM1 clock.
            rcc.apb2enr.modify(|_, w| w.tim1en().set_bit());

            let gpio = &peripherals.GPIOA;
            // Switch PA8 to alternate function mode.
            gpio.moder.modify(|_, w| unsafe { w.moder8().bits(0b10) });
            // No pull-up, pull-down.
            gpio.pupdr.modify(|_, w| unsafe { w.pupdr8().bits(0b0) });
            // Set "high" output speed.
            gpio.ospeedr
                .modify(|_, w| unsafe { w.ospeedr8().bits(0b11) });
            // Set alternative function #2.
            gpio.afrh.modify(|_, w| unsafe { w.afrh8().bits(0b0010) });

            let tim = &peripherals.TIM1;
            // Set prescaler, the counter clock frequency (CK_CNT) is equal to
            // f(CK_PSC) / (PSC[15:0] + 1).
            tim.psc.modify(|_, w| unsafe { w.bits(0b0) });

            tim.cr1.modify(|_, w| {
                // Set direction: counter used as up-counter.
                w.dir().clear_bit();
                // Set clock division to t(DTS) = t(CK_INT).
                unsafe {
                    w.ckd().bits(0b00);
                }
                w
            });

            // Set value to auto-reload register.
            tim.arr.write(|w| unsafe { w.bits(timer_period) });
            // Set repetition counter.
            tim.rcr.write(|w| unsafe { w.bits(0b0) });
            // Enable PWM mode 2 - In up-counting, channel 1 is inactive as long as TIMx_CNT<TIMx_CCR1
            // else active. In down-counting, channel 1 is active as long as TIMx_CNT>TIMx_CCR1 else
            //inactive.
            tim.ccmr1_output
                .modify(|_, w| unsafe { w.oc1m().bits(0b111) });

            // Configure capture/compare enable register.
            tim.ccer.modify(|_, w| {
                // Enable Capture/Compare 1 output.
                w.cc1e().set_bit();
                // Enable Capture/Compare 1 complementary output.
                w.cc1ne().set_bit();
                // Set low polarity for Capture/Compare 1 output.
                w.cc1p().set_bit();
                // Set high polarity for Capture/Compare complementary 1 output.
                w.cc1np().clear_bit();
                w
            });

            // CCR1 is the value to be loaded in the actual capture/compare 1 register (preload value).
            tim.ccr1.write(|w| unsafe { w.bits(channel_one_pulse) });

            // Configure control register 2.
            tim.cr2.modify(|_, w| {
                // Set output Idle state 1 (OC1 output).
                w.ois1().set_bit();
                // Set output Idle state 1 (OC1N output).
                w.ois1n().clear_bit();
                w
            });

            // Enable counter.
            tim.cr1.modify(|_, w| w.cen().set_bit());
        }
    });
}

fn toggle_pwm(enable: bool) {
    interrupt::free(|cs| {
        if let Some(peripherals) = PERIPHERALS.borrow(cs).borrow_mut().as_mut() {
            peripherals.TIM1.bdtr.modify(|_, w| {
                if enable {
                    w.moe().set_bit()
                } else {
                    w.moe().clear_bit()
                }
            });
        }
    });
}

fn toggle_rtc_write_protection(peripherals: &stm32f0x1::Peripherals, enable: bool) {
    peripherals.PWR.cr.modify(|_, w| {
        if enable {
            w.dbp().set_bit()
        } else {
            w.dbp().clear_bit()
        }
    });
}

// Enables or disables the Internal Low Speed oscillator (LSI).
fn toggle_lsi(peripherals: &stm32f0x1::Peripherals, enable: bool) {
    peripherals.RCC.csr.modify(|_, w| {
        if enable {
            w.lsion().set_bit()
        } else {
            w.lsion().clear_bit()
        }
    });
}

// Enables or disables the RTC clock.
fn toggle_rtc_clock(peripherals: &stm32f0x1::Peripherals, enable: bool) {
    peripherals.RCC.bdcr.modify(|_, w| {
        if enable {
            w.rtcen().set_bit()
        } else {
            w.rtcen().clear_bit()
        }
    });
}

// Enables or disables the specified RTC interrupts.
fn toggle_rtc_interrupt(peripherals: &stm32f0x1::Peripherals, enable: bool) {
    // Disable the write protection for RTC registers.
    peripherals.RTC.wpr.write(|w| unsafe { w.bits(0xCA) });
    peripherals.RTC.wpr.write(|w| unsafe { w.bits(0x53) });

    // Configure the ALARM A in the RTC_CR register
    peripherals.RTC.cr.modify(|_, w| {
        if enable {
            w.alraie().set_bit()
        } else {
            w.alraie().clear_bit()
        }
    });

    // Enable the write protection for RTC registers.
    peripherals.RTC.wpr.write(|w| unsafe { w.bits(0xFF) });
}

// Waits until the RTC Time and Date registers (RTC_TR and RTC_DR) are synchronized with
// RTC APB clock.
fn wait_for_rtc_synchronisation(peripherals: &stm32f0x1::Peripherals) -> Result<(), ()> {
    // Check if we're in bypass shadow mode.
    if peripherals.RTC.cr.read().bypshad().bit_is_set() {
        return Ok(());
    }

    // Disable the write protection for RTC registers.
    peripherals.RTC.wpr.write(|w| unsafe { w.bits(0xCA) });
    peripherals.RTC.wpr.write(|w| unsafe { w.bits(0x53) });

    // Clear RSF flag.
    peripherals.RTC.isr.modify(|_, w| w.rsf().clear_bit());

    // Wait the registers to be synchronised.
    let mut synchronisation_status = false;
    let mut synchronisation_counter = 0;
    while synchronisation_counter != SYNCHRONISATION_TIMEOUT && !synchronisation_status {
        synchronisation_status = peripherals.RTC.isr.read().rsf().bit_is_set();
        synchronisation_counter = synchronisation_counter + 1;
    }

    let result;
    if peripherals.RTC.isr.read().rsf().bit_is_set() {
        result = Ok(());
    } else {
        result = Err(());
    }

    // Enable the write protection for RTC registers.
    peripherals.RTC.wpr.write(|w| unsafe { w.bits(0xFF) });

    result
}

fn init_rtc(peripherals: &stm32f0x1::Peripherals) -> Result<(), ()> {
    // Disable the write protection for RTC registers.
    peripherals.RTC.wpr.write(|w| unsafe { w.bits(0xCA) });
    peripherals.RTC.wpr.write(|w| unsafe { w.bits(0x53) });

    let result;
    if enter_rtc_init_mode(peripherals).is_err() {
        result = Err(());
    } else {
        // 24 hour/day format.
        peripherals.RTC.cr.modify(|_, w| w.fmt().clear_bit());

        // Set prescaler, 40kHz/128 (0x7F + 1) => 312 Hz, 312Hz/312 (0x137 + 1) => 1Hz.
        peripherals
            .RTC
            .prer
            .modify(|_, w| unsafe { w.prediv_s().bits(0x137) });

        peripherals
            .RTC
            .prer
            .modify(|_, w| unsafe { w.prediv_a().bits(0x7F) });

        exit_rtc_init_mode(peripherals);

        result = Ok(());
    }

    // Enable the write protection for RTC registers.
    peripherals.RTC.wpr.write(|w| unsafe { w.bits(0xFF) });

    result
}

// Convert from 2 digit BCD to Binary.
fn bcd_to_byte(value: u8) -> u8 {
    ((value & 0xF0) >> 0x4) * 10 + (value & 0x0F)
}

fn set_alarm(peripherals: &stm32f0x1::Peripherals) {
    // Disable the write protection for RTC registers.
    peripherals.RTC.wpr.write(|w| unsafe { w.bits(0xCA) });
    peripherals.RTC.wpr.write(|w| unsafe { w.bits(0x53) });

    // Configure the Alarm register.
    peripherals.RTC.alrmar.modify(|_, w| {
        unsafe {
            w.hu().bits(0x1);
            w.mnu().bits(0x0);
            w.su().bits(0x2);
            w.du().bits(0x1);
        }

        w.msk3().set_bit();
        w.msk4().set_bit()
    });

    // Enable the write protection for RTC registers.
    peripherals.RTC.wpr.write(|w| unsafe { w.bits(0xFF) });
}

//  Set the RTC current time.
fn set_time(peripherals: &stm32f0x1::Peripherals) -> Result<(), ()> {
    // Disable the write protection for RTC registers.
    peripherals.RTC.wpr.write(|w| unsafe { w.bits(0xCA) });
    peripherals.RTC.wpr.write(|w| unsafe { w.bits(0x53) });

    let result;
    if enter_rtc_init_mode(peripherals).is_err() {
        result = Err(());
    } else {
        // Configure Time register.
        peripherals.RTC.tr.modify(|_, w| {
            unsafe {
                w.hu().bits(0x01);
                w.mnu().bits(0x0);
                w.su().bits(0x0);
            }

            w
        });

        exit_rtc_init_mode(peripherals);

        // If  RTC_CR_BYPSHAD bit = 0, wait for synchronisation.
        if peripherals.RTC.cr.read().bypshad().bit_is_set() {
            result = Ok(());
        } else if wait_for_rtc_synchronisation(peripherals).is_ok() {
            result = Ok(());
        } else {
            result = Err(());
        }
    }

    // Enable the write protection for RTC registers.
    peripherals.RTC.wpr.write(|w| unsafe { w.bits(0xFF) });

    result
}

fn toggle_alarm(peripherals: &stm32f0x1::Peripherals, enable: bool) -> Result<(), ()> {
    // Disable the write protection for RTC registers.
    peripherals.RTC.wpr.write(|w| unsafe { w.bits(0xCA) });
    peripherals.RTC.wpr.write(|w| unsafe { w.bits(0x53) });

    peripherals.RTC.cr.modify(|_, w| {
        if enable {
            w.alrae().set_bit()
        } else {
            w.alrae().clear_bit()
        }
    });

    let result;
    if enable {
        result = Ok(());
    } else {
        // Wait till RTC ALRAWF flag is set or if timeout is reached.
        let mut alarm_status = false;
        let mut alarm_counter = 0;
        while alarm_counter != INIT_TIMEOUT && !alarm_status {
            alarm_status = peripherals.RTC.isr.read().alrawf().bit_is_set();
            alarm_counter = alarm_counter + 1;
        }

        if peripherals.RTC.isr.read().alrawf().bit_is_clear() {
            result = Err(());
        } else {
            result = Ok(());
        }
    }

    // Enable the write protection for RTC registers.
    peripherals.RTC.wpr.write(|w| unsafe { w.bits(0xFF) });

    result
}

// Enters the RTC Initialization mode.
fn enter_rtc_init_mode(peripherals: &stm32f0x1::Peripherals) -> Result<(), ()> {
    // Check if the Initialization mode is set.
    if peripherals.RTC.isr.read().initf().bit_is_set() {
        return Ok(());
    }

    // Set the Initialization mode
    peripherals.RTC.isr.write(|w| w.init().set_bit());

    // Wait till RTC is in INIT state and if Time out is reached exit.
    let mut init_status = false;
    let mut init_counter = 0;
    while init_counter <= INIT_TIMEOUT && !init_status {
        init_status = peripherals.RTC.isr.read().initf().bit_is_set();
        init_counter = init_counter + 1;
    }

    if peripherals.RTC.isr.read().initf().bit_is_set() {
        Ok(())
    } else {
        Err(())
    }
}

// Exits from the RTC Initialization mode.
fn exit_rtc_init_mode(peripherals: &stm32f0x1::Peripherals) {
    peripherals.RTC.isr.write(|w| w.init().clear_bit());
}

fn toggle_rtc(peripherals: &stm32f0x1::Peripherals, enable: bool) {
    peripherals.RCC.bdcr.modify(|_, w| {
        if enable {
            w.bdrst().set_bit()
        } else {
            w.bdrst().clear_bit()
        }
    });
}

fn configure_rtc() {
    interrupt::free(|cs| {
        if let Some(peripherals) = PERIPHERALS.borrow(cs).borrow_mut().as_mut() {
            peripherals.RCC.apb1enr.modify(|_, w| w.pwren().set_bit());

            toggle_rtc(peripherals, true);
            toggle_rtc(peripherals, false);

            toggle_rtc_write_protection(peripherals, true);
            toggle_lsi(peripherals, true);

            // Wait for LSI to stabilize.
            while peripherals.RCC.csr.read().lsirdy().bit_is_clear() {}

            // Select the RTC clock source as LSI.
            peripherals
                .RCC
                .bdcr
                .modify(|_, w| unsafe { w.rtcsel().bits(0b10) });

            toggle_rtc_clock(peripherals, true);

            wait_for_rtc_synchronisation(peripherals).unwrap_or_else(|_| {
                let mut stdout = hio::hstdout().unwrap();
                writeln!(stdout, "Clock synchronisation failed.").unwrap();
            });

            init_rtc(peripherals).unwrap_or_else(|_| {
                let mut stdout = hio::hstdout().unwrap();
                writeln!(stdout, "RTC initialisation failed.").unwrap();
            });

            set_alarm(peripherals);

            toggle_rtc_interrupt(peripherals, true);

            toggle_alarm(peripherals, true).unwrap_or_else(|_| {
                let mut stdout = hio::hstdout().unwrap();
                writeln!(stdout, "Alarm failed to enable.").unwrap();
            });

            set_time(peripherals).unwrap_or_else(|_| {
                let mut stdout = hio::hstdout().unwrap();
                writeln!(stdout, "Set time failed.").unwrap();
            });

            // Clear Wakeup flag.
            peripherals.PWR.cr.modify(|_, w| w.cwuf().set_bit());

            // Clear Alarm A flag.
            peripherals.RTC.isr.modify(|_, w| w.alraf().clear_bit());
        }
    });
}

fn delay_us(us: u32) {
    interrupt::free(|cs| {
        if let Some(peripherals) = CORE_PERIPHERALS.borrow(cs).borrow_mut().as_mut() {
            let mut sys_tick = &mut peripherals.SYST;

            let rvr = us * (CLOCK_SPEED / 1_000_000);

            assert!(rvr < (1 << 24));

            sys_tick.set_reload(rvr);
            sys_tick.clear_current();
            sys_tick.enable_counter();

            while !sys_tick.has_wrapped() {}

            sys_tick.disable_counter();
        }
    });
}

fn delay_ms(ms: u32) {
    delay_us(ms * 1000);
}

fn play_note(note: u32, delay: u32) {
    interrupt::free(|cs| {
        if let Some(peripherals) = PERIPHERALS.borrow(cs).borrow_mut().as_mut() {
            let tim = &peripherals.TIM1;
            tim.arr
                .write(|w| unsafe { w.bits((CLOCK_SPEED / note) - 1) });
        }
    });

    delay_ms(delay);
}

fn play_melody() {
    toggle_pwm(true);

    play_note(SCALES[7], QUARTER_NOTE); // G
    play_note(SCALES[7], QUARTER_NOTE); // G
    play_note(SCALES[8], QUARTER_NOTE); // A
    play_note(SCALES[10], QUARTER_NOTE); // B
    play_note(SCALES[10], QUARTER_NOTE); // B
    play_note(SCALES[8], QUARTER_NOTE); // A
    play_note(SCALES[7], QUARTER_NOTE); // G
    play_note(SCALES[5], QUARTER_NOTE); // F
    play_note(SCALES[3], QUARTER_NOTE); // D#
    play_note(SCALES[3], QUARTER_NOTE); // E
    play_note(SCALES[5], QUARTER_NOTE); // F
    play_note(SCALES[7], QUARTER_NOTE); // G
    play_note(SCALES[7], QUARTER_DOT_NOTE); // G.
    play_note(SCALES[5], EIGHTH_NOTE); // F
    play_note(SCALES[5], QUARTER_DOT_NOTE); // F.

    toggle_pwm(false);
}

// Enters STANDBY mode.
// In Standby mode, all I/O pins are high impedance except for:
//   - Reset pad (still available)
//   - RTC_AF1 pin (PC13) if configured for Wakeup pin 2 (WKUP2), tamper,
//     time-stamp, RTC Alarm out, or RTC clock calibration out.
//   - WKUP pin 1 (PA0) if enabled.
fn enter_standby_mode() {
    interrupt::free(|cs| {
        if let (Some(peripherals), Some(core_peripherals)) = (
            PERIPHERALS.borrow(cs).borrow_mut().as_mut(),
            CORE_PERIPHERALS.borrow(cs).borrow_mut().as_mut(),
        ) {
            peripherals.PWR.cr.modify(|_, w| {
                // Clear Wakeup flag.
                w.cwuf().set_bit();
                // Select STANDBY mode.
                w.pdds().set_bit()
            });

            // Set SLEEPDEEP bit of Cortex-M0 System Control Register.
            unsafe { core_peripherals.SCB.scr.modify(|w| w | 0b100) }
        }
    });
}

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
