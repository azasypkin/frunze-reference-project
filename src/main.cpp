#include <avr/io.h>
#include <avr/interrupt.h>
#include <avr/sleep.h>
#include <util/delay.h>
#include "piso.h"
#include <stdlib.h>

#define DEBUG_ENABLED true

#if(DEBUG_ENABLED)

#include "uart.h"

#endif

volatile bool interrupt = false;
volatile uint8_t interruptDuration = 0;

ISR(PCINT0_vect) {
  interrupt = true;
}

void debug(const char *str, bool newLine = true) {
#if(DEBUG_ENABLED)
  while (*str) {
    TxByte(*str++);
  }

  if (newLine) {
    TxByte('\n');
  }
#endif
}

void printNumber(uint8_t number) {
  char buffer[8];
  itoa(number, buffer, 2);
  debug(buffer);
}

void printShiftRegister() {
  debug("Shift ", false);
  printNumber(shift_in());
}

void enablePowerDownMode() {
  debug("[app] going to power down...");

  PORTB &= ~_BV(PB1);

  set_sleep_mode(SLEEP_MODE_PWR_DOWN);
  cli();
  sleep_enable();
  sei();
  sleep_cpu();
  sleep_disable();
  sei();

  PORTB |= _BV(PB1);

  debug("[app] waken up!");
}

/**
 * GPIO Layout:
 *  - PB0 (pin 5) - I2C SDA;
 *  - PB1 (pin 6) - PWM (speaker) + UART Rx/Tx;
 *  - PB2 (pin 7) - I2C SCL;
 *  - PB3 (pin 2) - Alarm Interruption Pin;
 *  - PB4 (pin 3) - Snooze Button Pin.
 *
 *  - PB0 (pin 5) - Shift Register Data
 *  - PB1 (pin 6) - PWM (speaker) + UART Rx/Tx;
 *  - PB2 (pin 7) - Shift Register Parallel Load
 *  - PB3 (pin 2) - Reset Interruption Pin;
 *  - PB4 (pin 3) - Shift Register Clock
 *
 * UART Rx/Tx Layout:
 *            D1
 * AVR ----+--|>|-----+----- Tx
 *         |      10K $ R1
 *         +--------(/^\)--- Rx
 *              NPN E   C
 */
int main(void) {
  // Setup outputs. Set port to HIGH to signify UART default condition.
  DDRB |= _BV(DDB1);
  PORTB |= _BV(PB1);

  setup_piso(PB0, PB2, PB4);

  // Enable interruption that comes from Reset/Set button.
  DDRB &= ~_BV(DDB3);
  PCMSK |= _BV(PCINT3);
  GIMSK |= _BV(PCIE);

  debug("[app] all setup.");

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wmissing-noreturn"
  while (1) {
    _delay_ms(100);
    debug("Iterating... ", false);
    printNumber((PINB & (1 << PB3)));

    if (interrupt && interruptDuration >= 20) {
      printShiftRegister();

      interrupt = false;
      interruptDuration = 0;
    } else if (interrupt && !(PINB & (1 << PB3))) {
      interruptDuration++;
    } else {
      interrupt = false;
      interruptDuration = 0;

      enablePowerDownMode();
    }
  }
#pragma clang diagnostic pop
}