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

const QUARTER_DOT_NOTE: u32 = 450;
const QUARTER_NOTE: u32 = 300;
const EIGHTH_NOTE: u32 = 150;

static  SCALES: &'static [u32] = &[523,  554,  587,  622,  659,  698,  740,  784,  831,  880,  932,  988];

fn configure_pwm() {
    // Compute the value to be set in ARR register to generate signal frequency at 17.57 Khz.
    let timer_period: u32 = (8_000_000 / 17_570) - 1;
    // Compute CCR1 value to generate a duty cycle at 50% for channel 1 and 1N.
    let channel_one_pulse: u32 = (5 * (timer_period - 1)) / 10;

    if let Some(peripherals) = stm32f0x1::Peripherals::take() {
        let rcc = peripherals.RCC;

        // Enable clock for GPIO Port A.
        rcc.ahbenr.modify(|_, w| w.iopaen().set_bit());
        // Enable TIM1 clock.
        rcc.apb2enr.modify(|_, w| w.tim1en().set_bit());

        let gpio = peripherals.GPIOA;
        // Switch PA8 to alternate function mode.
        gpio.moder.modify(|_, w| unsafe { w.moder8().bits(0b10) });
        // No pull-up, pull-down.
        gpio.pupdr.modify(|_, w| unsafe { w.pupdr8().bits(0b0) });
        // Set "high" output speed.
        gpio.ospeedr.modify(|_, w| unsafe { w.ospeedr8().bits(0b11) });
        // Set alternative function #2.
        gpio.afrh.modify(|_, w| unsafe { w.afrh8().bits(0b0010) });

        let tim = peripherals.TIM1;
        // Set prescaler, the counter clock frequency (CK_CNT) is equal to
        // f(CK_PSC) / (PSC[15:0] + 1).
        tim.psc.modify(|_, w| unsafe { w.bits(0b0) });

        tim.cr1.modify(|_, w| {
            // Set direction: counter used as upcounter.
            w.dir().clear_bit();
            // Set clock division to t(DTS) = t(CK_INT).
            unsafe { w.ckd().bits(0b00); }
            w
        });

        // Set value to auto-reload register.
        tim.arr.write(| w| unsafe { w.bits(timer_period) });
        // Set repetition counter.
        tim.rcr.write(|w| unsafe { w.bits(0b0) });
        // Enable PWM mode 2 - In up-counting, channel 1 is inactive as long as TIMx_CNT<TIMx_CCR1
        // else active. In down-counting, channel 1 is active as long as TIMx_CNT>TIMx_CCR1 else
        //inactive.
        tim.ccmr1_output.modify(|_, w| unsafe { w.oc1m().bits(0b111) });

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
}

fn toggle_pwm(enable: bool) {
    if let Some(peripherals) = stm32f0x1::Peripherals::take() {
        let tim = peripherals.TIM1;

        if enable {
            tim.bdtr.modify(|_, w| w.moe().set_bit());
        } else {
            tim.bdtr.modify(|_, w| w.moe().clear_bit());
        }
    }
}

fn delay_ms(us: u32) {
    if let Some(peripherals) = cortex_m::Peripherals::take() {
        let mut sys_tick = peripherals.SYST;

        let rvr = us * (8_000_000 / 1_000_000);

        assert!(rvr < (1 << 24));

        sys_tick.set_reload(rvr);
        sys_tick.clear_current();
        sys_tick.enable_counter();

        while !sys_tick.has_wrapped() {}

        sys_tick.disable_counter();
    }
}

fn play_note(note: u32, delay: u32) {
    if let Some(peripherals) = stm32f0x1::Peripherals::take() {
        let tim = peripherals.TIM1;
        tim.arr.write(|w| unsafe { w.bits((8_000_000 / note) - 1) });
        delay_ms(delay);
    }
}

fn play_melody() {
    toggle_pwm(true);

    play_note(SCALES[7], QUARTER_NOTE);       // G
    play_note(SCALES[7], QUARTER_NOTE);       // G
    play_note(SCALES[8], QUARTER_NOTE);       // A
    play_note(SCALES[10], QUARTER_NOTE);       // B
    play_note(SCALES[10], QUARTER_NOTE);       // B
    play_note(SCALES[8], QUARTER_NOTE);       // A
    play_note(SCALES[7], QUARTER_NOTE);       // G
    play_note(SCALES[5], QUARTER_NOTE);       // F
    play_note(SCALES[3], QUARTER_NOTE);       // D#
    play_note(SCALES[3], QUARTER_NOTE);       // E
    play_note(SCALES[5], QUARTER_NOTE);       // F
    play_note(SCALES[7], QUARTER_NOTE);       // G
    play_note(SCALES[7], QUARTER_DOT_NOTE);   // G.
    play_note(SCALES[5], EIGHTH_NOTE);        // F
    play_note(SCALES[5], QUARTER_DOT_NOTE);   // F.
    
    toggle_pwm(false);
}

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

        configure_pwm();
        play_melody();

        //asm::wfi();

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
