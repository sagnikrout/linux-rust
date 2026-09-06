//! Automatically rewritten from C to Rust
//! Source: drivers/base/regmap/regmap-slimbus.c
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
// Copyright (c) 2017, Linaro Ltd.

#[no_mangle]
unsafe extern "C" fn regmap_slimbus_write(context: *mut c_void, data: *const c_void, count: usize) -> c_int {
    static int regmap_slimbus_write(void *context, const void *data, size_t count)
    {
    struct slim_device *sdev = context;
    return slim_write(sdev, *(u16 *)data, count - 2, (u8 *)data + 2);
    }
    static int regmap_slimbus_read(void *context, const void *reg, size_t reg_size,
    void *val, size_t val_size)
    {
    struct slim_device *sdev = context;
    return slim_read(sdev, *(u16 *)reg, val_size, val);
    }
    static const struct regmap_bus regmap_slimbus_bus = {
    .write = regmap_slimbus_write,
    .read = regmap_slimbus_read,
    .reg_format_endian_default = REGMAP_ENDIAN_LITTLE,
    .val_format_endian_default = REGMAP_ENDIAN_LITTLE,
    };
    static const struct regmap_bus *regmap_get_slimbus(struct slim_device *slim,
    const struct regmap_config *config)
    {
    if (config.val_bits == 8 && config.reg_bits == 16)
    return &regmap_slimbus_bus;
    return ERR_PTR(-ENOTSUPP);
    }
    struct regmap *__regmap_init_slimbus(struct slim_device *slimbus,
    const struct regmap_config *config,
    struct lock_class_key *lock_key,
    const char *lock_name)
    {
    const struct regmap_bus *bus = regmap_get_slimbus(slimbus, config);
    if (IS_ERR(bus))
    return ERR_CAST(bus);
    return __regmap_init(&slimbus.dev, bus, slimbus, config, lock_key, lock_name);
    }
    EXPORT_SYMBOL_GPL(__regmap_init_slimbus);
    struct regmap *__devm_regmap_init_slimbus(struct slim_device *slimbus,
    const struct regmap_config *config,
    struct lock_class_key *lock_key,
    const char *lock_name)
    {
    const struct regmap_bus *bus = regmap_get_slimbus(slimbus, config);
    if (IS_ERR(bus))
    return ERR_CAST(bus);
    return __devm_regmap_init(&slimbus.dev, bus, slimbus, config, lock_key, lock_name);
    }
    EXPORT_SYMBOL_GPL(__devm_regmap_init_slimbus);
    MODULE_DESCRIPTION("Register map access API - SLIMbus support");
    MODULE_LICENSE("GPL v2");
