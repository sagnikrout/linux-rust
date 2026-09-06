//! Automatically rewritten from C to Rust
//! Source: drivers/base/regmap/regmap-w1.c
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
// Register map access API - W1 (1-Wire) support
//
// Copyright (c) 2017 Radioavionica Corporation
// Author: Alex A. Mihaylov <minimumlaw@rambler.ru>

pub const W1_CMD_READ_DATA: c_uint = 0x69;
pub const W1_CMD_WRITE_DATA: c_uint = 0x6C;
//
// 1-Wire slaves registers with addess 8 bit and data 8 bit
//
#[no_mangle]
unsafe extern "C" fn w1_reg_a8_v8_read(context: *mut c_void, reg: c_uint, val: *mut c_uint) -> c_int {
    static int w1_reg_a8_v8_read(void *context, unsigned int reg, unsigned int *val)
    {
    struct device *dev = context;
    struct w1_slave *sl = container_of(dev, struct w1_slave, dev);
    let mut ret: c_int = 0;
    if (reg > 255)
    return -EINVAL;
    mutex_lock(&sl.master.bus_mutex);
    if (!w1_reset_select_slave(sl)) {
    w1_write_8(sl.master, W1_CMD_READ_DATA);
    w1_write_8(sl.master, reg);
// val = w1_read_8(sl->master);
    } else {
    ret = -ENODEV;
    }
    mutex_unlock(&sl.master.bus_mutex);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn w1_reg_a8_v8_write(context: *mut c_void, reg: c_uint, val: c_uint) -> c_int {
    static int w1_reg_a8_v8_write(void *context, unsigned int reg, unsigned int val)
    {
    struct device *dev = context;
    struct w1_slave *sl = container_of(dev, struct w1_slave, dev);
    let mut ret: c_int = 0;
    if (reg > 255)
    return -EINVAL;
    mutex_lock(&sl.master.bus_mutex);
    if (!w1_reset_select_slave(sl)) {
    w1_write_8(sl.master, W1_CMD_WRITE_DATA);
    w1_write_8(sl.master, reg);
    w1_write_8(sl.master, val);
    } else {
    ret = -ENODEV;
    }
    mutex_unlock(&sl.master.bus_mutex);
    return ret;
    }
//
// 1-Wire slaves registers with addess 8 bit and data 16 bit
//
    static int w1_reg_a8_v16_read(void *context, unsigned int reg,
    unsigned int *val)
    {
    struct device *dev = context;
    struct w1_slave *sl = container_of(dev, struct w1_slave, dev);
    let mut ret: c_int = 0;
    if (reg > 255)
    return -EINVAL;
    mutex_lock(&sl.master.bus_mutex);
    if (!w1_reset_select_slave(sl)) {
    w1_write_8(sl.master, W1_CMD_READ_DATA);
    w1_write_8(sl.master, reg);
// val = w1_read_8(sl->master);
// val |= w1_read_8(sl->master)<<8;
    } else {
    ret = -ENODEV;
    }
    mutex_unlock(&sl.master.bus_mutex);
    return ret;
    }
    static int w1_reg_a8_v16_write(void *context, unsigned int reg,
    unsigned int val)
    {
    struct device *dev = context;
    struct w1_slave *sl = container_of(dev, struct w1_slave, dev);
    let mut ret: c_int = 0;
    if (reg > 255)
    return -EINVAL;
    mutex_lock(&sl.master.bus_mutex);
    if (!w1_reset_select_slave(sl)) {
    w1_write_8(sl.master, W1_CMD_WRITE_DATA);
    w1_write_8(sl.master, reg);
    w1_write_8(sl.master, val & 0x00FF);
    w1_write_8(sl.master, val>>8 & 0x00FF);
    } else {
    ret = -ENODEV;
    }
    mutex_unlock(&sl.master.bus_mutex);
    return ret;
    }
//
// 1-Wire slaves registers with addess 16 bit and data 16 bit
//
    static int w1_reg_a16_v16_read(void *context, unsigned int reg,
    unsigned int *val)
    {
    struct device *dev = context;
    struct w1_slave *sl = container_of(dev, struct w1_slave, dev);
    let mut ret: c_int = 0;
    if (reg > 65535)
    return -EINVAL;
    mutex_lock(&sl.master.bus_mutex);
    if (!w1_reset_select_slave(sl)) {
    w1_write_8(sl.master, W1_CMD_READ_DATA);
    w1_write_8(sl.master, reg & 0x00FF);
    w1_write_8(sl.master, reg>>8 & 0x00FF);
// val = w1_read_8(sl->master);
// val |= w1_read_8(sl->master)<<8;
    } else {
    ret = -ENODEV;
    }
    mutex_unlock(&sl.master.bus_mutex);
    return ret;
    }
    static int w1_reg_a16_v16_write(void *context, unsigned int reg,
    unsigned int val)
    {
    struct device *dev = context;
    struct w1_slave *sl = container_of(dev, struct w1_slave, dev);
    let mut ret: c_int = 0;
    if (reg > 65535)
    return -EINVAL;
    mutex_lock(&sl.master.bus_mutex);
    if (!w1_reset_select_slave(sl)) {
    w1_write_8(sl.master, W1_CMD_WRITE_DATA);
    w1_write_8(sl.master, reg & 0x00FF);
    w1_write_8(sl.master, reg>>8 & 0x00FF);
    w1_write_8(sl.master, val & 0x00FF);
    w1_write_8(sl.master, val>>8 & 0x00FF);
    } else {
    ret = -ENODEV;
    }
    mutex_unlock(&sl.master.bus_mutex);
    return ret;
    }
//
// Various types of supported bus addressing
//
    static const struct regmap_bus regmap_w1_bus_a8_v8 = {
    .reg_read = w1_reg_a8_v8_read,
    .reg_write = w1_reg_a8_v8_write,
    };
    static const struct regmap_bus regmap_w1_bus_a8_v16 = {
    .reg_read = w1_reg_a8_v16_read,
    .reg_write = w1_reg_a8_v16_write,
    };
    static const struct regmap_bus regmap_w1_bus_a16_v16 = {
    .reg_read = w1_reg_a16_v16_read,
    .reg_write = w1_reg_a16_v16_write,
    };
    static const struct regmap_bus *regmap_get_w1_bus(struct device *w1_dev,
    const struct regmap_config *config)
    {
    if (config.reg_bits == 8 && config.val_bits == 8)
    return &regmap_w1_bus_a8_v8;
    if (config.reg_bits == 8 && config.val_bits == 16)
    return &regmap_w1_bus_a8_v16;
    if (config.reg_bits == 16 && config.val_bits == 16)
    return &regmap_w1_bus_a16_v16;
    return ERR_PTR(-ENOTSUPP);
    }
    struct regmap *__regmap_init_w1(struct device *w1_dev,
    const struct regmap_config *config,
    struct lock_class_key *lock_key,
    const char *lock_name)
    {
    const struct regmap_bus *bus = regmap_get_w1_bus(w1_dev, config);
    if (IS_ERR(bus))
    return ERR_CAST(bus);
    return __regmap_init(w1_dev, bus, w1_dev, config,
    lock_key, lock_name);
    }
    EXPORT_SYMBOL_GPL(__regmap_init_w1);
    struct regmap *__devm_regmap_init_w1(struct device *w1_dev,
    const struct regmap_config *config,
    struct lock_class_key *lock_key,
    const char *lock_name)
    {
    const struct regmap_bus *bus = regmap_get_w1_bus(w1_dev, config);
    if (IS_ERR(bus))
    return ERR_CAST(bus);
    return __devm_regmap_init(w1_dev, bus, w1_dev, config,
    lock_key, lock_name);
    }
    EXPORT_SYMBOL_GPL(__devm_regmap_init_w1);
    MODULE_DESCRIPTION("Register map access API - W1 (1-Wire) support");
    MODULE_LICENSE("GPL");
