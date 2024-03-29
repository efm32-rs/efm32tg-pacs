//! Peripheral access API for EFM32TG11B microcontrollers
//! (generated using [svd2rust](https://github.com/rust-embedded/svd2rust)
//! 0.28.0)
//!
//! You can find an overview of the API here:
//! [svd2rust/#peripheral-api](https://docs.rs/svd2rust/0.28.0/svd2rust/#peripheral-api)
//!
//! For more details see the README here:
//! [efm32-rs](https://github.com/efm32-rs/efm32tg-pacs)
//!
//! This crate supports all EFM32TG11B devices; for the complete list please see:
//! [efm32tg11b](https://github.com/efm32-rs/efm32tg-pacs/pacs/efm32tg11b)

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]

mod generic;
pub use self::generic::*;

#[cfg(feature = "efm32tg11b120")]
pub mod efm32tg11b120;

#[cfg(feature = "efm32tg11b140")]
pub mod efm32tg11b140;

#[cfg(feature = "efm32tg11b320")]
pub mod efm32tg11b320;

#[cfg(feature = "efm32tg11b340")]
pub mod efm32tg11b340;

#[cfg(feature = "efm32tg11b520")]
pub mod efm32tg11b520;

#[cfg(feature = "efm32tg11b540")]
pub mod efm32tg11b540;
