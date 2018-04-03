# `cortex-m-quickstart`

> A template for building applications for ARM Cortex-M microcontrollers

# [Documentation](https://docs.rs/cortex-m-quickstart)

# License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)

- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.


## Instructions

0. Read [Rust your ARM microcontroller!](http://blog.japaric.io/quickstart/).
1. Build example with `xargo build --example blinky` or `USE_STM32F0x2=1 xargo build --example blinky-stm32f0x2`
2. Run `openocd -f board/stm32f0discovery.cfg`
3. In another terminal run `arm-none-eabi-gdb target/thumbv6m-none-eabi/debug/examples/blinky` or `arm-none-eabi-gdb target/thumbv6m-none-eabi/debug/examples/blinky-stm32f0x2`
4. Download SVD from http://www.st.com/en/microcontrollers/stm32f051r8.html
5. Make sure that SVD doesn't contain any `bitWidth` that equals to `0` and generate
Rust lib with `svd2rust -i STM32F0x1.svd | rustfmt | tee src/lib.rs`
