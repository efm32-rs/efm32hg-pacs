//! Peripheral access API for EFM32HG microcontrollers
//! (generated using [svd2rust](https://github.com/rust-embedded/svd2rust)
//! 0.24.0)
//!
//! You can find an overview of the API here:
//! [svd2rust/#peripheral-api](https://docs.rs/svd2rust/0.24.0/svd2rust/#peripheral-api)
//!
//! For more details see the README here:
//! [efm32-rs](https://github.com/efm32-rs/efm32hg-pacs)
//!
//! This crate supports all EFM32HG devices; for the complete list please see:
//! [efm32hg](https://github.com/efm32-rs/efm32hg-pacs/pacs/efm32hg)

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]

mod generic;
pub use self::generic::*;

#[cfg(feature = "efm32hg108")]
pub mod efm32hg108;

#[cfg(feature = "efm32hg110")]
pub mod efm32hg110;

#[cfg(feature = "efm32hg210")]
pub mod efm32hg210;

#[cfg(feature = "efm32hg222")]
pub mod efm32hg222;

#[cfg(feature = "efm32hg308")]
pub mod efm32hg308;

#[cfg(feature = "efm32hg309")]
pub mod efm32hg309;

#[cfg(feature = "efm32hg310")]
pub mod efm32hg310;

#[cfg(feature = "efm32hg321")]
pub mod efm32hg321;

#[cfg(feature = "efm32hg322")]
pub mod efm32hg322;

#[cfg(feature = "efm32hg350")]
pub mod efm32hg350;
