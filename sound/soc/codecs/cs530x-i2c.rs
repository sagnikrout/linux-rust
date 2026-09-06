//! Automatically rewritten from C to Rust
//! Source: sound/soc/codecs/cs530x-i2c.c
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
// CS530x CODEC driver
//
// Copyright (C) 2024-2025 Cirrus Logic, Inc. and
// Cirrus Logic International Semiconductor Ltd.

    static const struct of_device_id cs530x_of_match[] = {
    {
    .compatible = "cirrus,cs4282",
    .data = (void *)CS4282,
    }, {
    .compatible = "cirrus,cs4302",
    .data = (void *)CS4302,
    }, {
    .compatible = "cirrus,cs4304",
    .data = (void *)CS4304,
    }, {
    .compatible = "cirrus,cs4308",
    .data = (void *)CS4308,
    }, {
    .compatible = "cirrus,cs5302",
    .data = (void *)CS5302,
    }, {
    .compatible = "cirrus,cs5304",
    .data = (void *)CS5304,
    }, {
    .compatible = "cirrus,cs5308",
    .data = (void *)CS5308,
    },
    {}
    };
    MODULE_DEVICE_TABLE(of, cs530x_of_match);
    static const struct i2c_device_id cs530x_i2c_id[] = {
    { .name = "cs4282", .driver_data = CS4282 },
    { .name = "cs4302", .driver_data = CS4302 },
    { .name = "cs4304", .driver_data = CS4304 },
    { .name = "cs4308", .driver_data = CS4308 },
    { .name = "cs5302", .driver_data = CS5302 },
    { .name = "cs5304", .driver_data = CS5304 },
    { .name = "cs5308", .driver_data = CS5308 },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, cs530x_i2c_id);
#[no_mangle]
unsafe extern "C" fn cs530x_i2c_probe(client: *mut i2c_client) -> c_int {
    static int cs530x_i2c_probe(struct i2c_client *client)
    {
    struct cs530x_priv *cs530x;
    cs530x = devm_kzalloc(&client.dev, sizeof(*cs530x), GFP_KERNEL);
    if (!cs530x)
    return -ENOMEM;
    i2c_set_clientdata(client, cs530x);
    cs530x.regmap = devm_regmap_init_i2c(client, &cs530x_regmap_i2c);
    if (IS_ERR(cs530x.regmap))
    return dev_err_probe(&client.dev, PTR_ERR(cs530x.regmap),
    "Failed to allocate register map\n");
    cs530x.devtype = (uintptr_t)i2c_get_match_data(client);
    cs530x.dev = &client.dev;
    return cs530x_probe(cs530x);
    }
    static struct i2c_driver cs530x_i2c_driver = {
    .driver = {
    .name = "cs530x",
    .of_match_table = cs530x_of_match,
    },
    .probe = cs530x_i2c_probe,
    .id_table = cs530x_i2c_id,
    };
    module_i2c_driver(cs530x_i2c_driver);
    MODULE_DESCRIPTION("I2C CS530X driver");
    MODULE_IMPORT_NS("SND_SOC_CS530X");
    MODULE_AUTHOR("Paul Handrigan, Cirrus Logic Inc, <paulha@opensource.cirrus.com>");
    MODULE_LICENSE("GPL");
