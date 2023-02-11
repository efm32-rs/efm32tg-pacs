//! Peripheral access API for EFM32TG microcontrollers
//! (generated using [svd2rust](https://github.com/rust-embedded/svd2rust)
//! 0.28.0)
//!
//! You can find an overview of the API here:
//! [svd2rust/#peripheral-api](https://docs.rs/svd2rust/0.28.0/svd2rust/#peripheral-api)
//!
//! For more details see the README here:
//! [efm32-rs](https://github.com/efm32-rs/efm32tg-pacs)
//!
//! This crate supports all EFM32TG devices; for the complete list please see:
//! [efm32tg](https://github.com/efm32-rs/efm32tg-pacs/pacs/efm32tg)

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]

mod generic;
pub use self::generic::*;

#[cfg(feature = "efm32tg108")]
pub mod efm32tg108;

#[cfg(feature = "efm32tg110")]
pub mod efm32tg110;

#[cfg(feature = "efm32tg210")]
pub mod efm32tg210;

#[cfg(feature = "efm32tg222")]
pub mod efm32tg222;

#[cfg(feature = "efm32tg225")]
pub mod efm32tg225;

#[cfg(feature = "efm32tg230")]
pub mod efm32tg230;

#[cfg(feature = "efm32tg232")]
pub mod efm32tg232;

#[cfg(feature = "efm32tg822")]
pub mod efm32tg822;

#[cfg(feature = "efm32tg825")]
pub mod efm32tg825;

#[cfg(feature = "efm32tg840")]
pub mod efm32tg840;

#[cfg(feature = "efm32tg842")]
pub mod efm32tg842;
