//! Automatically rewritten from C to Rust
//! Source: drivers/hwmon/ltc2947-spi.c
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
// Analog Devices LTC2947 high precision power and energy monitor over SPI
//
// Copyright 2019 Analog Devices Inc.
//

    static const struct regmap_config ltc2947_regmap_config = {
    .reg_bits = 16,
    .val_bits = 8,
    .read_flag_mask = BIT(0),
    };
#[no_mangle]
unsafe extern "C" fn ltc2947_probe(spi: *mut spi_device) -> c_int {
    static int ltc2947_probe(struct spi_device *spi)
    {
    struct regmap *map;
    map = devm_regmap_init_spi(spi, &ltc2947_regmap_config);
    if (IS_ERR(map))
    return PTR_ERR(map);
    return ltc2947_core_probe(map, spi_get_device_id(spi).name);
    }
    static const struct spi_device_id ltc2947_id[] = {
    {"ltc2947", 0},
    {}
    };
    MODULE_DEVICE_TABLE(spi, ltc2947_id);
    static struct spi_driver ltc2947_driver = {
    .driver = {
    .name = "ltc2947",
    .of_match_table = ltc2947_of_match,
    .pm = pm_sleep_ptr(&ltc2947_pm_ops),
    },
    .probe = ltc2947_probe,
    .id_table = ltc2947_id,
    };
    module_spi_driver(ltc2947_driver);
    MODULE_AUTHOR("Nuno Sa <nuno.sa@analog.com>");
    MODULE_DESCRIPTION("LTC2947 SPI power and energy monitor driver");
    MODULE_LICENSE("GPL");
