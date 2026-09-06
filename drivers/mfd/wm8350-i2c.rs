//! Automatically rewritten from C to Rust
//! Source: drivers/mfd/wm8350-i2c.c
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
// wm8350-i2c.c  --  Generic I2C driver for Wolfson WM8350 PMIC
//
// Copyright 2007, 2008 Wolfson Microelectronics PLC.
//
// Author: Liam Girdwood
// linux@wolfsonmicro.com
//

#[no_mangle]
unsafe extern "C" fn wm8350_i2c_probe(i2c: *mut i2c_client) -> c_int {
    static int wm8350_i2c_probe(struct i2c_client *i2c)
    {
    struct wm8350 *wm8350;
    struct wm8350_platform_data *pdata = dev_get_platdata(&i2c.dev);
    let mut ret: c_int = 0;
    wm8350 = devm_kzalloc(&i2c.dev, sizeof(struct wm8350), GFP_KERNEL);
    if (wm8350 == core::ptr::null_mut())
    return -ENOMEM;
    wm8350.regmap = devm_regmap_init_i2c(i2c, &wm8350_regmap);
    if (IS_ERR(wm8350.regmap)) {
    ret = PTR_ERR(wm8350.regmap);
    dev_err(&i2c.dev, "Failed to allocate register map: %d\n",
    ret);
    return ret;
    }
    i2c_set_clientdata(i2c, wm8350);
    wm8350.dev = &i2c.dev;
    return wm8350_device_init(wm8350, i2c.irq, pdata);
    }
    static const struct i2c_device_id wm8350_i2c_id[] = {
    { "wm8350" },
    { "wm8351" },
    { "wm8352" },
    { }
    };
    static struct i2c_driver wm8350_i2c_driver = {
    .driver = {
    .name = "wm8350",
    .suppress_bind_attrs = true,
    },
    .probe = wm8350_i2c_probe,
    .id_table = wm8350_i2c_id,
    };
#[no_mangle]
unsafe extern "C" fn wm8350_i2c_init() -> int __init {
    static int __init wm8350_i2c_init(void)
    {
    return i2c_add_driver(&wm8350_i2c_driver);
    }
// init early so consumer devices can complete system boot
    subsys_initcall(wm8350_i2c_init);
