use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    if env::var_os("CARGO_FEATURE_RT").is_some() {
        let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
        println!("cargo:rustc-link-search={}", out.display());

        let device_file = if env::var_os("CARGO_FEATURE_EFM32TG108").is_some() {
            "src/efm32tg108/device.x"
        } else if env::var_os("CARGO_FEATURE_EFM32TG110").is_some() {
            "src/efm32tg110/device.x"
        } else if env::var_os("CARGO_FEATURE_EFM32TG210").is_some() {
            "src/efm32tg210/device.x"
        } else if env::var_os("CARGO_FEATURE_EFM32TG222").is_some() {
            "src/efm32tg222/device.x"
        } else if env::var_os("CARGO_FEATURE_EFM32TG225").is_some() {
            "src/efm32tg225/device.x"
        } else if env::var_os("CARGO_FEATURE_EFM32TG230").is_some() {
            "src/efm32tg230/device.x"
        } else if env::var_os("CARGO_FEATURE_EFM32TG232").is_some() {
            "src/efm32tg232/device.x"
        } else if env::var_os("CARGO_FEATURE_EFM32TG822").is_some() {
            "src/efm32tg822/device.x"
        } else if env::var_os("CARGO_FEATURE_EFM32TG825").is_some() {
            "src/efm32tg825/device.x"
        } else if env::var_os("CARGO_FEATURE_EFM32TG840").is_some() {
            "src/efm32tg840/device.x"
        } else if env::var_os("CARGO_FEATURE_EFM32TG842").is_some() {
            "src/efm32tg842/device.x"
        } else { panic!("No device features selected"); };

        fs::copy(device_file, out.join("device.x")).unwrap();
        println!("cargo:rerun-if-changed={}", device_file);
    }

    println!("cargo:rerun-if-changed=build.rs");
}

