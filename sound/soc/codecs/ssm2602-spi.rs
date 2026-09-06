//! Automatically rewritten from C to Rust
//! Source: sound/soc/codecs/ssm2602-spi.c
#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use core::ffi::*;

// --- Linux Kernel Primitives Prelude ---
pub type uid_t = u32;
pub type gid_t = u32;
pub type uid16_t = u16;
pub type gid16_t = u16;
pub type pid_t = i32;
pub type mode_t = u32;
pub type umode_t = u16;
pub type nlink_t = u32;
pub type off_t = i64;
pub type loff_t = i64;
pub type dev_t = u32;
pub type ino_t = u64;
pub type size_t = usize;
pub type ssize_t = isize;
pub type uintptr_t = usize;
pub type intptr_t = isize;
pub type ptrdiff_t = isize;
pub type clockid_t = i32;
pub type timer_t = i32;
pub type time64_t = i64;
pub type atomic_t = core::sync::atomic::AtomicI32;
pub type atomic64_t = core::sync::atomic::AtomicI64;
// ---------------------------------------


// SPDX-License-Identifier: GPL-2.0-only
//
// SSM2602 SPI audio driver
//
// Copyright 2014 Analog Devices Inc.
//

#[no_mangle]
unsafe extern "C" fn ssm2602_spi_probe(spi: *mut spi_device) -> c_int {
    static int ssm2602_spi_probe(struct spi_device *spi)
    {
    return ssm2602_probe(&spi.dev, SSM2602,
    devm_regmap_init_spi(spi, &ssm2602_regmap_config));
    }
    static const struct of_device_id ssm2602_of_match[] = {
    { .compatible = "adi,ssm2602", },
    { }
    };
    MODULE_DEVICE_TABLE(of, ssm2602_of_match);
    static struct spi_driver ssm2602_spi_driver = {
    .driver = {
    .name	= "ssm2602",
    .of_match_table = ssm2602_of_match,
    },
    .probe		= ssm2602_spi_probe,
    };
    module_spi_driver(ssm2602_spi_driver);
    MODULE_DESCRIPTION("ASoC SSM2602 SPI driver");
    MODULE_AUTHOR("Cliff Cai");
    MODULE_LICENSE("GPL");
