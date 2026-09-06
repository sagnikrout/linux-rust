//! Automatically rewritten from C to Rust
//! Source: drivers/iio/chemical/ens160_i2c.c
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
// ScioSense ENS160 multi-gas sensor I2C driver
//
// Copyright (c) 2024 Gustavo Silva <gustavograzs@gmail.com>
//
// 7-Bit I2C slave address is:
// - 0x52 if ADDR pin LOW
// - 0x53 if ADDR pin HIGH
//

    static const struct regmap_config ens160_regmap_i2c_conf = {
    .reg_bits = 8,
    .val_bits = 8,
    };
#[no_mangle]
unsafe extern "C" fn ens160_i2c_probe(client: *mut i2c_client) -> c_int {
    static int ens160_i2c_probe(struct i2c_client *client)
    {
    struct regmap *regmap;
    regmap = devm_regmap_init_i2c(client, &ens160_regmap_i2c_conf);
    if (IS_ERR(regmap))
    return dev_err_probe(&client.dev, PTR_ERR(regmap),
    "Failed to register i2c regmap\n");
    return devm_ens160_core_probe(&client.dev, regmap, client.irq,
    "ens160");
    }
    static const struct i2c_device_id ens160_i2c_id[] = {
    { .name = "ens160" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, ens160_i2c_id);
    static const struct of_device_id ens160_of_i2c_match[] = {
    { .compatible = "sciosense,ens160" },
    { }
    };
    MODULE_DEVICE_TABLE(of, ens160_of_i2c_match);
    static struct i2c_driver ens160_i2c_driver = {
    .driver = {
    .name		= "ens160",
    .of_match_table	= ens160_of_i2c_match,
    .pm		= pm_sleep_ptr(&ens160_pm_ops),
    },
    .probe = ens160_i2c_probe,
    .id_table = ens160_i2c_id,
    };
    module_i2c_driver(ens160_i2c_driver);
    MODULE_AUTHOR("Gustavo Silva <gustavograzs@gmail.com>");
    MODULE_DESCRIPTION("ScioSense ENS160 I2C driver");
    MODULE_LICENSE("GPL v2");
    MODULE_IMPORT_NS("IIO_ENS160");
