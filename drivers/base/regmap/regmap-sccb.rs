//! Automatically rewritten from C to Rust
//! Source: drivers/base/regmap/regmap-sccb.c
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
// Register map access API - SCCB support

//
// sccb_is_available - Check if the adapter supports SCCB protocol
// @adap: I2C adapter
//
// Return true if the I2C adapter is capable of using SCCB helper functions,
// false otherwise.
//
#[no_mangle]
unsafe extern "C" fn sccb_is_available(adap: *mut i2c_adapter) -> bool {
    static bool sccb_is_available(struct i2c_adapter *adap)
    {
    let mut needed_funcs: u32 = I2C_FUNC_SMBUS_BYTE | I2C_FUNC_SMBUS_WRITE_BYTE_DATA;
//
// If we ever want support for hardware doing SCCB natively, we will
// introduce a sccb_xfer() callback to struct i2c_algorithm and check
// for it here.
//
    return (i2c_get_functionality(adap) & needed_funcs) == needed_funcs;
    }
//
// regmap_sccb_read - Read data from SCCB slave device
// @context: Device that will be interacted with
// @reg: Register to be read from
// @val: Pointer to store read value
//
// This executes the 2-phase write transmission cycle that is followed by a
// 2-phase read transmission cycle, returning negative errno else zero on
// success.
//
#[no_mangle]
unsafe extern "C" fn regmap_sccb_read(context: *mut c_void, reg: c_uint, val: *mut c_uint) -> c_int {
    static int regmap_sccb_read(void *context, unsigned int reg, unsigned int *val)
    {
    struct device *dev = context;
    struct i2c_client *i2c = to_i2c_client(dev);
    int ret;
    union i2c_smbus_data data;
    i2c_lock_bus(i2c.adapter, I2C_LOCK_SEGMENT);
    ret = __i2c_smbus_xfer(i2c.adapter, i2c.addr, i2c.flags,
    I2C_SMBUS_WRITE, reg, I2C_SMBUS_BYTE, core::ptr::null_mut());
    if (ret < 0)
    goto out;
    ret = __i2c_smbus_xfer(i2c.adapter, i2c.addr, i2c.flags,
    I2C_SMBUS_READ, 0, I2C_SMBUS_BYTE, &data);
    if (ret < 0)
    goto out;
// val = data.byte;
    out:
    i2c_unlock_bus(i2c.adapter, I2C_LOCK_SEGMENT);
    return ret;
    }
//
// regmap_sccb_write - Write data to SCCB slave device
// @context: Device that will be interacted with
// @reg: Register to write to
// @val: Value to be written
//
// This executes the SCCB 3-phase write transmission cycle, returning negative
// errno else zero on success.
//
#[no_mangle]
unsafe extern "C" fn regmap_sccb_write(context: *mut c_void, reg: c_uint, val: c_uint) -> c_int {
    static int regmap_sccb_write(void *context, unsigned int reg, unsigned int val)
    {
    struct device *dev = context;
    struct i2c_client *i2c = to_i2c_client(dev);
    return i2c_smbus_write_byte_data(i2c, reg, val);
    }
    static const struct regmap_bus regmap_sccb_bus = {
    .reg_write = regmap_sccb_write,
    .reg_read = regmap_sccb_read,
    };
    static const struct regmap_bus *regmap_get_sccb_bus(struct i2c_client *i2c,
    const struct regmap_config *config)
    {
    if (config.val_bits == 8 && config.reg_bits == 8 &&
    sccb_is_available(i2c.adapter))
    return &regmap_sccb_bus;
    return ERR_PTR(-ENOTSUPP);
    }
    struct regmap *__regmap_init_sccb(struct i2c_client *i2c,
    const struct regmap_config *config,
    struct lock_class_key *lock_key,
    const char *lock_name)
    {
    const struct regmap_bus *bus = regmap_get_sccb_bus(i2c, config);
    if (IS_ERR(bus))
    return ERR_CAST(bus);
    return __regmap_init(&i2c.dev, bus, &i2c.dev, config,
    lock_key, lock_name);
    }
    EXPORT_SYMBOL_GPL(__regmap_init_sccb);
    struct regmap *__devm_regmap_init_sccb(struct i2c_client *i2c,
    const struct regmap_config *config,
    struct lock_class_key *lock_key,
    const char *lock_name)
    {
    const struct regmap_bus *bus = regmap_get_sccb_bus(i2c, config);
    if (IS_ERR(bus))
    return ERR_CAST(bus);
    return __devm_regmap_init(&i2c.dev, bus, &i2c.dev, config,
    lock_key, lock_name);
    }
    EXPORT_SYMBOL_GPL(__devm_regmap_init_sccb);
    MODULE_DESCRIPTION("Register map access API - SCCB support");
    MODULE_LICENSE("GPL v2");
