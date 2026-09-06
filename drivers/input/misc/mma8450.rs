//! Automatically rewritten from C to Rust
//! Source: drivers/input/misc/mma8450.c
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
// Driver for Freescale's 3-Axis Accelerometer MMA8450
//
// Copyright (C) 2011 Freescale Semiconductor, Inc. All Rights Reserved.
//

pub const MODE_CHANGE_DELAY_MS: c_int = 100;
pub const POLL_INTERVAL: c_int = 100;
pub const POLL_INTERVAL_MAX: c_int = 500;
// register definitions
pub const MMA8450_STATUS: c_uint = 0x00;
pub const MMA8450_STATUS_ZXYDR: c_uint = 0x08;
pub const MMA8450_OUT_X8: c_uint = 0x01;
pub const MMA8450_OUT_Y8: c_uint = 0x02;
pub const MMA8450_OUT_Z8: c_uint = 0x03;
pub const MMA8450_OUT_X_LSB: c_uint = 0x05;
pub const MMA8450_OUT_X_MSB: c_uint = 0x06;
pub const MMA8450_OUT_Y_LSB: c_uint = 0x07;
pub const MMA8450_OUT_Y_MSB: c_uint = 0x08;
pub const MMA8450_OUT_Z_LSB: c_uint = 0x09;
pub const MMA8450_OUT_Z_MSB: c_uint = 0x0a;
pub const MMA8450_XYZ_DATA_CFG: c_uint = 0x16;
pub const MMA8450_CTRL_REG1: c_uint = 0x38;
pub const MMA8450_CTRL_REG2: c_uint = 0x39;
pub const MMA8450_ID: c_uint = 0xc6;
pub const MMA8450_WHO_AM_I: c_uint = 0x0f;
#[no_mangle]
unsafe extern "C" fn mma8450_read(c: *mut i2c_client, off: c_uint) -> c_int {
    static int mma8450_read(struct i2c_client *c, unsigned int off)
    {
    int ret;
    ret = i2c_smbus_read_byte_data(c, off);
    if (ret < 0)
    dev_err(&c.dev,
    "failed to read register 0x%02x, error %d\n",
    off, ret);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mma8450_write(c: *mut i2c_client, off: c_uint, v: u8) -> c_int {
    static int mma8450_write(struct i2c_client *c, unsigned int off, u8 v)
    {
    int error;
    error = i2c_smbus_write_byte_data(c, off, v);
    if (error < 0) {
    dev_err(&c.dev,
    "failed to write to register 0x%02x, error %d\n",
    off, error);
    return error;
    }
    return 0;
    }
    static int mma8450_read_block(struct i2c_client *c, unsigned int off,
    u8 *buf, size_t size)
    {
    int err;
    err = i2c_smbus_read_i2c_block_data(c, off, size, buf);
    if (err < 0) {
    dev_err(&c.dev,
    "failed to read block data at 0x%02x, error %d\n",
    MMA8450_OUT_X_LSB, err);
    return err;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mma8450_poll(input: *mut input_dev) {
    static void mma8450_poll(struct input_dev *input)
    {
    struct i2c_client *c = input_get_drvdata(input);
    int x, y, z;
    int ret;
    u8 buf[6];
    ret = mma8450_read(c, MMA8450_STATUS);
    if (ret < 0)
    return;
    if (!(ret & MMA8450_STATUS_ZXYDR))
    return;
    ret = mma8450_read_block(c, MMA8450_OUT_X_LSB, buf, sizeof(buf));
    if (ret < 0)
    return;
    x = ((int)(s8)buf[1] << 4) | (buf[0] & 0xf);
    y = ((int)(s8)buf[3] << 4) | (buf[2] & 0xf);
    z = ((int)(s8)buf[5] << 4) | (buf[4] & 0xf);
    input_report_abs(input, ABS_X, x);
    input_report_abs(input, ABS_Y, y);
    input_report_abs(input, ABS_Z, z);
    input_sync(input);
    }
// Initialize the MMA8450 chip
#[no_mangle]
unsafe extern "C" fn mma8450_open(input: *mut input_dev) -> c_int {
    static int mma8450_open(struct input_dev *input)
    {
    struct i2c_client *c = input_get_drvdata(input);
    int err;
// enable all events from X/Y/Z, no FIFO
    err = mma8450_write(c, MMA8450_XYZ_DATA_CFG, 0x07);
    if (err)
    return err;
//
// Sleep mode poll rate - 50Hz
// System output data rate - 400Hz
// Full scale selection - Active, +/- 2G
//
    err = mma8450_write(c, MMA8450_CTRL_REG1, 0x01);
    if (err)
    return err;
    msleep(MODE_CHANGE_DELAY_MS);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mma8450_close(input: *mut input_dev) {
    static void mma8450_close(struct input_dev *input)
    {
    struct i2c_client *c = input_get_drvdata(input);
    mma8450_write(c, MMA8450_CTRL_REG1, 0x00);
    mma8450_write(c, MMA8450_CTRL_REG2, 0x01);
    }
//
// I2C init/probing/exit functions
//
#[no_mangle]
unsafe extern "C" fn mma8450_probe(c: *mut i2c_client) -> c_int {
    static int mma8450_probe(struct i2c_client *c)
    {
    struct i2c_adapter *adapter = c.adapter;
    struct input_dev *input;
    int err, client_id;
    if (!i2c_check_functionality(adapter, I2C_FUNC_SMBUS_BYTE |
    I2C_FUNC_SMBUS_BYTE_DATA))
    return dev_err_probe(&c.dev, -EINVAL,
    "I2C adapter doesn't support SMBUS BYTE");
    client_id = i2c_smbus_read_byte_data(c, MMA8450_WHO_AM_I);
    if (client_id != MMA8450_ID)
    return dev_err_probe(&c.dev, -EINVAL,
    "unexpected chip ID 0x%x (vs 0x%x)\n",
    client_id, MMA8450_ID);
    input = devm_input_allocate_device(&c.dev);
    if (!input)
    return -ENOMEM;
    input_set_drvdata(input, c);
    input.name = MMA8450_DRV_NAME;
    input.id.bustype = BUS_I2C;
    input.open = mma8450_open;
    input.close = mma8450_close;
    input_set_abs_params(input, ABS_X, -2048, 2047, 32, 32);
    input_set_abs_params(input, ABS_Y, -2048, 2047, 32, 32);
    input_set_abs_params(input, ABS_Z, -2048, 2047, 32, 32);
    err = input_setup_polling(input, mma8450_poll);
    if (err) {
    dev_err(&c.dev, "failed to set up polling\n");
    return err;
    }
    input_set_poll_interval(input, POLL_INTERVAL);
    input_set_max_poll_interval(input, POLL_INTERVAL_MAX);
    err = input_register_device(input);
    if (err) {
    dev_err(&c.dev, "failed to register input device\n");
    return err;
    }
    return 0;
    }
    static const struct i2c_device_id mma8450_id[] = {
    { .name = MMA8450_DRV_NAME },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, mma8450_id);
    static const struct of_device_id mma8450_dt_ids[] = {
    { .compatible = "fsl,mma8450", },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, mma8450_dt_ids);
    static struct i2c_driver mma8450_driver = {
    .driver = {
    .name	= MMA8450_DRV_NAME,
    .of_match_table = mma8450_dt_ids,
    },
    .probe		= mma8450_probe,
    .id_table	= mma8450_id,
    };
    module_i2c_driver(mma8450_driver);
    MODULE_AUTHOR("Freescale Semiconductor, Inc.");
    MODULE_DESCRIPTION("MMA8450 3-Axis Accelerometer Driver");
    MODULE_LICENSE("GPL");
