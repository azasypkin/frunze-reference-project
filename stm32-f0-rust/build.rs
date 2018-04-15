use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    let use_stm32f0x2 = env::var_os("USE_STM32F0x2").is_some();

    // Put the linker script somewhere the linker can find it
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let mut file = File::create(out.join("memory.x")).unwrap();

    println!(
        "Using {}",
        if use_stm32f0x2 {
            "memory_stm32f0x2.x"
        } else {
            "memory.x"
        }
    );

    if use_stm32f0x2 {
        file.write_all(include_bytes!("memory_stm32f0x2.x"))
            .unwrap();
    } else {
        file.write_all(include_bytes!("memory.x")).unwrap();
    }

    println!("cargo:rustc-link-search={}", out.display());

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=memory.x");
}
