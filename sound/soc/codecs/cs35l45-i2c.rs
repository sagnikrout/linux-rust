//! Automatically rewritten from C to Rust
//! Source: sound/soc/codecs/cs35l45-i2c.c
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
// cs35l45-i2c.c -- CS35L45 I2C driver
//
// Copyright 2019-2022 Cirrus Logic, Inc.
//
// Author: James Schulman <james.schulman@cirrus.com>

#[no_mangle]
unsafe extern "C" fn cs35l45_i2c_probe(client: *mut i2c_client) -> c_int {
    static int cs35l45_i2c_probe(struct i2c_client *client)
    {
    struct cs35l45_private *cs35l45;
    struct device *dev = &client.dev;
    int ret;
    cs35l45 = devm_kzalloc(dev, sizeof(struct cs35l45_private), GFP_KERNEL);
    if (!cs35l45)
    return -ENOMEM;
    i2c_set_clientdata(client, cs35l45);
    cs35l45.regmap = devm_regmap_init_i2c(client, &cs35l45_i2c_regmap);
    if (IS_ERR(cs35l45.regmap)) {
    ret = PTR_ERR(cs35l45.regmap);
    dev_err(dev, "Failed to allocate register map: %d\n", ret);
    return ret;
    }
    cs35l45.dev = dev;
    cs35l45.irq = client.irq;
    cs35l45.bus_type = CONTROL_BUS_I2C;
    cs35l45.i2c_addr = client.addr;
    return cs35l45_probe(cs35l45);
    }
#[no_mangle]
unsafe extern "C" fn cs35l45_i2c_remove(client: *mut i2c_client) {
    static void cs35l45_i2c_remove(struct i2c_client *client)
    {
    struct cs35l45_private *cs35l45 = i2c_get_clientdata(client);
    cs35l45_remove(cs35l45);
    }
    static const struct of_device_id cs35l45_of_match[] = {
    { .compatible = "cirrus,cs35l45" },
    {},
    };
    MODULE_DEVICE_TABLE(of, cs35l45_of_match);
    static const struct i2c_device_id cs35l45_id_i2c[] = {
    { .name = "cs35l45" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, cs35l45_id_i2c);
    static struct i2c_driver cs35l45_i2c_driver = {
    .driver = {
    .name		= "cs35l45",
    .of_match_table = cs35l45_of_match,
    .pm		= pm_ptr(&cs35l45_pm_ops),
    },
    .id_table	= cs35l45_id_i2c,
    .probe		= cs35l45_i2c_probe,
    .remove		= cs35l45_i2c_remove,
    };
    module_i2c_driver(cs35l45_i2c_driver);
    MODULE_DESCRIPTION("I2C CS35L45 driver");
    MODULE_AUTHOR("James Schulman, Cirrus Logic Inc, <james.schulman@cirrus.com>");
    MODULE_LICENSE("GPL");
    MODULE_IMPORT_NS("SND_SOC_CS35L45");
