#ifndef MIKATRON_SPEAKER_H
#define MIKATRON_SPEAKER_H

// Example for Note C:
// Frequency for note C4 is 261.63 Hz (see http://www.phy.mtu.edu/~suits/notefreqs.html),
// Divisor for this note will be = 1000000 (1 Mhz clock) / 261.63 = 3822.19.
// However, the divider for the counter must fit into 8 bits, so the maximum number we can
// represent is 255. The solution is to set the prescaler to divide the clock by 2^4, or 16.
// Then we get: divisor = 1000000 / 16 / 261.63 = 238.89 which rounds to 239.
const uint8_t Note_C = 239;
const uint8_t Note_CS = 225;
const uint8_t Note_D = 213;
const uint8_t Note_DS = 201;
const uint8_t Note_E = 190;
const uint8_t Note_F = 179;
const uint8_t Note_FS = 169;
const uint8_t Note_G = 159;
const uint8_t Note_GS = 150;
const uint8_t Note_A = 142;
const uint8_t Note_AS = 134;
const uint8_t Note_B = 127;

struct SpeakerNote {
    uint8_t tone;
    uint8_t octave;
    uint32_t duration;
};

struct Melody {
    uint8_t length;
    const SpeakerNote *notes;
};

extern const Melody MELODY_BEEP;
extern const Melody MELODY_ALARM;
extern const Melody MELODY_DOUBLE_BEEP;
extern const Melody MELODY_MODE;

class Speaker {
public:
  static void play(Melody melody);
};

#endif //MIKATRON_SPEAKER_H
