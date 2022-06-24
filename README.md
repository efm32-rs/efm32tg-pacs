# EFM32TG (Tiny Gecko / Tiny Gecko S1) support for Rust

[![PACs](https://github.com/efm32-rs/efm32tg-pacs/actions/workflows/pacs.yml/badge.svg)](https://github.com/efm32-rs/efm32tg-pacs/actions/workflows/pacs.yml)

This repository contains Peripheral Access Crates (PACs) for Silabs' EFM32 series of Cortex-M microcontrollers.

All these crates are automatically generated using [svd2rust](https://github.com/rust-embedded/svd2rust).

Refer to the [CHANGELOG](CHANGELOG.md) to see what changed in the last releases.

## Crates

Every EFM32TG chip has its own PAC, listed below:

| Crate               | Docs                                                                                         | crates.io                                                                                                         | target               |
|---------------------|----------------------------------------------------------------------------------------------|-------------------------------------------------------------------------------------------------------------------|----------------------|
| `efm32tg108-pac`    | [![docs.rs](https://docs.rs/efm32tg108-pac/badge.svg)](https://docs.rs/efm32tg108-pac)       | [![crates.io](https://img.shields.io/crates/d/efm32tg108-pac.svg)](https://crates.io/crates/efm32tg108-pac)       | `thumbv7m-none-eabi` |
| `efm32tg110-pac`    | [![docs.rs](https://docs.rs/efm32tg110-pac/badge.svg)](https://docs.rs/efm32tg110-pac)       | [![crates.io](https://img.shields.io/crates/d/efm32tg110-pac.svg)](https://crates.io/crates/efm32tg110-pac)       | `thumbv7m-none-eabi` |
| `efm32tg210-pac`    | [![docs.rs](https://docs.rs/efm32tg210-pac/badge.svg)](https://docs.rs/efm32tg210-pac)       | [![crates.io](https://img.shields.io/crates/d/efm32tg210-pac.svg)](https://crates.io/crates/efm32tg210-pac)       | `thumbv7m-none-eabi` |
| `efm32tg222-pac`    | [![docs.rs](https://docs.rs/efm32tg222-pac/badge.svg)](https://docs.rs/efm32tg222-pac)       | [![crates.io](https://img.shields.io/crates/d/efm32tg222-pac.svg)](https://crates.io/crates/efm32tg222-pac)       | `thumbv7m-none-eabi` |
| `efm32tg225-pac`    | [![docs.rs](https://docs.rs/efm32tg225-pac/badge.svg)](https://docs.rs/efm32tg225-pac)       | [![crates.io](https://img.shields.io/crates/d/efm32tg225-pac.svg)](https://crates.io/crates/efm32tg225-pac)       | `thumbv7m-none-eabi` |
| `efm32tg230-pac`    | [![docs.rs](https://docs.rs/efm32tg230-pac/badge.svg)](https://docs.rs/efm32tg230-pac)       | [![crates.io](https://img.shields.io/crates/d/efm32tg230-pac.svg)](https://crates.io/crates/efm32tg230-pac)       | `thumbv7m-none-eabi` |
| `efm32tg232-pac`    | [![docs.rs](https://docs.rs/efm32tg232-pac/badge.svg)](https://docs.rs/efm32tg232-pac)       | [![crates.io](https://img.shields.io/crates/d/efm32tg232-pac.svg)](https://crates.io/crates/efm32tg232-pac)       | `thumbv7m-none-eabi` |
| `efm32tg822-pac`    | [![docs.rs](https://docs.rs/efm32tg822-pac/badge.svg)](https://docs.rs/efm32tg822-pac)       | [![crates.io](https://img.shields.io/crates/d/efm32tg822-pac.svg)](https://crates.io/crates/efm32tg822-pac)       | `thumbv7m-none-eabi` |
| `efm32tg825-pac`    | [![docs.rs](https://docs.rs/efm32tg825-pac/badge.svg)](https://docs.rs/efm32tg825-pac)       | [![crates.io](https://img.shields.io/crates/d/efm32tg825-pac.svg)](https://crates.io/crates/efm32tg825-pac)       | `thumbv7m-none-eabi` |
| `efm32tg840-pac`    | [![docs.rs](https://docs.rs/efm32tg840-pac/badge.svg)](https://docs.rs/efm32tg840-pac)       | [![crates.io](https://img.shields.io/crates/d/efm32tg840-pac.svg)](https://crates.io/crates/efm32tg840-pac)       | `thumbv7m-none-eabi` |
| `efm32tg842-pac`    | [![docs.rs](https://docs.rs/efm32tg842-pac/badge.svg)](https://docs.rs/efm32tg842-pac)       | [![crates.io](https://img.shields.io/crates/d/efm32tg842-pac.svg)](https://crates.io/crates/efm32tg842-pac)       | `thumbv7m-none-eabi` |
| `efm32tg11b120-pac` | [![docs.rs](https://docs.rs/efm32tg11b120-pac/badge.svg)](https://docs.rs/efm32tg11b120-pac) | [![crates.io](https://img.shields.io/crates/d/efm32tg11b120-pac.svg)](https://crates.io/crates/efm32tg11b120-pac) | `thumbv6m-none-eabi` |
| `efm32tg11b140-pac` | [![docs.rs](https://docs.rs/efm32tg11b140-pac/badge.svg)](https://docs.rs/efm32tg11b140-pac) | [![crates.io](https://img.shields.io/crates/d/efm32tg11b140-pac.svg)](https://crates.io/crates/efm32tg11b140-pac) | `thumbv6m-none-eabi` |
| `efm32tg11b320-pac` | [![docs.rs](https://docs.rs/efm32tg11b320-pac/badge.svg)](https://docs.rs/efm32tg11b320-pac) | [![crates.io](https://img.shields.io/crates/d/efm32tg11b320-pac.svg)](https://crates.io/crates/efm32tg11b320-pac) | `thumbv6m-none-eabi` |
| `efm32tg11b340-pac` | [![docs.rs](https://docs.rs/efm32tg11b340-pac/badge.svg)](https://docs.rs/efm32tg11b340-pac) | [![crates.io](https://img.shields.io/crates/d/efm32tg11b340-pac.svg)](https://crates.io/crates/efm32tg11b340-pac) | `thumbv6m-none-eabi` |
| `efm32tg11b520-pac` | [![docs.rs](https://docs.rs/efm32tg11b520-pac/badge.svg)](https://docs.rs/efm32tg11b520-pac) | [![crates.io](https://img.shields.io/crates/d/efm32tg11b520-pac.svg)](https://crates.io/crates/efm32tg11b520-pac) | `thumbv6m-none-eabi` |
| `efm32tg11b540-pac` | [![docs.rs](https://docs.rs/efm32tg11b540-pac/badge.svg)](https://docs.rs/efm32tg11b540-pac) | [![crates.io](https://img.shields.io/crates/d/efm32tg11b540-pac.svg)](https://crates.io/crates/efm32tg11b540-pac) | `thumbv6m-none-eabi` |

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