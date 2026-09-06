//! Automatically rewritten from C to Rust
//! Source: drivers/input/touchscreen/ad7879-spi.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// AD7879/AD7889 touchscreen (SPI bus)
//
// Copyright (C) 2008-2010 Michael Hennerich, Analog Devices Inc.
//

pub const AD7879_DEVID: c_uint = 0x7A	/* AD7879/AD7889 */;
pub const MAX_SPI_FREQ_HZ: c_int = 5000000;
pub const AD7879_CMD_MAGIC: c_uint = 0xE0;

    static const struct regmap_config ad7879_spi_regmap_config = {
    .reg_bits = 16,
    .val_bits = 16,
    .max_register = 15,
    .read_flag_mask = AD7879_CMD_MAGIC | AD7879_CMD_READ,
    .write_flag_mask = AD7879_CMD_MAGIC,
    };
#[no_mangle]
unsafe extern "C" fn ad7879_spi_probe(spi: *mut spi_device) -> c_int {
    static int ad7879_spi_probe(struct spi_device *spi)
    {
    struct regmap *regmap;
// don't exceed max specified SPI CLK frequency
    if (spi.max_speed_hz > MAX_SPI_FREQ_HZ) {
    dev_err(&spi.dev, "SPI CLK %d Hz?\n", spi.max_speed_hz);
    return -EINVAL;
    }
    regmap = devm_regmap_init_spi(spi, &ad7879_spi_regmap_config);
    if (IS_ERR(regmap))
    return PTR_ERR(regmap);
    return ad7879_probe(&spi.dev, regmap, spi.irq, BUS_SPI, AD7879_DEVID);
    }

    static const struct of_device_id ad7879_spi_dt_ids[] = {
    { .compatible = "adi,ad7879", },
    { }
    };
    MODULE_DEVICE_TABLE(of, ad7879_spi_dt_ids);

    static struct spi_driver ad7879_spi_driver = {
    .driver = {
    .name		= "ad7879",
    .dev_groups	= ad7879_groups,
    .pm		= &ad7879_pm_ops,
    .of_match_table	= of_match_ptr(ad7879_spi_dt_ids),
    },
    .probe		= ad7879_spi_probe,
    };
    module_spi_driver(ad7879_spi_driver);
    MODULE_AUTHOR("Michael Hennerich <michael.hennerich@analog.com>");
    MODULE_DESCRIPTION("AD7879(-1) touchscreen SPI bus driver");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("spi:ad7879");
