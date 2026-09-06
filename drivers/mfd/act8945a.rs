//! Automatically rewritten from C to Rust
//! Source: drivers/mfd/act8945a.c
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
// MFD driver for Active-semi ACT8945a PMIC
//
// Copyright (C) 2015 Atmel Corporation.
//
// Author: Wenyou Yang <wenyou.yang@atmel.com>
//

    static const struct mfd_cell act8945a_devs[] = {
    {
    .name = "act8945a-regulator",
    },
    {
    .name = "act8945a-charger",
    .of_compatible = "active-semi,act8945a-charger",
    },
    };
    static const struct regmap_config act8945a_regmap_config = {
    .reg_bits = 8,
    .val_bits = 8,
    };
#[no_mangle]
unsafe extern "C" fn act8945a_i2c_probe(i2c: *mut i2c_client) -> c_int {
    static int act8945a_i2c_probe(struct i2c_client *i2c)
    {
    int ret;
    struct regmap *regmap;
    regmap = devm_regmap_init_i2c(i2c, &act8945a_regmap_config);
    if (IS_ERR(regmap)) {
    ret = PTR_ERR(regmap);
    dev_err(&i2c.dev, "regmap init failed: %d\n", ret);
    return ret;
    }
    i2c_set_clientdata(i2c, regmap);
    ret = devm_mfd_add_devices(&i2c.dev, PLATFORM_DEVID_NONE,
    act8945a_devs, ARRAY_SIZE(act8945a_devs),
    core::ptr::null_mut(), 0, core::ptr::null_mut());
    if (ret) {
    dev_err(&i2c.dev, "Failed to add sub devices\n");
    return ret;
    }
    return 0;
    }
    static const struct i2c_device_id act8945a_i2c_id[] = {
    { "act8945a" },
    {}
    };
    MODULE_DEVICE_TABLE(i2c, act8945a_i2c_id);
    static const struct of_device_id act8945a_of_match[] = {
    { .compatible = "active-semi,act8945a", },
    {},
    };
    MODULE_DEVICE_TABLE(of, act8945a_of_match);
    static struct i2c_driver act8945a_i2c_driver = {
    .driver = {
    .name = "act8945a",
    .of_match_table = act8945a_of_match,
    },
    .probe = act8945a_i2c_probe,
    .id_table = act8945a_i2c_id,
    };
#[no_mangle]
unsafe extern "C" fn act8945a_i2c_init() -> int __init {
    static int __init act8945a_i2c_init(void)
    {
    return i2c_add_driver(&act8945a_i2c_driver);
    }
    subsys_initcall(act8945a_i2c_init);
#[no_mangle]
unsafe extern "C" fn act8945a_i2c_exit() -> void __exit {
    static void __exit act8945a_i2c_exit(void)
    {
    i2c_del_driver(&act8945a_i2c_driver);
    }
    module_exit(act8945a_i2c_exit);
    MODULE_DESCRIPTION("ACT8945A PMIC multi-function driver");
    MODULE_AUTHOR("Wenyou Yang <wenyou.yang@atmel.com>");
    MODULE_LICENSE("GPL");
