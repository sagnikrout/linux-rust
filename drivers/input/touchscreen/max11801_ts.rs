//! Automatically rewritten from C to Rust
//! Source: drivers/input/touchscreen/max11801_ts.c
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
// Driver for MAXI MAX11801 - A Resistive touch screen controller with
// i2c interface
//
// Copyright (C) 2011 Freescale Semiconductor, Inc.
// Author: Zhang Jiejing <jiejing.zhang@freescale.com>
//
// Based on mcs5000_ts.c
//
// This driver aims to support the series of MAXI touch chips max11801
// through max11803. The main difference between these 4 chips can be
// found in the table below:
// -----------------------------------------------------
// | CHIP     |  AUTO MODE SUPPORT(FIFO) | INTERFACE    |
// |----------------------------------------------------|
// | max11800 |  YES                     |   SPI        |
// | max11801 |  YES                     |   I2C        |
// | max11802 |  NO                      |   SPI        |
// | max11803 |  NO                      |   I2C        |
// ------------------------------------------------------
//
// Currently, this driver only supports max11801.
//
// Data Sheet:
// http://www.maxim-ic.com/datasheet/index.mvp/id/5943
//

// Register Address define
pub const GENERNAL_STATUS_REG: c_uint = 0x00;
pub const GENERNAL_CONF_REG: c_uint = 0x01;
pub const MESURE_RES_CONF_REG: c_uint = 0x02;
pub const MESURE_AVER_CONF_REG: c_uint = 0x03;
pub const ADC_SAMPLE_TIME_CONF_REG: c_uint = 0x04;
pub const PANEL_SETUPTIME_CONF_REG: c_uint = 0x05;
pub const DELAY_CONVERSION_CONF_REG: c_uint = 0x06;
pub const TOUCH_DETECT_PULLUP_CONF_REG: c_uint = 0x07;
pub const AUTO_MODE_TIME_CONF_REG: c_uint = 0x08 /* only for max11800/max11801 */;
pub const APERTURE_CONF_REG: c_uint = 0x09 /* only for max11800/max11801 */;
pub const AUX_MESURE_CONF_REG: c_uint = 0x0a;
pub const OP_MODE_CONF_REG: c_uint = 0x0b;
// FIFO is found only in max11800 and max11801

pub const XY_BUFSIZE: c_int = 4;
pub const XY_BUF_OFFSET: c_int = 4;
pub const MAX11801_MAX_X: c_uint = 0xfff;
pub const MAX11801_MAX_Y: c_uint = 0xfff;
pub const MEASURE_TAG_OFFSET: c_int = 2;

pub const EVENT_TAG_OFFSET: c_int = 0;

