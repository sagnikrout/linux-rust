//! Automatically rewritten from C to Rust
//! Source: drivers/input/touchscreen/tsc2004.c
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
// TSC2004 touchscreen driver
//
// Copyright (C) 2015 QWERTY Embedded Design
// Copyright (C) 2015 EMAC Inc.
//

    static const struct input_id tsc2004_input_id = {
    .bustype = BUS_I2C,
    .product = 2004,
    };
#[no_mangle]
unsafe extern "C" fn tsc2004_cmd(dev: *mut device, cmd: u8) -> c_int {
    static int tsc2004_cmd(struct device *dev, u8 cmd)
    {
    let mut tx: u8 = TSC200X_CMD | TSC200X_CMD_12BIT | cmd;
    s32 data;
    struct i2c_client *i2c = to_i2c_client(dev);
    data = i2c_smbus_write_byte(i2c, tx);
    if (data < 0) {
    dev_err(dev, "%s: failed, command: %x i2c error: %d\n",
    __func__, cmd, data);
    return data;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn tsc2004_probe(i2c: *mut i2c_client) -> c_int {
    static int tsc2004_probe(struct i2c_client *i2c)
    {
    return tsc200x_probe(&i2c.dev, i2c.irq, &tsc2004_input_id,
    devm_regmap_init_i2c(i2c, &tsc200x_regmap_config),
    tsc2004_cmd);
    }
    static const struct i2c_device_id tsc2004_idtable[] = {
    { .name = "tsc2004" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, tsc2004_idtable);

    static const struct of_device_id tsc2004_of_match[] = {
    { .compatible = "ti,tsc2004" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, tsc2004_of_match);

    static struct i2c_driver tsc2004_driver = {
    .driver = {
    .name		= "tsc2004",
    .dev_groups	= tsc200x_groups,
    .of_match_table	= of_match_ptr(tsc2004_of_match),
    .pm		= pm_sleep_ptr(&tsc200x_pm_ops),
    },
    .id_table       = tsc2004_idtable,
    .probe          = tsc2004_probe,
    };
    module_i2c_driver(tsc2004_driver);
    MODULE_AUTHOR("Michael Welling <mwelling@ieee.org>");
    MODULE_DESCRIPTION("TSC2004 Touchscreen Driver");
    MODULE_LICENSE("GPL");
