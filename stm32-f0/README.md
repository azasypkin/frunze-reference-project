# Frunze Reference Project

Project template is generated with the following command:

```
platformio init --ide clion --board disco_f051r8
```

Framework should be set to [cmsis](http://docs.platformio.org/en/latest/frameworks/cmsis.html).

## Custom platform

Currently `platformio` does not support `cmsis` framework for `stm32f0` MCUs, so we should point to
the [forked and patched platform instead](https://github.com/azasypkin/platform-ststm32/releases/tag/v4.1.1).

The patched `platform-ststm32` requires the following changes in `platformio.ini`:

* `platform = https://github.com/azasypkin/platform-ststm32.git#stm32f0-cmsis` - to point to the patched platform;
* `build_unflags = -nostdlib -nostartfiles` - required for the `__libc_init_array` to work with `Platformio`


## Printed circuit boards

### `TSSOP-20` breakout board

First we need to manufacture `TSSOP-20` breakout board for `stm32f042f4p6`. The process is the following:

1. Download [Adafruit SMT Breakout for 20-pin SOIC+TSSOP](https://github.com/adafruit/Adafruit-SMT-Breakout-PCBs/blob/master/20-pin%20SOIC%2BTSSOP.brd)
2. Then `Import Non-KiCad Board File` in `KiCad` and save it as `KiCad` board. See `docs/schematics/TSSOP-20x6.5mmx4.4mm-Breakout/board.kicad_pcb`
3. Open the board in `KiCad` and choose `Place the origin point for drill and place files` at the toolbox and set the origin 
point to the bottom left corner of the board cutout
4. Generate gerber (for the bottom layer only and edge cuts) and drill files, open them in the `Flatcam`
5. Generate tool paths (see `docs/schematics/TSSOP-20x6.5mmx4.4mm-Breakout/cnc/flatcam`) and export G-Code

To program `stm32f042f4p6` with `STM32F0DISCOVERY` remove the 2 jumpers from CN2 (see `en.DM00050135.pdf`, page 16):

| `STM32F0DISCOVERY` CN3/SWD connector | `STM32F042F4P6`           |
| ------------------------------------ | -------------------------:|
| 1 - VDD_TARGET - VDD from target MCU | 16 - VDD __and__ 5 - VDDA |
| 2 - SWCLK - SWD clock                | 20 - PA14                 |
| 3 - GND - Ground                     | 15 - GND                  |
| 4 - SWDIO - SWD data input/output    | 19 - PA13                 |
| 5 - NRST - RESET of target MCU       | 4 - NRST                  |
| 6 - SWO - Reserved                   | NC                        |


