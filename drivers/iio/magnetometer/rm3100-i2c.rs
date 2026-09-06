//! Automatically rewritten from C to Rust
//! Source: drivers/iio/magnetometer/rm3100-i2c.c
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
// Support for PNI RM3100 3-axis geomagnetic sensor on a i2c bus.
//
// Copyright (C) 2018 Song Qiang <songqiang1304521@gmail.com>
//
// i2c slave address: 0x20 + SA1 << 1 + SA0.
//

    static const struct regmap_config rm3100_regmap_config = {
    .reg_bits = 8,
    .val_bits = 8,
    .rd_table = &rm3100_readable_table,
    .wr_table = &rm3100_writable_table,
    .volatile_table = &rm3100_volatile_table,
    .cache_type = REGCACHE_RBTREE,
    };
#[no_mangle]
unsafe extern "C" fn rm3100_probe(client: *mut i2c_client) -> c_int {
    static int rm3100_probe(struct i2c_client *client)
    {
    struct regmap *regmap;
    regmap = devm_regmap_init_i2c(client, &rm3100_regmap_config);
    if (IS_ERR(regmap))
    return PTR_ERR(regmap);
    return rm3100_common_probe(&client.dev, regmap, client.irq);
    }
    static const struct of_device_id rm3100_dt_match[] = {
    { .compatible = "pni,rm3100", },
    { }
    };
    MODULE_DEVICE_TABLE(of, rm3100_dt_match);
    static struct i2c_driver rm3100_driver = {
    .driver = {
    .name = "rm3100-i2c",
    .of_match_table = rm3100_dt_match,
    },
    .probe = rm3100_probe,
    };
    module_i2c_driver(rm3100_driver);
    MODULE_AUTHOR("Song Qiang <songqiang1304521@gmail.com>");
    MODULE_DESCRIPTION("PNI RM3100 3-axis magnetometer i2c driver");
    MODULE_LICENSE("GPL v2");
    MODULE_IMPORT_NS("IIO_RM3100");
