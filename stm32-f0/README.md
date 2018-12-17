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
