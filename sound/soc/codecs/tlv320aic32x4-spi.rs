//! Automatically rewritten from C to Rust
//! Source: sound/soc/codecs/tlv320aic32x4-spi.c
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


// SPDX-License-Identifier: GPL-2.0
//
// Copyright 2011-2019 NW Digital Radio
//
// Author: Annaliese McDermond <nh6z@nh6z.net>
//
// Based on sound/soc/codecs/wm8974 and TI driver for kernel 2.6.27.
//

    static const struct regmap_config aic32x4_spi_regmap_config = {
    .reg_bits = 7,
    .pad_bits = 1,
    .val_bits = 8,
    .read_flag_mask = 0x01,
    .max_register = AIC32X4_REFPOWERUP,
    .ranges = aic32x4_regmap_pages,
    .num_ranges = 1,
    };
#[no_mangle]
unsafe extern "C" fn aic32x4_spi_probe(spi: *mut spi_device) -> c_int {
    static int aic32x4_spi_probe(struct spi_device *spi)
    {
    struct regmap *regmap;
    enum aic32x4_type type;
    regmap = devm_regmap_init_spi(spi, &aic32x4_spi_regmap_config);
    if (IS_ERR(regmap))
    return PTR_ERR(regmap);
    type = (uintptr_t)spi_get_device_match_data(spi);
    return aic32x4_probe(&spi.dev, regmap, type);
    }
#[no_mangle]
unsafe extern "C" fn aic32x4_spi_remove(spi: *mut spi_device) {
    static void aic32x4_spi_remove(struct spi_device *spi)
    {
    aic32x4_remove(&spi.dev);
    }
    static const struct spi_device_id aic32x4_spi_id[] = {
    { "tlv320aic32x4", (kernel_ulong_t)AIC32X4_TYPE_AIC32X4 },
    { "tlv320aic32x6", (kernel_ulong_t)AIC32X4_TYPE_AIC32X6 },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(spi, aic32x4_spi_id);
    static const struct of_device_id aic32x4_of_id[] = {
    { .compatible = "ti,tlv320aic32x4", .data = (void *)AIC32X4_TYPE_AIC32X4 },
    { .compatible = "ti,tlv320aic32x6", .data = (void *)AIC32X4_TYPE_AIC32X6 },
    { /* senitel */ }
    };
    MODULE_DEVICE_TABLE(of, aic32x4_of_id);
    static struct spi_driver aic32x4_spi_driver = {
    .driver = {
    .name = "tlv320aic32x4",
    .of_match_table = aic32x4_of_id,
    },
    .probe =    aic32x4_spi_probe,
    .remove =   aic32x4_spi_remove,
    .id_table = aic32x4_spi_id,
    };
    module_spi_driver(aic32x4_spi_driver);
    MODULE_DESCRIPTION("ASoC TLV320AIC32x4 codec driver SPI");
    MODULE_AUTHOR("Annaliese McDermond <nh6z@nh6z.net>");
    MODULE_LICENSE("GPL");
