//! Automatically rewritten from C to Rust
//! Source: drivers/base/regmap/regmap-ram.c
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
// Register map access API - Memory region
//
// This is intended for testing only
//
// Copyright (c) 2023, Arm Ltd

#[no_mangle]
unsafe extern "C" fn regmap_ram_write(context: *mut c_void, reg: c_uint, val: c_uint) -> c_int {
    static int regmap_ram_write(void *context, unsigned int reg, unsigned int val)
    {
    struct regmap_ram_data *data = context;
    data.vals[reg] = val;
    data.written[reg] = true;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn regmap_ram_read(context: *mut c_void, reg: c_uint, val: *mut c_uint) -> c_int {
    static int regmap_ram_read(void *context, unsigned int reg, unsigned int *val)
    {
    struct regmap_ram_data *data = context;
// val = data->vals[reg];
    data.read[reg] = true;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn regmap_ram_free_context(context: *mut c_void) {
    static void regmap_ram_free_context(void *context)
    {
    struct regmap_ram_data *data = context;
    kfree(data.vals);
    kfree(data.read);
    kfree(data.written);
    kfree(data);
    }
    static const struct regmap_bus regmap_ram = {
    .fast_io = true,
    .reg_write = regmap_ram_write,
    .reg_read = regmap_ram_read,
    .free_context = regmap_ram_free_context,
    };
    struct regmap *__regmap_init_ram(struct device *dev,
    const struct regmap_config *config,
    struct regmap_ram_data *data,
    struct lock_class_key *lock_key,
    const char *lock_name)
    {
    struct regmap *map;
    if (!config.max_register) {
    pr_crit("No max_register specified for RAM regmap\n");
    return ERR_PTR(-EINVAL);
    }
    data.read = kzalloc_objs(bool, config.max_register + 1);
    if (!data.read)
    return ERR_PTR(-ENOMEM);
    data.written = kzalloc_objs(bool, config.max_register + 1);
    if (!data.written) {
    kfree(data.read);
    return ERR_PTR(-ENOMEM);
    }
    map = __regmap_init(dev, &regmap_ram, data, config,
    lock_key, lock_name);
    if (IS_ERR(map)) {
    kfree(data.read);
    kfree(data.written);
    }
    return map;
    }
    EXPORT_SYMBOL_GPL(__regmap_init_ram);
    MODULE_DESCRIPTION("Register map access API - Memory region");
    MODULE_LICENSE("GPL v2");
