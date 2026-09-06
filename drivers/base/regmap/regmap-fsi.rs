//! Automatically rewritten from C to Rust
//! Source: drivers/base/regmap/regmap-fsi.c
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
// Register map access API - FSI support
//
// Copyright 2022 IBM Corp
//
// Author: Eddie James <eajames@linux.ibm.com>

#[no_mangle]
unsafe extern "C" fn regmap_fsi32_reg_read(context: *mut c_void, reg: c_uint, val: *mut c_uint) -> c_int {
    static int regmap_fsi32_reg_read(void *context, unsigned int reg, unsigned int *val)
    {
    u32 v;
    int ret;
    ret = fsi_slave_read(context, reg, &v, sizeof(v));
    if (ret)
    return ret;
// val = v;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn regmap_fsi32_reg_write(context: *mut c_void, reg: c_uint, val: c_uint) -> c_int {
    static int regmap_fsi32_reg_write(void *context, unsigned int reg, unsigned int val)
    {
    let mut v: u32 = val;
    return fsi_slave_write(context, reg, &v, sizeof(v));
    }
    static const struct regmap_bus regmap_fsi32 = {
    .reg_write = regmap_fsi32_reg_write,
    .reg_read = regmap_fsi32_reg_read,
    };
#[no_mangle]
unsafe extern "C" fn regmap_fsi32le_reg_read(context: *mut c_void, reg: c_uint, val: *mut c_uint) -> c_int {
    static int regmap_fsi32le_reg_read(void *context, unsigned int reg, unsigned int *val)
    {
    __be32 v;
    int ret;
    ret = fsi_slave_read(context, reg, &v, sizeof(v));
    if (ret)
    return ret;
// val = be32_to_cpu(v);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn regmap_fsi32le_reg_write(context: *mut c_void, reg: c_uint, val: c_uint) -> c_int {
    static int regmap_fsi32le_reg_write(void *context, unsigned int reg, unsigned int val)
    {
    let mut v: __be32 = cpu_to_be32(val);
    return fsi_slave_write(context, reg, &v, sizeof(v));
    }
    static const struct regmap_bus regmap_fsi32le = {
    .reg_write = regmap_fsi32le_reg_write,
    .reg_read = regmap_fsi32le_reg_read,
    };
#[no_mangle]
unsafe extern "C" fn regmap_fsi16_reg_read(context: *mut c_void, reg: c_uint, val: *mut c_uint) -> c_int {
    static int regmap_fsi16_reg_read(void *context, unsigned int reg, unsigned int *val)
    {
    u16 v;
    int ret;
    ret = fsi_slave_read(context, reg, &v, sizeof(v));
    if (ret)
    return ret;
// val = v;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn regmap_fsi16_reg_write(context: *mut c_void, reg: c_uint, val: c_uint) -> c_int {
    static int regmap_fsi16_reg_write(void *context, unsigned int reg, unsigned int val)
    {
    u16 v;
    if (val > 0xffff)
    return -EINVAL;
    v = val;
    return fsi_slave_write(context, reg, &v, sizeof(v));
    }
    static const struct regmap_bus regmap_fsi16 = {
    .reg_write = regmap_fsi16_reg_write,
    .reg_read = regmap_fsi16_reg_read,
    };
#[no_mangle]
unsafe extern "C" fn regmap_fsi16le_reg_read(context: *mut c_void, reg: c_uint, val: *mut c_uint) -> c_int {
    static int regmap_fsi16le_reg_read(void *context, unsigned int reg, unsigned int *val)
    {
    __be16 v;
    int ret;
    ret = fsi_slave_read(context, reg, &v, sizeof(v));
    if (ret)
    return ret;
// val = be16_to_cpu(v);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn regmap_fsi16le_reg_write(context: *mut c_void, reg: c_uint, val: c_uint) -> c_int {
    static int regmap_fsi16le_reg_write(void *context, unsigned int reg, unsigned int val)
    {
    __be16 v;
    if (val > 0xffff)
    return -EINVAL;
    v = cpu_to_be16(val);
    return fsi_slave_write(context, reg, &v, sizeof(v));
    }
    static const struct regmap_bus regmap_fsi16le = {
    .reg_write = regmap_fsi16le_reg_write,
    .reg_read = regmap_fsi16le_reg_read,
    };
#[no_mangle]
unsafe extern "C" fn regmap_fsi8_reg_read(context: *mut c_void, reg: c_uint, val: *mut c_uint) -> c_int {
    static int regmap_fsi8_reg_read(void *context, unsigned int reg, unsigned int *val)
    {
    u8 v;
    int ret;
    ret = fsi_slave_read(context, reg, &v, sizeof(v));
    if (ret)
    return ret;
// val = v;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn regmap_fsi8_reg_write(context: *mut c_void, reg: c_uint, val: c_uint) -> c_int {
    static int regmap_fsi8_reg_write(void *context, unsigned int reg, unsigned int val)
    {
    u8 v;
    if (val > 0xff)
    return -EINVAL;
    v = val;
    return fsi_slave_write(context, reg, &v, sizeof(v));
    }
    static const struct regmap_bus regmap_fsi8 = {
    .reg_write = regmap_fsi8_reg_write,
    .reg_read = regmap_fsi8_reg_read,
    };
    static const struct regmap_bus *regmap_get_fsi_bus(struct fsi_device *fsi_dev,
    const struct regmap_config *config)
    {
    const struct regmap_bus *bus = core::ptr::null_mut();
    if (config.reg_bits == 8 || config.reg_bits == 16 || config.reg_bits == 32) {
    switch (config.val_bits) {
    case 8:
    bus = &regmap_fsi8;
    break;
    case 16:
    switch (regmap_get_val_endian(&fsi_dev.dev, core::ptr::null_mut(), config)) {
    case REGMAP_ENDIAN_LITTLE:

    case REGMAP_ENDIAN_NATIVE:

    bus = &regmap_fsi16le;
    break;
    case REGMAP_ENDIAN_DEFAULT:
    case REGMAP_ENDIAN_BIG:

    case REGMAP_ENDIAN_NATIVE:

    bus = &regmap_fsi16;
    break;
    default:
    break;
    }
    break;
    case 32:
    switch (regmap_get_val_endian(&fsi_dev.dev, core::ptr::null_mut(), config)) {
    case REGMAP_ENDIAN_LITTLE:

    case REGMAP_ENDIAN_NATIVE:

    bus = &regmap_fsi32le;
    break;
    case REGMAP_ENDIAN_DEFAULT:
    case REGMAP_ENDIAN_BIG:

    case REGMAP_ENDIAN_NATIVE:

    bus = &regmap_fsi32;
    break;
    default:
    break;
    }
    break;
    }
    }
    return bus ?: ERR_PTR(-EOPNOTSUPP);
    }
    struct regmap *__regmap_init_fsi(struct fsi_device *fsi_dev, const struct regmap_config *config,
    struct lock_class_key *lock_key, const char *lock_name)
    {
    const struct regmap_bus *bus = regmap_get_fsi_bus(fsi_dev, config);
    if (IS_ERR(bus))
    return ERR_CAST(bus);
    return __regmap_init(&fsi_dev.dev, bus, fsi_dev.slave, config, lock_key, lock_name);
    }
    EXPORT_SYMBOL_GPL(__regmap_init_fsi);
    struct regmap *__devm_regmap_init_fsi(struct fsi_device *fsi_dev,
    const struct regmap_config *config,
    struct lock_class_key *lock_key, const char *lock_name)
    {
    const struct regmap_bus *bus = regmap_get_fsi_bus(fsi_dev, config);
    if (IS_ERR(bus))
    return ERR_CAST(bus);
    return __devm_regmap_init(&fsi_dev.dev, bus, fsi_dev.slave, config, lock_key, lock_name);
    }
    EXPORT_SYMBOL_GPL(__devm_regmap_init_fsi);
    MODULE_LICENSE("GPL");
