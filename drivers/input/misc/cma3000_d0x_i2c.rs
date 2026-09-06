//! Automatically rewritten from C to Rust
//! Source: drivers/input/misc/cma3000_d0x_i2c.c
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
// Implements I2C interface for VTI CMA300_D0x Accelerometer driver
//
// Copyright (C) 2010 Texas Instruments
// Author: Hemanth V <hemanthv@ti.com>
//

    static int cma3000_i2c_set(struct device *dev,
    u8 reg, u8 val, char *msg)
    {
    struct i2c_client *client = to_i2c_client(dev);
    int ret;
    ret = i2c_smbus_write_byte_data(client, reg, val);
    if (ret < 0)
    dev_err(&client.dev,
    "%s failed (%s, %d)\n", __func__, msg, ret);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn cma3000_i2c_read(dev: *mut device, reg: u8, msg: *mut c_char) -> c_int {
    static int cma3000_i2c_read(struct device *dev, u8 reg, char *msg)
    {
    struct i2c_client *client = to_i2c_client(dev);
    int ret;
    ret = i2c_smbus_read_byte_data(client, reg);
    if (ret < 0)
    dev_err(&client.dev,
    "%s failed (%s, %d)\n", __func__, msg, ret);
    return ret;
    }
    static const struct cma3000_bus_ops cma3000_i2c_bops = {
    .bustype	= BUS_I2C,

    .ctrl_mod	= CMA3000_BUSI2C,
    .read		= cma3000_i2c_read,
    .write		= cma3000_i2c_set,
    };
#[no_mangle]
unsafe extern "C" fn cma3000_i2c_probe(client: *mut i2c_client) -> c_int {
    static int cma3000_i2c_probe(struct i2c_client *client)
    {
    struct cma3000_accl_data *data;
    data = cma3000_init(&client.dev, client.irq, &cma3000_i2c_bops);
    if (IS_ERR(data))
    return PTR_ERR(data);
    i2c_set_clientdata(client, data);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cma3000_i2c_remove(client: *mut i2c_client) {
    static void cma3000_i2c_remove(struct i2c_client *client)
    {
    struct cma3000_accl_data *data = i2c_get_clientdata(client);
    cma3000_exit(data);
    }
#[no_mangle]
unsafe extern "C" fn cma3000_i2c_suspend(dev: *mut device) -> c_int {
    static int cma3000_i2c_suspend(struct device *dev)
    {
    struct i2c_client *client = to_i2c_client(dev);
    struct cma3000_accl_data *data = i2c_get_clientdata(client);
    cma3000_suspend(data);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cma3000_i2c_resume(dev: *mut device) -> c_int {
    static int cma3000_i2c_resume(struct device *dev)
    {
    struct i2c_client *client = to_i2c_client(dev);
    struct cma3000_accl_data *data = i2c_get_clientdata(client);
    cma3000_resume(data);
    return 0;
    }
    static const struct dev_pm_ops cma3000_i2c_pm_ops = {
    .suspend	= cma3000_i2c_suspend,
    .resume		= cma3000_i2c_resume,
    };
    static const struct i2c_device_id cma3000_i2c_id[] = {
    { .name = "cma3000_d01" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, cma3000_i2c_id);
    static struct i2c_driver cma3000_i2c_driver = {
    .probe		= cma3000_i2c_probe,
    .remove		= cma3000_i2c_remove,
    .id_table	= cma3000_i2c_id,
    .driver = {
    .name	= "cma3000_i2c_accl",
    .pm	= pm_sleep_ptr(&cma3000_i2c_pm_ops),
    },
    };
    module_i2c_driver(cma3000_i2c_driver);
    MODULE_DESCRIPTION("CMA3000-D0x Accelerometer I2C Driver");
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Hemanth V <hemanthv@ti.com>");
