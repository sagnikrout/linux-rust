//! Automatically rewritten from C to Rust
//! Source: drivers/iio/magnetometer/hmc5843_spi.c
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
// SPI driver for hmc5983
//
// Copyright (C) Josef Gajdusek <atx@atx.name>
//

    static const struct regmap_range hmc5843_readable_ranges[] = {
    regmap_reg_range(0, HMC5843_ID_END),
    };
    static const struct regmap_access_table hmc5843_readable_table = {
    .yes_ranges = hmc5843_readable_ranges,
    .n_yes_ranges = ARRAY_SIZE(hmc5843_readable_ranges),
    };
    static const struct regmap_range hmc5843_writable_ranges[] = {
    regmap_reg_range(0, HMC5843_MODE_REG),
    };
    static const struct regmap_access_table hmc5843_writable_table = {
    .yes_ranges = hmc5843_writable_ranges,
    .n_yes_ranges = ARRAY_SIZE(hmc5843_writable_ranges),
    };
    static const struct regmap_range hmc5843_volatile_ranges[] = {
    regmap_reg_range(HMC5843_DATA_OUT_MSB_REGS, HMC5843_STATUS_REG),
    };
    static const struct regmap_access_table hmc5843_volatile_table = {
    .yes_ranges = hmc5843_volatile_ranges,
    .n_yes_ranges = ARRAY_SIZE(hmc5843_volatile_ranges),
    };
    static const struct regmap_config hmc5843_spi_regmap_config = {
    .reg_bits = 8,
    .val_bits = 8,
    .rd_table = &hmc5843_readable_table,
    .wr_table = &hmc5843_writable_table,
    .volatile_table = &hmc5843_volatile_table,
// Autoincrement address pointer
    .read_flag_mask = 0xc0,
    .cache_type = REGCACHE_RBTREE,
    };
#[no_mangle]
unsafe extern "C" fn hmc5843_spi_probe(spi: *mut spi_device) -> c_int {
    static int hmc5843_spi_probe(struct spi_device *spi)
    {
    int ret;
    struct regmap *regmap;
    spi.mode = SPI_MODE_3;
    spi.max_speed_hz = 8000000;
    ret = spi_setup(spi);
    if (ret)
    return ret;
    regmap = devm_regmap_init_spi(spi, &hmc5843_spi_regmap_config);
    if (IS_ERR(regmap))
    return PTR_ERR(regmap);
    return hmc5843_common_probe(&spi.dev,
    regmap, HMC5983_ID, "hmc5983");
    }
#[no_mangle]
unsafe extern "C" fn hmc5843_spi_remove(spi: *mut spi_device) {
    static void hmc5843_spi_remove(struct spi_device *spi)
    {
    hmc5843_common_remove(&spi.dev);
    }
    static const struct spi_device_id hmc5843_id[] = {
    { .name = "hmc5983" },
    { }
    };
    MODULE_DEVICE_TABLE(spi, hmc5843_id);
    static struct spi_driver hmc5843_driver = {
    .driver = {
    .name = "hmc5843",
    .pm = pm_sleep_ptr(&hmc5843_pm_ops),
    },
    .id_table = hmc5843_id,
    .probe = hmc5843_spi_probe,
    .remove = hmc5843_spi_remove,
    };
    module_spi_driver(hmc5843_driver);
    MODULE_AUTHOR("Josef Gajdusek <atx@atx.name>");
    MODULE_DESCRIPTION("HMC5983 SPI driver");
    MODULE_LICENSE("GPL");
    MODULE_IMPORT_NS("IIO_HMC5843");