// These are the state of touch event state machine
    enum {
    EVENT_INIT,
    EVENT_MIDDLE,
    EVENT_RELEASE,
    EVENT_FIFO_END
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct max11801_data {
    pub client: *mut i2c_client,
    pub input_dev: *mut input_dev,
}

#[no_mangle]
unsafe extern "C" fn read_register(client: *mut i2c_client, addr: c_int) -> u8 {
    static u8 read_register(struct i2c_client *client, int addr)
    {
// XXX: The chip ignores LSB of register address
    return i2c_smbus_read_byte_data(client, addr << 1);
    }
#[no_mangle]
unsafe extern "C" fn max11801_write_reg(client: *mut i2c_client, addr: c_int, data: c_int) -> c_int {
    static int max11801_write_reg(struct i2c_client *client, int addr, int data)
    {
// XXX: The chip ignores LSB of register address
    return i2c_smbus_write_byte_data(client, addr << 1, data);
    }
#[no_mangle]
unsafe extern "C" fn max11801_ts_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t max11801_ts_interrupt(int irq, void *dev_id)
    {
    struct max11801_data *data = dev_id;
    struct i2c_client *client = data.client;
    int status, i, ret;
    u8 buf[XY_BUFSIZE];
    let mut x: c_int = -1;
    let mut y: c_int = -1;
    status = read_register(data.client, GENERNAL_STATUS_REG);
    if (status & (MAX11801_FIFO_INT | MAX11801_FIFO_OVERFLOW)) {
    status = read_register(data.client, GENERNAL_STATUS_REG);
    ret = i2c_smbus_read_i2c_block_data(client, FIFO_RD_CMD,
    XY_BUFSIZE, buf);
//
// We should get 4 bytes buffer that contains X,Y
// and event tag
//
    if (ret < XY_BUFSIZE)
    goto out;
    for (i = 0; i < XY_BUFSIZE; i += XY_BUFSIZE / 2) {
    if ((buf[i + 1] & MEASURE_TAG_MASK) == MEASURE_X_TAG)
    x = (buf[i] << XY_BUF_OFFSET) +
    (buf[i + 1] >> XY_BUF_OFFSET);
#[no_mangle]
pub unsafe extern "C" fn if(MEASURE_Y_TAG: (buf[i + 1] & MEASURE_TAG_MASK) ==) -> else {
    else if ((buf[i + 1] & MEASURE_TAG_MASK) == MEASURE_Y_TAG)
    y = (buf[i] << XY_BUF_OFFSET) +
    (buf[i + 1] >> XY_BUF_OFFSET);
    }
    if ((buf[1] & EVENT_TAG_MASK) != (buf[3] & EVENT_TAG_MASK))
    goto out;
    switch (buf[1] & EVENT_TAG_MASK) {
    case EVENT_INIT:
    case EVENT_MIDDLE:
    input_report_abs(data.input_dev, ABS_X, x);
    input_report_abs(data.input_dev, ABS_Y, y);
    input_event(data.input_dev, EV_KEY, BTN_TOUCH, 1);
    input_sync(data.input_dev);
    break;
    case EVENT_RELEASE:
    input_event(data.input_dev, EV_KEY, BTN_TOUCH, 0);
    input_sync(data.input_dev);
    break;
    case EVENT_FIFO_END:
    break;
    }
    }
    out:
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn max11801_ts_phy_init(data: *mut max11801_data) {
    static void max11801_ts_phy_init(struct max11801_data *data)
    {
    struct i2c_client *client = data.client;
// Average X,Y, take 16 samples, average eight media sample
    max11801_write_reg(client, MESURE_AVER_CONF_REG, 0xff);
// X,Y panel setup time set to 20us
    max11801_write_reg(client, PANEL_SETUPTIME_CONF_REG, 0x11);
// Rough pullup time (2uS), Fine pullup time (10us)
    max11801_write_reg(client, TOUCH_DETECT_PULLUP_CONF_REG, 0x10);
// Auto mode init period = 5ms , scan period = 5ms
    max11801_write_reg(client, AUTO_MODE_TIME_CONF_REG, 0xaa);
// Aperture X,Y set to +- 4LSB
    max11801_write_reg(client, APERTURE_CONF_REG, 0x33);
// Enable Power, enable Automode, enable Aperture, enable Average X,Y
    max11801_write_reg(client, OP_MODE_CONF_REG, 0x36);
    }
#[no_mangle]
unsafe extern "C" fn max11801_ts_probe(client: *mut i2c_client) -> c_int {
    static int max11801_ts_probe(struct i2c_client *client)
    {
    struct max11801_data *data;
    struct input_dev *input_dev;
    int error;
    data = devm_kzalloc(&client.dev, sizeof(*data), GFP_KERNEL);
    input_dev = devm_input_allocate_device(&client.dev);
    if (!data || !input_dev) {
    dev_err(&client.dev, "Failed to allocate memory\n");
    return -ENOMEM;
    }
    data.client = client;
    data.input_dev = input_dev;
    input_dev.name = "max11801_ts";
    input_dev.id.bustype = BUS_I2C;
    input_dev.dev.parent = &client.dev;
    __set_bit(EV_ABS, input_dev.evbit);
    __set_bit(EV_KEY, input_dev.evbit);
    __set_bit(BTN_TOUCH, input_dev.keybit);
    input_set_abs_params(input_dev, ABS_X, 0, MAX11801_MAX_X, 0, 0);
    input_set_abs_params(input_dev, ABS_Y, 0, MAX11801_MAX_Y, 0, 0);
    max11801_ts_phy_init(data);
    error = devm_request_threaded_irq(&client.dev, client.irq, core::ptr::null_mut(),
    max11801_ts_interrupt,
    IRQF_TRIGGER_LOW | IRQF_ONESHOT,
    "max11801_ts", data);
    if (error) {
    dev_err(&client.dev, "Failed to register interrupt\n");
    return error;
    }
    error = input_register_device(data.input_dev);
    if (error)
    return error;
    return 0;
    }
    static const struct i2c_device_id max11801_ts_id[] = {
    { .name = "max11801" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, max11801_ts_id);
    static const struct of_device_id max11801_ts_dt_ids[] = {
    { .compatible = "maxim,max11801" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, max11801_ts_dt_ids);
    static struct i2c_driver max11801_ts_driver = {
    .driver = {
    .name	= "max11801_ts",
    .of_match_table = max11801_ts_dt_ids,
    },
    .id_table	= max11801_ts_id,
    .probe		= max11801_ts_probe,
    };
    module_i2c_driver(max11801_ts_driver);
    MODULE_AUTHOR("Zhang Jiejing <jiejing.zhang@freescale.com>");
    MODULE_DESCRIPTION("Touchscreen driver for MAXI MAX11801 controller");
    MODULE_LICENSE("GPL");
