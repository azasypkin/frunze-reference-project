#ifndef REFERENCE_PROJECT_PISO_H
#define REFERENCE_PROJECT_PISO_H

#include <avr/io.h>

volatile struct {
  uint8_t data, pload, clock;
} piso;

void setup_piso(uint8_t dataPin, uint8_t ploadPin, uint8_t clockPin) {
  piso.data = _BV(dataPin);
  piso.pload = _BV(ploadPin);
  piso.clock = _BV(clockPin);

  // Initialise pload high and clock low.
  DDRB |= (piso.pload | piso.clock);
  PORTB |= piso.pload;
  PORTB &= ~piso.clock;

  // Enable pull-ups.
  DDRB &= ~piso.data;
  PORTB |= piso.data;
}

int shift_in(void) {
  int pisoVal = 0, bitVal = 0;

  // Load parallel values to '165.
  PORTB &= ~piso.pload;
  PORTB |= piso.pload;

  // Get bits stored in '165.
  for (uint8_t i = 0; i < 8; i++) {
    // read value of data pin.
    bitVal = PINB & piso.data;

    pisoVal |= (bitVal << (7 - i));

    // Cycle clock for next value.
    PORTB |= piso.clock;
    PORTB &= ~piso.clock;
  }

  return pisoVal;
}

#endif //REFERENCE_PROJECT_PISO_H
