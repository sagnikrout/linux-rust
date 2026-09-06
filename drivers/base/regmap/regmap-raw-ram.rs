//! Automatically rewritten from C to Rust
//! Source: drivers/base/regmap/regmap-raw-ram.c
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
// Register map access API - Memory region with raw access
//
// This is intended for testing only
//
// Copyright (c) 2023, Arm Ltd

#[no_mangle]
unsafe extern "C" fn decode_reg(endian: enum regmap_endian, reg: *const c_void) -> c_uint {
    static unsigned int decode_reg(enum regmap_endian endian, const void *reg)
    {
    const u16 *r = reg;
    if (endian == REGMAP_ENDIAN_BIG)
    return be16_to_cpu(*r);
    else
    return le16_to_cpu(*r);
    }
    static int regmap_raw_ram_gather_write(void *context,
    const void *reg, size_t reg_len,
    const void *val, size_t val_len)
    {
    struct regmap_ram_data *data = context;
    unsigned int r;
    u16 *our_buf = (u16 *)data.vals;
    int i;
    if (reg_len != 2)
    return -EINVAL;
    if (val_len % 2)
    return -EINVAL;
    r = decode_reg(data.reg_endian, reg);
    if (data.noinc_reg && data.noinc_reg(data, r)) {
    memcpy(&our_buf[r], val + val_len - 2, 2);
    data.written[r] = true;
    } else {
    memcpy(&our_buf[r], val, val_len);
    for (i = 0; i < val_len / 2; i++)
    data.written[r + i] = true;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn regmap_raw_ram_write(context: *mut c_void, data: *const c_void, count: usize) -> c_int {
    static int regmap_raw_ram_write(void *context, const void *data, size_t count)
    {
    return regmap_raw_ram_gather_write(context, data, 2,
    data + 2, count - 2);
    }
    static int regmap_raw_ram_read(void *context,
    const void *reg, size_t reg_len,
    void *val, size_t val_len)
    {
    struct regmap_ram_data *data = context;
    unsigned int r;
    u16 *our_buf = (u16 *)data.vals;
    int i;
    if (reg_len != 2)
    return -EINVAL;
    if (val_len % 2)
    return -EINVAL;
    r = decode_reg(data.reg_endian, reg);
    if (data.noinc_reg && data.noinc_reg(data, r)) {
    for (i = 0; i < val_len; i += 2)
    memcpy(val + i, &our_buf[r], 2);
    data.read[r] = true;
    } else {
    memcpy(val, &our_buf[r], val_len);
    for (i = 0; i < val_len / 2; i++)
    data.read[r + i] = true;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn regmap_raw_ram_free_context(context: *mut c_void) {
    static void regmap_raw_ram_free_context(void *context)
    {
    struct regmap_ram_data *data = context;
    kfree(data.vals);
    kfree(data.read);
    kfree(data.written);
    kfree(data);
    }
    static const struct regmap_bus regmap_raw_ram = {
    .fast_io = true,
    .write = regmap_raw_ram_write,
    .gather_write = regmap_raw_ram_gather_write,
    .read = regmap_raw_ram_read,
    .free_context = regmap_raw_ram_free_context,
    };
    struct regmap *__regmap_init_raw_ram(struct device *dev,
    const struct regmap_config *config,
    struct regmap_ram_data *data,
    struct lock_class_key *lock_key,
    const char *lock_name)
    {
    struct regmap *map;
    if (config.reg_bits != 16)
    return ERR_PTR(-EINVAL);
    if (!config.max_register) {
    pr_crit("No max_register specified for RAM regmap\n");
    return ERR_PTR(-EINVAL);
    }
    data.read = kzalloc_objs(bool, config.max_register + 1);
    if (!data.read)
    return ERR_PTR(-ENOMEM);
    data.written = kzalloc_objs(bool, config.max_register + 1);
    if (!data.written)
    return ERR_PTR(-ENOMEM);
    data.reg_endian = config.reg_format_endian;
    map = __regmap_init(dev, &regmap_raw_ram, data, config,
    lock_key, lock_name);
    return map;
    }
    EXPORT_SYMBOL_GPL(__regmap_init_raw_ram);
    MODULE_DESCRIPTION("Register map access API - Memory region with raw access");
    MODULE_LICENSE("GPL v2");
