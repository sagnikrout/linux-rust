//! Automatically rewritten from C to Rust
//! Source: drivers/mfd/cs40l50-i2c.c
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
// CS40L50 Advanced Haptic Driver with waveform memory,
// integrated DSP, and closed-loop algorithms
//
// Copyright 2024 Cirrus Logic, Inc.
//
// Author: James Ogletree <james.ogletree@cirrus.com>
//

#[no_mangle]
unsafe extern "C" fn cs40l50_i2c_probe(i2c: *mut i2c_client) -> c_int {
    static int cs40l50_i2c_probe(struct i2c_client *i2c)
    {
    struct cs40l50 *cs40l50;
    cs40l50 = devm_kzalloc(&i2c.dev, sizeof(*cs40l50), GFP_KERNEL);
    if (!cs40l50)
    return -ENOMEM;
    i2c_set_clientdata(i2c, cs40l50);
    cs40l50.dev = &i2c.dev;
    cs40l50.irq = i2c.irq;
    cs40l50.regmap = devm_regmap_init_i2c(i2c, &cs40l50_regmap);
    if (IS_ERR(cs40l50.regmap))
    return dev_err_probe(cs40l50.dev, PTR_ERR(cs40l50.regmap),
    "Failed to initialize register map\n");
    return cs40l50_probe(cs40l50);
    }
#[no_mangle]
unsafe extern "C" fn cs40l50_i2c_remove(i2c: *mut i2c_client) {
    static void cs40l50_i2c_remove(struct i2c_client *i2c)
    {
    struct cs40l50 *cs40l50 = i2c_get_clientdata(i2c);
    cs40l50_remove(cs40l50);
    }
    static const struct i2c_device_id cs40l50_id_i2c[] = {
    { "cs40l50" },
    {}
    };
    MODULE_DEVICE_TABLE(i2c, cs40l50_id_i2c);
    static const struct of_device_id cs40l50_of_match[] = {
    { .compatible = "cirrus,cs40l50" },
    {}
    };
    MODULE_DEVICE_TABLE(of, cs40l50_of_match);
    static struct i2c_driver cs40l50_i2c_driver = {
    .driver = {
    .name = "cs40l50",
    .of_match_table = cs40l50_of_match,
    .pm = pm_ptr(&cs40l50_pm_ops),
    },
    .id_table = cs40l50_id_i2c,
    .probe = cs40l50_i2c_probe,
    .remove = cs40l50_i2c_remove,
    };
    module_i2c_driver(cs40l50_i2c_driver);
    MODULE_DESCRIPTION("CS40L50 I2C Driver");
    MODULE_AUTHOR("James Ogletree, Cirrus Logic Inc. <james.ogletree@cirrus.com>");
    MODULE_LICENSE("GPL");
