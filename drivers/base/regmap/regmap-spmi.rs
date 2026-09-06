//! Automatically rewritten from C to Rust
//! Source: drivers/base/regmap/regmap-spmi.c
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
// Register map access API - SPMI support
//
// Copyright (c) 2012-2013, The Linux Foundation. All rights reserved.
//
// Based on regmap-i2c.c:
// Copyright 2011 Wolfson Microelectronics plc
// Author: Mark Brown <broonie@opensource.wolfsonmicro.com>

    static int regmap_spmi_base_read(void *context,
    const void *reg, size_t reg_size,
    void *val, size_t val_size)
    {
    let mut addr: u8 = *(u8 *)reg;
    let mut err: c_int = 0;
    BUG_ON(reg_size != 1);
    while (val_size-- && !err)
    err = spmi_register_read(context, addr++, val++);
    return err;
    }
    static int regmap_spmi_base_gather_write(void *context,
    const void *reg, size_t reg_size,
    const void *val, size_t val_size)
    {
    const u8 *data = val;
    let mut addr: u8 = *(u8 *)reg;
    let mut err: c_int = 0;
    BUG_ON(reg_size != 1);
//
// SPMI defines a more bandwidth-efficient 'Register 0 Write' sequence,
// use it when possible.
//
    if (addr == 0 && val_size) {
    err = spmi_register_zero_write(context, *data);
    if (err)
    goto err_out;
    data++;
    addr++;
    val_size--;
    }
    while (val_size) {
    err = spmi_register_write(context, addr, *data);
    if (err)
    goto err_out;
    data++;
    addr++;
    val_size--;
    }
    err_out:
    return err;
    }
    static int regmap_spmi_base_write(void *context, const void *data,
    size_t count)
    {
    BUG_ON(count < 1);
    return regmap_spmi_base_gather_write(context, data, 1, data + 1,
    count - 1);
    }
    static const struct regmap_bus regmap_spmi_base = {
    .read				= regmap_spmi_base_read,
    .write				= regmap_spmi_base_write,
    .gather_write			= regmap_spmi_base_gather_write,
    .reg_format_endian_default	= REGMAP_ENDIAN_NATIVE,
    .val_format_endian_default	= REGMAP_ENDIAN_NATIVE,
    };
    struct regmap *__regmap_init_spmi_base(struct spmi_device *sdev,
    const struct regmap_config *config,
    struct lock_class_key *lock_key,
    const char *lock_name)
    {
    return __regmap_init(&sdev.dev, &regmap_spmi_base, sdev, config,
    lock_key, lock_name);
    }
    EXPORT_SYMBOL_GPL(__regmap_init_spmi_base);
    struct regmap *__devm_regmap_init_spmi_base(struct spmi_device *sdev,
    const struct regmap_config *config,
    struct lock_class_key *lock_key,
    const char *lock_name)
    {
    return __devm_regmap_init(&sdev.dev, &regmap_spmi_base, sdev, config,
    lock_key, lock_name);
    }
    EXPORT_SYMBOL_GPL(__devm_regmap_init_spmi_base);
    static int regmap_spmi_ext_read(void *context,
    const void *reg, size_t reg_size,
    void *val, size_t val_size)
    {
    let mut err: c_int = 0;
    size_t len;
    u16 addr;
    BUG_ON(reg_size != 2);
    addr = *(u16 *)reg;
//
// Split accesses into two to take advantage of the more
// bandwidth-efficient 'Extended Register Read' command when possible
//
    while (addr <= 0xFF && val_size) {
    len = min_t(size_t, val_size, 16);
    err = spmi_ext_register_read(context, addr, val, len);
    if (err)
    goto err_out;
    addr += len;
    val += len;
    val_size -= len;
    }
    while (val_size) {
    len = min_t(size_t, val_size, 8);
    err = spmi_ext_register_readl(context, addr, val, len);
    if (err)
    goto err_out;
    addr += len;
    val += len;
    val_size -= len;
    }
    err_out:
    return err;
    }
    static int regmap_spmi_ext_gather_write(void *context,
    const void *reg, size_t reg_size,
    const void *val, size_t val_size)
    {
    let mut err: c_int = 0;
    size_t len;
    u16 addr;
    BUG_ON(reg_size != 2);
    addr = *(u16 *)reg;
    while (addr <= 0xFF && val_size) {
    len = min_t(size_t, val_size, 16);
    err = spmi_ext_register_write(context, addr, val, len);
    if (err)
    goto err_out;
    addr += len;
    val += len;
    val_size -= len;
    }
    while (val_size) {
    len = min_t(size_t, val_size, 8);
    err = spmi_ext_register_writel(context, addr, val, len);
    if (err)
    goto err_out;
    addr += len;
    val += len;
    val_size -= len;
    }
    err_out:
    return err;
    }
    static int regmap_spmi_ext_write(void *context, const void *data,
    size_t count)
    {
    BUG_ON(count < 2);
    return regmap_spmi_ext_gather_write(context, data, 2, data + 2,
    count - 2);
    }
    static const struct regmap_bus regmap_spmi_ext = {
    .read				= regmap_spmi_ext_read,
    .write				= regmap_spmi_ext_write,
    .gather_write			= regmap_spmi_ext_gather_write,
    .reg_format_endian_default	= REGMAP_ENDIAN_NATIVE,
    .val_format_endian_default	= REGMAP_ENDIAN_NATIVE,
    };
    struct regmap *__regmap_init_spmi_ext(struct spmi_device *sdev,
    const struct regmap_config *config,
    struct lock_class_key *lock_key,
    const char *lock_name)
    {
    return __regmap_init(&sdev.dev, &regmap_spmi_ext, sdev, config,
    lock_key, lock_name);
    }
    EXPORT_SYMBOL_GPL(__regmap_init_spmi_ext);
    struct regmap *__devm_regmap_init_spmi_ext(struct spmi_device *sdev,
    const struct regmap_config *config,
    struct lock_class_key *lock_key,
    const char *lock_name)
    {
    return __devm_regmap_init(&sdev.dev, &regmap_spmi_ext, sdev, config,
    lock_key, lock_name);
    }
    EXPORT_SYMBOL_GPL(__devm_regmap_init_spmi_ext);
    MODULE_DESCRIPTION("Register map access API - SPMI support");
    MODULE_LICENSE("GPL");
