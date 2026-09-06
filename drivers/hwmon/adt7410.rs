//! Automatically rewritten from C to Rust
//! Source: drivers/hwmon/adt7410.c
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
// ADT7410/ADT7420 digital temperature sensor driver
//
// Copyright 2012-2013 Analog Devices Inc.
// Author: Lars-Peter Clausen <lars@metafoo.de>
//

#[no_mangle]
unsafe extern "C" fn adt7410_regmap_is_volatile(dev: *mut device, reg: c_uint) -> bool {
    static bool adt7410_regmap_is_volatile(struct device *dev, unsigned int reg)
    {
    switch (reg) {
    case ADT7X10_TEMPERATURE:
    case ADT7X10_STATUS:
    return true;
    default:
    return false;
    }
    }
#[no_mangle]
unsafe extern "C" fn adt7410_reg_read(context: *mut c_void, reg: c_uint, val: *mut c_uint) -> c_int {
    static int adt7410_reg_read(void *context, unsigned int reg, unsigned int *val)
    {
    struct i2c_client *client = context;
    int regval;
    switch (reg) {
    case ADT7X10_TEMPERATURE:
    case ADT7X10_T_ALARM_HIGH:
    case ADT7X10_T_ALARM_LOW:
    case ADT7X10_T_CRIT:
    regval = i2c_smbus_read_word_swapped(client, reg);
    break;
    default:
    regval = i2c_smbus_read_byte_data(client, reg);
    break;
    }
    if (regval < 0)
    return regval;
// val = regval;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn adt7410_reg_write(context: *mut c_void, reg: c_uint, val: c_uint) -> c_int {
    static int adt7410_reg_write(void *context, unsigned int reg, unsigned int val)
    {
    struct i2c_client *client = context;
    int ret;
    switch (reg) {
    case ADT7X10_TEMPERATURE:
    case ADT7X10_T_ALARM_HIGH:
    case ADT7X10_T_ALARM_LOW:
    case ADT7X10_T_CRIT:
    ret = i2c_smbus_write_word_swapped(client, reg, val);
    break;
    default:
    ret = i2c_smbus_write_byte_data(client, reg, val);
    break;
    }
    return ret;
    }
    static const struct regmap_config adt7410_regmap_config = {
    .reg_bits = 8,
    .val_bits = 16,
    .max_register = ADT7X10_ID,
    .cache_type = REGCACHE_MAPLE,
    .volatile_reg = adt7410_regmap_is_volatile,
    .reg_read = adt7410_reg_read,
    .reg_write = adt7410_reg_write,
    };
#[no_mangle]
unsafe extern "C" fn adt7410_i2c_probe(client: *mut i2c_client) -> c_int {
    static int adt7410_i2c_probe(struct i2c_client *client)
    {
    struct regmap *regmap;
    regmap = devm_regmap_init(&client.dev, core::ptr::null_mut(), client,
    &adt7410_regmap_config);
    if (IS_ERR(regmap))
    return PTR_ERR(regmap);
    return adt7x10_probe(&client.dev, client.name, client.irq, regmap);
    }
    static const struct i2c_device_id adt7410_ids[] = {
    { .name = "adt7410" },
    { .name = "adt7420" },
    { .name = "adt7422" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, adt7410_ids);
    static const struct of_device_id adt7410_of_match[] = {
    { .compatible = "adi,adt7410" },
    { .compatible = "adi,adt7420" },
    { .compatible = "adi,adt7422" },
    { }
    };
    MODULE_DEVICE_TABLE(of, adt7410_of_match);
    static struct i2c_driver adt7410_driver = {
    .driver = {
    .name	= "adt7410",
    .pm	= pm_sleep_ptr(&adt7x10_dev_pm_ops),
    .of_match_table = adt7410_of_match,
    },
    .probe		= adt7410_i2c_probe,
    .id_table	= adt7410_ids,
    };
    module_i2c_driver(adt7410_driver);
    MODULE_AUTHOR("Lars-Peter Clausen <lars@metafoo.de>");
    MODULE_DESCRIPTION("ADT7410/AD7420 driver");
    MODULE_LICENSE("GPL");
