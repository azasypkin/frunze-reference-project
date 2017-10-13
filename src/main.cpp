#include <avr/io.h>
#include <avr/interrupt.h>
#include <util/delay.h>
#include "piso.h"
#include <stdlib.h>

#define DEBUG_ENABLED true

#if(DEBUG_ENABLED)
#include "uart.h"
#endif

volatile bool interrupt = false;

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

/**
 * GPIO Layout:
 *  - PB0 (pin 5) - I2C SDA;
 *  - PB1 (pin 6) - PWM (speaker) + UART Rx/Tx;
 *  - PB2 (pin 7) - I2C SCL;
 *  - PB3 (pin 2) - Alarm Interruption Pin;
 *  - PB4 (pin 3) - Snooze Button Pin.
 *
 *  - PB0 (pin 5) - Shift Register Data
 *  - PB2 (pin 7) - Shift Register Parallel Load
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

  setup_piso(DDB0, DDB2, DDB4);

  debug("[app] all setup.");

  while (1) {
    _delay_ms(1000);
    debug("Iterating...");

    char buffer[8];
    itoa(shift_in(), buffer, 2);
    debug(buffer);
  }
}