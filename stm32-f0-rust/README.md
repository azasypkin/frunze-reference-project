## Debugging

Firs of all read [The Embedded Rust Book/Hardware](https://rust-embedded.github.io/book/start/hardware.html)!

Will debug with the help of ST-Link included into `STM32F0DISCOVERY` evaluation board. To program and debug `stm32f042f4p6` 
with this board remove the 2 jumpers from CN2 (see [en.DM00050135.pdf](./docs/en.DM00050135.pdf), page 16):

| `STM32F0DISCOVERY` CN3/SWD connector | `STM32F042F4P6`           |
| ------------------------------------ | -------------------------:|
| 1 - VDD_TARGET - VDD from target MCU | 16 - VDD __and__ 5 - VDDA |
| 2 - SWCLK - SWD clock                | 20 - PA14                 |
| 3 - GND - Ground                     | 15 - GND                  |
| 4 - SWDIO - SWD data input/output    | 19 - PA13                 |
| 5 - NRST - RESET of target MCU       | 4 - NRST                  |
| 6 - SWO - Reserved                   | NC                        |

1. Add `udev` rule for the `STMicroelectronics ST-LINK/V2`:

```bash
$ sudo vim /etc/udev/rules.d/99-stlink.rules

-------
ATTRS{idVendor}=="0483", ATTRS{idProduct}=="3748", MODE="0666"
-------

$ sudo udevadm trigger
```

1. Build project with one of the following commands:
```bash
$ cargo build --features stm32f051
$ cargo build --features stm32f051 --target=thumbv6m-none-eabi
$ cargo build --features stm32f042
$ cargo build --features stm32f042 --target=thumbv6m-none-eabi
$ cargo build --features stm32f042 --target=thumbv6m-none-eabi --release
```
2. Run `openocd -f openocd.cfg`
3. In another terminal run `arm-none-eabi-gdb target/thumbv6m-none-eabi/release/frunze-reference-project`


## Notes

SVD file can be downloaded from http://www.st.com/en/microcontrollers/stm32f051r8.html. Make sure that SVD
does not contain any `bitWidth` that equals to `0` and generate Rust lib with `svd2rust`.

If binary is too large GDB may fail so try to use `--release` flag with `cargo build`.

To reload program on the MCU use `monitor reset halt`

RTC & Low Power modes: https://github.com/mattico/stm32f0-Discovery_Tools/blob/master/ST_Example_Projects/Projects/Peripheral_Examples/PWR_CurrentConsumption/stm32f0xx_lp_modes.c
