# EFM32TG (Tiny Gecko / Tiny Gecko S1) support for Rust

[![PACs](https://github.com/efm32-rs/efm32tg-pacs/actions/workflows/pacs.yml/badge.svg)](https://github.com/efm32-rs/efm32tg-pacs/actions/workflows/pacs.yml)

This repository contains Peripheral Access Crates (PACs) for Silabs' EFM32TG series of Cortex-M microcontrollers.
All these crates are automatically generated using [svd2rust](https://github.com/rust-embedded/svd2rust).

Refer to the [CHANGELOG](CHANGELOG.md) to see what changed in the last releases.

## Crates

Every EFM32G chip has its own PAC, listed below:

| Crate           | Docs                                                                                 | crates.io                                                                                                 | Target               |
|-----------------|--------------------------------------------------------------------------------------|-----------------------------------------------------------------------------------------------------------|----------------------|
| `efm32tg-pac`| [![docs.rs](https://docs.rs/efm32tg-pac/badge.svg)](https://docs.rs/efm32tg-pac)| [![crates.io](https://img.shields.io/crates/d/efm32tg-pac)](https://crates.io/crates/efm32tg-pac)| `thumbv7m-none-eabi` |
| `efm32tg11b-pac`| [![docs.rs](https://docs.rs/efm32tg11b-pac/badge.svg)](https://docs.rs/efm32tg11b-pac)| [![crates.io](https://img.shields.io/crates/d/efm32tg11b-pac)](https://crates.io/crates/efm32tg11b-pac)| `thumbv6m-none-eabi` |

## Device Reference Manuals from Silabs

**WIP**

## License

The included SVD files are sourced from https://www.silabs.com/documents/public/cmsis-packs and
are licensed under the Zlib (see [LICENSE-3RD-PARTY](LICENSE-3RD-PARTY-Zlib)).

The remainder of the code is under:

- 3-Clause BSD license ([LICENSE-3BSD](LICENSE-3BSD) or https://opensource.org/licenses/BSD-3-Clause)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the
work by you, as defined in the BSD-3-Clause license without any additional terms or conditions.
