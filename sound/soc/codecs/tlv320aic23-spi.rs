//! Automatically rewritten from C to Rust
//! Source: sound/soc/codecs/tlv320aic23-spi.c
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
// ALSA SoC TLV320AIC23 codec driver SPI interface
//
// Author:      Arun KS, <arunks@mistralsolutions.com>
// Copyright:   (C) 2008 Mistral Solutions Pvt Ltd.,
//
// Based on sound/soc/codecs/wm8731.c by Richard Purdie
//

#[no_mangle]
unsafe extern "C" fn aic23_spi_probe(spi: *mut spi_device) -> c_int {
    static int aic23_spi_probe(struct spi_device *spi)
    {
    int ret;
    struct regmap *regmap;
    dev_dbg(&spi.dev, "probing tlv320aic23 spi device\n");
    spi.mode = SPI_MODE_0;
    ret = spi_setup(spi);
    if (ret < 0)
    return ret;
    regmap = devm_regmap_init_spi(spi, &tlv320aic23_regmap);
    return tlv320aic23_probe(&spi.dev, regmap);
    }
    static struct spi_driver aic23_spi = {
    .driver = {
    .name = "tlv320aic23",
    },
    .probe = aic23_spi_probe,
    };
    module_spi_driver(aic23_spi);
    MODULE_DESCRIPTION("ASoC TLV320AIC23 codec driver SPI");
    MODULE_AUTHOR("Arun KS <arunks@mistralsolutions.com>");
    MODULE_LICENSE("GPL");
