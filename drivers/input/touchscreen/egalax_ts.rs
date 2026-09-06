//! Automatically rewritten from C to Rust
//! Source: drivers/input/touchscreen/egalax_ts.c
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
// Driver for EETI eGalax Multiple Touch Controller
//
// Copyright (C) 2011 Freescale Semiconductor, Inc.
//
// based on max11801_ts.c
//
// EETI eGalax serial touch screen controller is a I2C based multiple
// touch screen controller, it supports 5 point multiple touch.
// TODO:
    - auto idle mode support
//

//
// Mouse Mode: some panel may configure the controller to mouse mode,
// which can only report one point at a given time.
// This driver will ignore events in this mode.
//
pub const REPORT_MODE_MOUSE: c_uint = 0x1;
//
// Vendor Mode: this mode is used to transfer some vendor specific
// messages.
// This driver will ignore events in this mode.
//
pub const REPORT_MODE_VENDOR: c_uint = 0x3;
// Multiple Touch Mode
pub const REPORT_MODE_MTTOUCH: c_uint = 0x4;
pub const MAX_SUPPORT_POINTS: c_int = 5;
pub const EVENT_VALID_OFFSET: c_int = 7;

pub const EVENT_ID_OFFSET: c_int = 2;

pub const MAX_I2C_DATA_LEN: c_int = 10;
pub const EGALAX_MAX_X: c_int = 32760;
pub const EGALAX_MAX_Y: c_int = 32760;
pub const EGALAX_MAX_TRIES: c_int = 100;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct egalax_ts {
    pub client: *mut i2c_client,
    pub input_dev: *mut input_dev,
}

#[no_mangle]
unsafe extern "C" fn egalax_ts_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t egalax_ts_interrupt(int irq, void *dev_id)
    {
    struct egalax_ts *ts = dev_id;
    struct input_dev *input_dev = ts.input_dev;
    struct i2c_client *client = ts.client;
    u8 buf[MAX_I2C_DATA_LEN];
    int id, ret, x, y, z;
    let mut tries: c_int = 0;
    bool down, valid;
    u8 state;
    do {
    ret = i2c_master_recv(client, buf, MAX_I2C_DATA_LEN);
    } while (ret == -EAGAIN && tries++ < EGALAX_MAX_TRIES);
    if (ret < 0)
    return IRQ_HANDLED;
    if (buf[0] != REPORT_MODE_MTTOUCH) {
// ignore mouse events and vendor events
    return IRQ_HANDLED;
    }
    state = buf[1];
    x = (buf[3] << 8) | buf[2];
    y = (buf[5] << 8) | buf[4];
    z = (buf[7] << 8) | buf[6];
    valid = state & EVENT_VALID_MASK;
    id = (state & EVENT_ID_MASK) >> EVENT_ID_OFFSET;
    down = state & EVENT_DOWN_UP;
    if (!valid || id > MAX_SUPPORT_POINTS) {
    dev_dbg(&client.dev, "point invalid\n");
    return IRQ_HANDLED;
    }
    input_mt_slot(input_dev, id);
    input_mt_report_slot_state(input_dev, MT_TOOL_FINGER, down);
    dev_dbg(&client.dev, "%s id:%d x:%d y:%d z:%d",
    str_down_up(down), id, x, y, z);
    if (down) {
    input_report_abs(input_dev, ABS_MT_POSITION_X, x);
    input_report_abs(input_dev, ABS_MT_POSITION_Y, y);
    input_report_abs(input_dev, ABS_MT_PRESSURE, z);
    }
    input_mt_report_pointer_emulation(input_dev, true);
    input_sync(input_dev);
    return IRQ_HANDLED;
    }
