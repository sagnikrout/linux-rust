//! Automatically rewritten from C to Rust
//! Source: drivers/mfd/stmpe-i2c.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// ST Microelectronics MFD: stmpe's i2c client specific driver
//
// Copyright (C) ST-Ericsson SA 2010
// Copyright (C) ST Microelectronics SA 2011
//
// Author: Rabin Vincent <rabin.vincent@stericsson.com> for ST-Ericsson
// Author: Viresh Kumar <vireshk@kernel.org> for ST Microelectronics
//

#[no_mangle]
unsafe extern "C" fn i2c_reg_read(stmpe: *mut stmpe, reg: u8) -> c_int {
    static int i2c_reg_read(struct stmpe *stmpe, u8 reg)
    {
    struct i2c_client *i2c = stmpe.client;
    return i2c_smbus_read_byte_data(i2c, reg);
    }
#[no_mangle]
unsafe extern "C" fn i2c_reg_write(stmpe: *mut stmpe, reg: u8, val: u8) -> c_int {
    static int i2c_reg_write(struct stmpe *stmpe, u8 reg, u8 val)
    {
    struct i2c_client *i2c = stmpe.client;
    return i2c_smbus_write_byte_data(i2c, reg, val);
    }
#[no_mangle]
unsafe extern "C" fn i2c_block_read(stmpe: *mut stmpe, reg: u8, length: u8, values: *mut u8) -> c_int {
    static int i2c_block_read(struct stmpe *stmpe, u8 reg, u8 length, u8 *values)
    {
    struct i2c_client *i2c = stmpe.client;
    return i2c_smbus_read_i2c_block_data(i2c, reg, length, values);
    }
    static int i2c_block_write(struct stmpe *stmpe, u8 reg, u8 length,
    const u8 *values)
    {
    struct i2c_client *i2c = stmpe.client;
    return i2c_smbus_write_i2c_block_data(i2c, reg, length, values);
    }
    static struct stmpe_client_info i2c_ci = {
    .read_byte = i2c_reg_read,
    .write_byte = i2c_reg_write,
    .read_block = i2c_block_read,
    .write_block = i2c_block_write,
    };
    static const struct of_device_id stmpe_of_match[] = {
    { .compatible = "st,stmpe610", .data = (void *)STMPE610, },
    { .compatible = "st,stmpe801", .data = (void *)STMPE801, },
    { .compatible = "st,stmpe811", .data = (void *)STMPE811, },
    { .compatible = "st,stmpe1600", .data = (void *)STMPE1600, },
    { .compatible = "st,stmpe1601", .data = (void *)STMPE1601, },
    { .compatible = "st,stmpe1801", .data = (void *)STMPE1801, },
    { .compatible = "st,stmpe2401", .data = (void *)STMPE2401, },
    { .compatible = "st,stmpe2403", .data = (void *)STMPE2403, },
    {},
    };
    MODULE_DEVICE_TABLE(of, stmpe_of_match);
    static int
    stmpe_i2c_probe(struct i2c_client *i2c)
    {
    const struct i2c_device_id *id = i2c_client_get_device_id(i2c);
    enum stmpe_partnum partnum;
    const struct of_device_id *of_id;
    i2c_ci.data = (void *)id;
    i2c_ci.irq = i2c.irq;
    i2c_ci.client = i2c;
    i2c_ci.dev = &i2c.dev;
    of_id = of_match_device(stmpe_of_match, &i2c.dev);
    if (!of_id) {
//
// This happens when the I2C ID matches the node name
// but no real compatible string has been given.
//
    dev_info(&i2c.dev, "matching on node name, compatible is preferred\n");
    partnum = id.driver_data;
    } else
    partnum = (uintptr_t)of_id.data;
    return stmpe_probe(&i2c_ci, partnum);
    }
#[no_mangle]
unsafe extern "C" fn stmpe_i2c_remove(i2c: *mut i2c_client) {
    static void stmpe_i2c_remove(struct i2c_client *i2c)
    {
    struct stmpe *stmpe = dev_get_drvdata(&i2c.dev);
    stmpe_remove(stmpe);
    }
    static const struct i2c_device_id stmpe_i2c_id[] = {
    { "stmpe610", STMPE610 },
    { "stmpe801", STMPE801 },
    { "stmpe811", STMPE811 },
    { "stmpe1600", STMPE1600 },
    { "stmpe1601", STMPE1601 },
    { "stmpe1801", STMPE1801 },
    { "stmpe2401", STMPE2401 },
    { "stmpe2403", STMPE2403 },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, stmpe_i2c_id);
    static struct i2c_driver stmpe_i2c_driver = {
    .driver = {
    .name = "stmpe-i2c",
    .pm = pm_sleep_ptr(&stmpe_dev_pm_ops),
    .of_match_table = stmpe_of_match,
    },
    .probe		= stmpe_i2c_probe,
    .remove		= stmpe_i2c_remove,
    .id_table	= stmpe_i2c_id,
    };
    module_i2c_driver(stmpe_i2c_driver);
    MODULE_DESCRIPTION("STMPE MFD I2C Interface Driver");
    MODULE_AUTHOR("Rabin Vincent <rabin.vincent@stericsson.com>");
    MODULE_LICENSE("GPL");
