use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    if env::var_os("CARGO_FEATURE_RT").is_some() {
        let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
        println!("cargo:rustc-link-search={}", out.display());

        let device_file = if env::var_os("CARGO_FEATURE_EFM32TG11B120").is_some() {
            "src/efm32tg11b120/device.x"
        } else if env::var_os("CARGO_FEATURE_EFM32TG11B140").is_some() {
            "src/efm32tg11b140/device.x"
        } else if env::var_os("CARGO_FEATURE_EFM32TG11B320").is_some() {
            "src/efm32tg11b320/device.x"
        } else if env::var_os("CARGO_FEATURE_EFM32TG11B340").is_some() {
            "src/efm32tg11b340/device.x"
        } else if env::var_os("CARGO_FEATURE_EFM32TG11B520").is_some() {
            "src/efm32tg11b520/device.x"
        } else if env::var_os("CARGO_FEATURE_EFM32TG11B540").is_some() {
            "src/efm32tg11b540/device.x"
        } else { panic!("No device features selected"); };

        fs::copy(device_file, out.join("device.x")).unwrap();
        println!("cargo:rerun-if-changed={}", device_file);
    }

    println!("cargo:rerun-if-changed=build.rs");
}

