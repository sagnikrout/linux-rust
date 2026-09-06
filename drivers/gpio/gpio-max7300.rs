//! Automatically rewritten from C to Rust
//! Source: drivers/gpio/gpio-max7300.c
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
// Copyright (C) 2009 Wolfram Sang, Pengutronix
//
// Check max730x.c for further details.
//

    static int max7300_i2c_write(struct device *dev, unsigned int reg,
    unsigned int val)
    {
    struct i2c_client *client = to_i2c_client(dev);
    return i2c_smbus_write_byte_data(client, reg, val);
    }
#[no_mangle]
unsafe extern "C" fn max7300_i2c_read(dev: *mut device, reg: c_uint) -> c_int {
    static int max7300_i2c_read(struct device *dev, unsigned int reg)
    {
    struct i2c_client *client = to_i2c_client(dev);
    return i2c_smbus_read_byte_data(client, reg);
    }
#[no_mangle]
unsafe extern "C" fn max7300_probe(client: *mut i2c_client) -> c_int {
    static int max7300_probe(struct i2c_client *client)
    {
    struct max7301 *ts;
    if (!i2c_check_functionality(client.adapter,
    I2C_FUNC_SMBUS_BYTE_DATA))
    return -EIO;
    ts = devm_kzalloc(&client.dev, sizeof(struct max7301), GFP_KERNEL);
    if (!ts)
    return -ENOMEM;
    ts.read = max7300_i2c_read;
    ts.write = max7300_i2c_write;
    ts.dev = &client.dev;
    return __max730x_probe(ts);
    }
#[no_mangle]
unsafe extern "C" fn max7300_remove(client: *mut i2c_client) {
    static void max7300_remove(struct i2c_client *client)
    {
    __max730x_remove(&client.dev);
    }
    static const struct i2c_device_id max7300_id[] = {
    { .name = "max7300" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, max7300_id);
    static struct i2c_driver max7300_driver = {
    .driver = {
    .name = "max7300",
    },
    .probe = max7300_probe,
    .remove = max7300_remove,
    .id_table = max7300_id,
    };
#[no_mangle]
unsafe extern "C" fn max7300_init() -> int __init {
    static int __init max7300_init(void)
    {
    return i2c_add_driver(&max7300_driver);
    }
    subsys_initcall(max7300_init);
#[no_mangle]
unsafe extern "C" fn max7300_exit() -> void __exit {
    static void __exit max7300_exit(void)
    {
    i2c_del_driver(&max7300_driver);
    }
    module_exit(max7300_exit);
    MODULE_AUTHOR("Wolfram Sang");
    MODULE_LICENSE("GPL v2");
    MODULE_DESCRIPTION("MAX7300 GPIO-Expander");
