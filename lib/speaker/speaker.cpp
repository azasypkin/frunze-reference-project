#include <avr/io.h>
#include <util/delay.h>
#include "speaker.h"

// Taken from http://www.technoblogy.com/show?KVO.

const SpeakerNote BEEP_NOTES[1] =         {{ Note_C, 4, 150 }};
const SpeakerNote DOUBLE_BEEP_NOTES[2] =  {{ Note_C, 4, 150 }, { Note_D, 4, 150 }};
const SpeakerNote ALARM_NOTES[8] =        {{ Note_C, 4, 50 }, { Note_D, 4, 50 }, { Note_E, 4, 50 }, { Note_F, 4, 50 },
                                           { Note_G, 4, 50 }, { Note_A, 4, 50 }, { Note_B, 4, 50 }, { Note_C, 4, 50 }};
const SpeakerNote MODE_NOTES[6] =         {{ Note_C, 4, 50 }, { Note_D, 4, 50 }, { Note_E, 4, 50 }, { Note_C, 4, 50 },
                                           { Note_D, 4, 50 }, { Note_E, 4, 50 }};

const Melody MELODY_BEEP =        {1, BEEP_NOTES};
const Melody MELODY_ALARM =       {8, ALARM_NOTES};
const Melody MELODY_DOUBLE_BEEP = {2, DOUBLE_BEEP_NOTES};
const Melody MELODY_MODE =        {6, MODE_NOTES};

void Speaker::play(Melody melody) {
  const uint8_t TCCR1_BAK = TCCR1;

  for (uint8_t i = 0; i < melody.length; i++) {
    const SpeakerNote note = melody.notes[i];

    // For 1MHz clock.
    // 1. Timer/Counter1 is reset to $00 in the CPU clock cycle after a compare match with
    // OCR1A register;
    // 2. Output pin action following a compare match with compare register A in Timer/Counter1 will
    // be: "Toggle the OC1A output line.".
    TCCR1 = (1 << CTC1) | (1 << COM1A0) | (8 - note.octave);

    // for 8MHz clock
    // TCCR1 = (1 << CTC1) | (1 << COM1A0) | (11-octave);

    // Set the OCR.
    OCR1C = note.tone - 1;

    for (uint32_t i = 0; i < note.duration; i++) {
      _delay_ms(1);
    }

    // Stop the counter.
    TCCR1 = TCCR1_BAK;
  }
}