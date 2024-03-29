# EFM32HG
    
[![crates.io](https://img.shields.io/crates/v/efm32hg-pac?label=efm32hg)](https://crates.io/crates/efm32hg-pac)

This crate provides an autogenerated API for access to EFM32HG peripherals.

## Usage

Each device supported by this crate is behind a feature gate so that you only
compile the device(s) you want. To use, in your Cargo.toml:

```toml
[dependencies.efm32hg-pac]
version = "0.1.4"
features = ["efm32hg108"]
```

The `rt` feature is enabled by default and brings in support for `cortex-m-rt`.
To disable, specify `default-features = false` in `Cargo.toml`.

For full details on the autogenerated API, please see `svd2rust` Peripheral API [here].

[here]: https://docs.rs/svd2rust/0.28.0/svd2rust/#peripheral-api

## Supported Devices
| Feature | Devices |
|:-----:|:-------:|
|`efm32hg108`|EFM32HG108F32, EFM32HG108F64|
|`efm32hg110`|EFM32HG110F32, EFM32HG110F64|
|`efm32hg210`|EFM32HG210F32, EFM32HG210F64|
|`efm32hg222`|EFM32HG222F32, EFM32HG222F64|
|`efm32hg308`|EFM32HG308F32, EFM32HG308F64|
|`efm32hg309`|EFM32HG309F32, EFM32HG309F64|
|`efm32hg310`|EFM32HG310F32, EFM32HG310F64|
|`efm32hg321`|EFM32HG321F32, EFM32HG321F64|
|`efm32hg322`|EFM32HG322F32, EFM32HG322F64|
|`efm32hg350`|EFM32HG350F32, EFM32HG350F64|
