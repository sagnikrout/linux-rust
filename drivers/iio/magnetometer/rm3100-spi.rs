//! Automatically rewritten from C to Rust
//! Source: drivers/iio/magnetometer/rm3100-spi.c
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
// Support for PNI RM3100 3-axis geomagnetic sensor on a spi bus.
//
// Copyright (C) 2018 Song Qiang <songqiang1304521@gmail.com>
//

    static const struct regmap_config rm3100_regmap_config = {
    .reg_bits = 8,
    .val_bits = 8,
    .rd_table = &rm3100_readable_table,
    .wr_table = &rm3100_writable_table,
    .volatile_table = &rm3100_volatile_table,
    .read_flag_mask = 0x80,
    .cache_type = REGCACHE_RBTREE,
    };
#[no_mangle]
unsafe extern "C" fn rm3100_probe(spi: *mut spi_device) -> c_int {
    static int rm3100_probe(struct spi_device *spi)
    {
    struct regmap *regmap;
    int ret;
// Actually this device supports both mode 0 and mode 3.
    spi.mode = SPI_MODE_0;
// Data rates cannot exceed 1Mbits.
    spi.max_speed_hz = 1000000;
    ret = spi_setup(spi);
    if (ret)
    return ret;
    regmap = devm_regmap_init_spi(spi, &rm3100_regmap_config);
    if (IS_ERR(regmap))
    return PTR_ERR(regmap);
    return rm3100_common_probe(&spi.dev, regmap, spi.irq);
    }
    static const struct of_device_id rm3100_dt_match[] = {
    { .compatible = "pni,rm3100", },
    { }
    };
    MODULE_DEVICE_TABLE(of, rm3100_dt_match);
    static struct spi_driver rm3100_driver = {
    .driver = {
    .name = "rm3100-spi",
    .of_match_table = rm3100_dt_match,
    },
    .probe = rm3100_probe,
    };
    module_spi_driver(rm3100_driver);
    MODULE_AUTHOR("Song Qiang <songqiang1304521@gmail.com>");
    MODULE_DESCRIPTION("PNI RM3100 3-axis magnetometer spi driver");
    MODULE_LICENSE("GPL v2");
    MODULE_IMPORT_NS("IIO_RM3100");
