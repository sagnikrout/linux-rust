//! Automatically rewritten from C to Rust
//! Source: drivers/input/touchscreen/wacom_i2c.c
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
// Wacom Penabled Driver for I2C
//
// Copyright (c) 2011 - 2013 Tatsunosuke Tobita, Wacom.
// <tobita.tatsunosuke@wacom.co.jp>
//

// Bitmasks (for data[3])

// Registers
pub const WACOM_COMMAND_LSB: c_uint = 0x04;
pub const WACOM_COMMAND_MSB: c_uint = 0x00;
pub const WACOM_DATA_LSB: c_uint = 0x05;
pub const WACOM_DATA_MSB: c_uint = 0x00;
// Report types
pub const REPORT_FEATURE: c_uint = 0x30;
// Requests / operations
pub const OPCODE_GET_REPORT: c_uint = 0x02;
pub const WACOM_QUERY_REPORT: c_int = 3;
pub const WACOM_QUERY_SIZE: c_int = 19;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wacom_features {
    pub x_max: c_int,
    pub y_max: c_int,
    pub pressure_max: c_int,
    pub fw_version: c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wacom_i2c {
    pub client: *mut i2c_client,
    pub input: *mut input_dev,
    pub data: [u8; WACOM_QUERY_SIZE],
    pub prox: bool,
    pub tool: c_int,
}

    static int wacom_query_device(struct i2c_client *client,
    struct wacom_features *features)
    {
    u8 get_query_data_cmd[] = {
    WACOM_COMMAND_LSB,
    WACOM_COMMAND_MSB,
    REPORT_FEATURE | WACOM_QUERY_REPORT,
    OPCODE_GET_REPORT,
    WACOM_DATA_LSB,
    WACOM_DATA_MSB,
    };
    u8 data[WACOM_QUERY_SIZE];
    int ret;
    struct i2c_msg msgs[] = {
// Request reading of feature ReportID: 3 (Pen Query Data)
    {
    .addr = client.addr,
    .flags = 0,
    .len = sizeof(get_query_data_cmd),
    .buf = get_query_data_cmd,
    },
    {
    .addr = client.addr,
    .flags = I2C_M_RD,
    .len = sizeof(data),
    .buf = data,
    },
    };
    ret = i2c_transfer(client.adapter, msgs, ARRAY_SIZE(msgs));
    if (ret < 0)
    return ret;
    if (ret != ARRAY_SIZE(msgs))
    return -EIO;
    features.x_max = get_unaligned_le16(&data[3]);
    features.y_max = get_unaligned_le16(&data[5]);
    features.pressure_max = get_unaligned_le16(&data[11]);
    features.fw_version = get_unaligned_le16(&data[13]);
    dev_dbg(&client.dev,
    "x_max:%d, y_max:%d, pressure:%d, fw:%d\n",
    features.x_max, features.y_max,
    features.pressure_max, features.fw_version);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn wacom_i2c_irq(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t wacom_i2c_irq(int irq, void *dev_id)
    {
    struct wacom_i2c *wac_i2c = dev_id;
    struct input_dev *input = wac_i2c.input;
    u8 *data = wac_i2c.data;
    unsigned int x, y, pressure;
    unsigned char tsw, f1, f2, ers;
    int error;
    error = i2c_master_recv(wac_i2c.client,
    wac_i2c.data, sizeof(wac_i2c.data));
    if (error < 0)
    goto out;
    tsw = data[3] & WACOM_TIP_SWITCH;
    ers = data[3] & WACOM_ERASER;
    f1 = data[3] & WACOM_BARREL_SWITCH;
    f2 = data[3] & WACOM_BARREL_SWITCH_2;
    x = le16_to_cpup((__le16 *)&data[4]);
    y = le16_to_cpup((__le16 *)&data[6]);
    pressure = le16_to_cpup((__le16 *)&data[8]);
    if (!wac_i2c.prox)
    wac_i2c.tool = (data[3] & (WACOM_ERASER | WACOM_INVERT)) ?
    BTN_TOOL_RUBBER : BTN_TOOL_PEN;
    wac_i2c.prox = data[3] & WACOM_IN_PROXIMITY;
    input_report_key(input, BTN_TOUCH, tsw || ers);
    input_report_key(input, wac_i2c.tool, wac_i2c.prox);
    input_report_key(input, BTN_STYLUS, f1);
    input_report_key(input, BTN_STYLUS2, f2);
    input_report_abs(input, ABS_X, x);
    input_report_abs(input, ABS_Y, y);
    input_report_abs(input, ABS_PRESSURE, pressure);
    input_sync(input);
    out:
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn wacom_i2c_open(dev: *mut input_dev) -> c_int {
    static int wacom_i2c_open(struct input_dev *dev)
    {
    struct wacom_i2c *wac_i2c = input_get_drvdata(dev);
    struct i2c_client *client = wac_i2c.client;
    enable_irq(client.irq);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn wacom_i2c_close(dev: *mut input_dev) {
    static void wacom_i2c_close(struct input_dev *dev)
    {
    struct wacom_i2c *wac_i2c = input_get_drvdata(dev);
    struct i2c_client *client = wac_i2c.client;
    disable_irq(client.irq);
    }
#[no_mangle]
unsafe extern "C" fn wacom_i2c_probe(client: *mut i2c_client) -> c_int {
    static int wacom_i2c_probe(struct i2c_client *client)
    {
    struct device *dev = &client.dev;
    struct wacom_i2c *wac_i2c;
    struct input_dev *input;
    let mut features: wacom_features = { 0 };
    int error;
    if (!i2c_check_functionality(client.adapter, I2C_FUNC_I2C)) {
    dev_err(dev, "i2c_check_functionality error\n");
    return -EIO;
    }
    error = wacom_query_device(client, &features);
    if (error)
    return error;
    wac_i2c = devm_kzalloc(dev, sizeof(*wac_i2c), GFP_KERNEL);
    if (!wac_i2c)
    return -ENOMEM;
    wac_i2c.client = client;
    input = devm_input_allocate_device(dev);
    if (!input)
    return -ENOMEM;
    wac_i2c.input = input;
    input.name = "Wacom I2C Digitizer";
    input.id.bustype = BUS_I2C;
    input.id.vendor = 0x56a;
    input.id.version = features.fw_version;
    input.open = wacom_i2c_open;
    input.close = wacom_i2c_close;
    input.evbit[0] |= BIT_MASK(EV_KEY) | BIT_MASK(EV_ABS);
    __set_bit(BTN_TOOL_PEN, input.keybit);
    __set_bit(BTN_TOOL_RUBBER, input.keybit);
    __set_bit(BTN_STYLUS, input.keybit);
    __set_bit(BTN_STYLUS2, input.keybit);
    __set_bit(BTN_TOUCH, input.keybit);
    input_set_abs_params(input, ABS_X, 0, features.x_max, 0, 0);
    input_set_abs_params(input, ABS_Y, 0, features.y_max, 0, 0);
    input_set_abs_params(input, ABS_PRESSURE,
    0, features.pressure_max, 0, 0);
    input_set_drvdata(input, wac_i2c);
    error = devm_request_threaded_irq(dev, client.irq, core::ptr::null_mut(), wacom_i2c_irq,
    IRQF_ONESHOT, "wacom_i2c", wac_i2c);
    if (error) {
    dev_err(dev, "Failed to request IRQ: %d\n", error);
    return error;
    }
// Disable the IRQ, we'll enable it in wac_i2c_open()
    disable_irq(client.irq);
    error = input_register_device(wac_i2c.input);
    if (error) {
    dev_err(dev, "Failed to register input device: %d\n", error);
    return error;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn wacom_i2c_suspend(dev: *mut device) -> c_int {
    static int wacom_i2c_suspend(struct device *dev)
    {
    struct i2c_client *client = to_i2c_client(dev);
    disable_irq(client.irq);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn wacom_i2c_resume(dev: *mut device) -> c_int {
    static int wacom_i2c_resume(struct device *dev)
    {
    struct i2c_client *client = to_i2c_client(dev);
    enable_irq(client.irq);
    return 0;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(wacom_i2c_pm, wacom_i2c_suspend, wacom_i2c_resume);
    static const struct i2c_device_id wacom_i2c_id[] = {
    { .name = "WAC_I2C_EMR" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, wacom_i2c_id);
    static struct i2c_driver wacom_i2c_driver = {
    .driver	= {
    .name	= "wacom_i2c",
    .pm	= pm_sleep_ptr(&wacom_i2c_pm),
    },
    .probe		= wacom_i2c_probe,
    .id_table	= wacom_i2c_id,
    };
    module_i2c_driver(wacom_i2c_driver);
    MODULE_AUTHOR("Tatsunosuke Tobita <tobita.tatsunosuke@wacom.co.jp>");
    MODULE_DESCRIPTION("WACOM EMR I2C Driver");
    MODULE_LICENSE("GPL");
