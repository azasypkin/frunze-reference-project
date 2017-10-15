#include <avr/io.h>
#include <avr/interrupt.h>
#include <avr/sleep.h>
#include <util/delay.h>
#include "piso.h"
#include "speaker.h"

#define DEBUG_ENABLED true

#if(DEBUG_ENABLED)

#include "uart.h"

#endif

volatile uint8_t resetButtonPressDuration = 0;

enum Mode {
  SLEEP,
  INPUT
};

static const char * ModeStrings[] = { "SLEEP", "INPUT" };

volatile Mode mode = Mode::SLEEP;

ISR(PCINT0_vect) {
  resetButtonPressDuration = 0;
}

void debug(const char *str, bool newLine = true) {
#if(DEBUG_ENABLED)
  while (*str) {
    TxByte(*str++);
  }

  if (newLine) {
    TxByte('\r');
    TxByte('\n');
  }
#endif
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

bool isResetPressed() {
  return !(PINB & _BV(PB3));
}

bool isResetLongPressed() {
  return isResetPressed() && resetButtonPressDuration >= 20;
}

void readButtons() {
  uint8_t shift = shift_in();
  switch (shift) {
    case 1:
      TxByte('0');
      break;
    case 2:
      TxByte('1');
      break;
    case 4:
      TxByte('2');
      break;
    case 8:
      TxByte('3');
      break;
    case 16:
      TxByte('4');
      break;
    case 32:
      TxByte('5');
      break;
    default:
      TxByte('.');
      break;
  }

  if (shift) {
    Speaker::play(MELODY_BEEP);
  }
}

void toggleMode() {
  mode = mode == Mode::SLEEP ? Mode::INPUT : Mode::SLEEP;
  resetButtonPressDuration = 0;

  debug("Mode toggled to: ", false);
  debug(ModeStrings[mode]);
  Speaker::play(MELODY_MODE);
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

  Speaker::play(MELODY_ALARM);

#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wmissing-noreturn"
  while (1) {
    enablePowerDownMode();

    while(1) {
      _delay_ms(100);

      if (mode == Mode::SLEEP && !isResetPressed()) {
        break;
      }

      if (isResetLongPressed()) {
        toggleMode();

        if (mode == Mode::SLEEP) {
          break;
        }
      }

      if (isResetPressed()) {
        resetButtonPressDuration++;
      } else if (mode == INPUT) {
        readButtons();
      }
    }
  }
#pragma clang diagnostic pop
}