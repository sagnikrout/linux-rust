//! Automatically rewritten from C to Rust
//! Source: drivers/base/regmap/regmap-i3c.c
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
// Copyright (c) 2018 Synopsys, Inc. and/or its affiliates.

#[no_mangle]
unsafe extern "C" fn regmap_i3c_write(context: *mut c_void, data: *const c_void, count: usize) -> c_int {
    static int regmap_i3c_write(void *context, const void *data, size_t count)
    {
    struct device *dev = context;
    struct i3c_device *i3c = dev_to_i3cdev(dev);
    struct i3c_xfer xfers[] = {
    {
    .rnw = false,
    .len = count,
    .data.out = data,
    },
    };
    return i3c_device_do_xfers(i3c, xfers, ARRAY_SIZE(xfers), I3C_SDR);
    }
    static int regmap_i3c_read(void *context,
    const void *reg, size_t reg_size,
    void *val, size_t val_size)
    {
    struct device *dev = context;
    struct i3c_device *i3c = dev_to_i3cdev(dev);
    struct i3c_xfer xfers[2];
    xfers[0].rnw = false;
    xfers[0].len = reg_size;
    xfers[0].data.out = reg;
    xfers[1].rnw = true;
    xfers[1].len = val_size;
    xfers[1].data.in = val;
    return i3c_device_do_xfers(i3c, xfers, ARRAY_SIZE(xfers), I3C_SDR);
    }
    static const struct regmap_bus regmap_i3c = {
    .write = regmap_i3c_write,
    .read = regmap_i3c_read,
    };
    struct regmap *__regmap_init_i3c(struct i3c_device *i3c,
    const struct regmap_config *config,
    struct lock_class_key *lock_key,
    const char *lock_name)
    {
    return __regmap_init(&i3c.dev, &regmap_i3c, &i3c.dev, config,
    lock_key, lock_name);
    }
    EXPORT_SYMBOL_GPL(__regmap_init_i3c);
    struct regmap *__devm_regmap_init_i3c(struct i3c_device *i3c,
    const struct regmap_config *config,
    struct lock_class_key *lock_key,
    const char *lock_name)
    {
    return __devm_regmap_init(&i3c.dev, &regmap_i3c, &i3c.dev, config,
    lock_key, lock_name);
    }
    EXPORT_SYMBOL_GPL(__devm_regmap_init_i3c);
    MODULE_AUTHOR("Vitor Soares <vitor.soares@synopsys.com>");
    MODULE_DESCRIPTION("regmap I3C Module");
    MODULE_LICENSE("GPL v2");