// wake up controller by an falling edge of interrupt gpio.
#[no_mangle]
unsafe extern "C" fn egalax_wake_up_device(client: *mut i2c_client) -> c_int {
    static int egalax_wake_up_device(struct i2c_client *client)
    {
    struct gpio_desc *gpio;
    int ret;
// wake up controller via an falling edge on IRQ gpio.
    gpio = gpiod_get(&client.dev, "wakeup", GPIOD_OUT_HIGH);
    ret = PTR_ERR_OR_ZERO(gpio);
    if (ret) {
    if (ret != -EPROBE_DEFER)
    dev_err(&client.dev,
    "failed to request wakeup gpio, cannot wake up controller: %d\n",
    ret);
    return ret;
    }
// release the line
    gpiod_set_value_cansleep(gpio, 0);
// controller should be woken up, return irq.
    gpiod_direction_input(gpio);
    gpiod_put(gpio);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn egalax_firmware_version(client: *mut i2c_client) -> c_int {
    static int egalax_firmware_version(struct i2c_client *client)
    {
    static const u8 cmd[MAX_I2C_DATA_LEN] = { 0x03, 0x03, 0xa, 0x01, 0x41 };
    int ret;
    ret = i2c_master_send(client, cmd, MAX_I2C_DATA_LEN);
    if (ret < 0)
    return ret;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn egalax_ts_probe(client: *mut i2c_client) -> c_int {
    static int egalax_ts_probe(struct i2c_client *client)
    {
    struct egalax_ts *ts;
    struct input_dev *input_dev;
    int error;
    ts = devm_kzalloc(&client.dev, sizeof(struct egalax_ts), GFP_KERNEL);
    if (!ts) {
    dev_err(&client.dev, "Failed to allocate memory\n");
    return -ENOMEM;
    }
    input_dev = devm_input_allocate_device(&client.dev);
    if (!input_dev) {
    dev_err(&client.dev, "Failed to allocate memory\n");
    return -ENOMEM;
    }
    ts.client = client;
    ts.input_dev = input_dev;
// controller may be in sleep, wake it up.
    error = egalax_wake_up_device(client);
    if (error)
    return error;
    error = egalax_firmware_version(client);
    if (error < 0) {
    dev_err(&client.dev, "Failed to read firmware version\n");
    return error;
    }
    input_dev.name = "EETI eGalax Touch Screen";
    input_dev.id.bustype = BUS_I2C;
    __set_bit(EV_ABS, input_dev.evbit);
    __set_bit(EV_KEY, input_dev.evbit);
    __set_bit(BTN_TOUCH, input_dev.keybit);
    input_set_abs_params(input_dev, ABS_X, 0, EGALAX_MAX_X, 0, 0);
    input_set_abs_params(input_dev, ABS_Y, 0, EGALAX_MAX_Y, 0, 0);
    input_set_abs_params(input_dev,
    ABS_MT_POSITION_X, 0, EGALAX_MAX_X, 0, 0);
    input_set_abs_params(input_dev,
    ABS_MT_POSITION_Y, 0, EGALAX_MAX_Y, 0, 0);
    input_mt_init_slots(input_dev, MAX_SUPPORT_POINTS, 0);
    error = devm_request_threaded_irq(&client.dev, client.irq,
    core::ptr::null_mut(), egalax_ts_interrupt,
    IRQF_ONESHOT, "egalax_ts", ts);
    if (error < 0) {
    dev_err(&client.dev, "Failed to register interrupt\n");
    return error;
    }
    error = input_register_device(ts.input_dev);
    if (error)
    return error;
    return 0;
    }
    static const struct i2c_device_id egalax_ts_id[] = {
    { .name = "egalax_ts" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, egalax_ts_id);
#[no_mangle]
unsafe extern "C" fn egalax_ts_suspend(dev: *mut device) -> c_int {
    static int egalax_ts_suspend(struct device *dev)
    {
    static const u8 suspend_cmd[MAX_I2C_DATA_LEN] = {
    0x3, 0x6, 0xa, 0x3, 0x36, 0x3f, 0x2, 0, 0, 0
    };
    struct i2c_client *client = to_i2c_client(dev);
    int ret;
    if (device_may_wakeup(dev))
    return enable_irq_wake(client.irq);
    ret = i2c_master_send(client, suspend_cmd, MAX_I2C_DATA_LEN);
    return ret > 0 ? 0 : ret;
    }
#[no_mangle]
unsafe extern "C" fn egalax_ts_resume(dev: *mut device) -> c_int {
    static int egalax_ts_resume(struct device *dev)
    {
    struct i2c_client *client = to_i2c_client(dev);
    if (device_may_wakeup(dev))
    return disable_irq_wake(client.irq);
    return egalax_wake_up_device(client);
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(egalax_ts_pm_ops,
    egalax_ts_suspend, egalax_ts_resume);
    static const struct of_device_id egalax_ts_dt_ids[] = {
    { .compatible = "eeti,egalax_ts" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, egalax_ts_dt_ids);
    static struct i2c_driver egalax_ts_driver = {
    .driver = {
    .name	= "egalax_ts",
    .pm	= pm_sleep_ptr(&egalax_ts_pm_ops),
    .of_match_table	= egalax_ts_dt_ids,
    },
    .id_table	= egalax_ts_id,
    .probe		= egalax_ts_probe,
    };
    module_i2c_driver(egalax_ts_driver);
    MODULE_AUTHOR("Freescale Semiconductor, Inc.");
    MODULE_DESCRIPTION("Touchscreen driver for EETI eGalax touch controller");
    MODULE_LICENSE("GPL");
