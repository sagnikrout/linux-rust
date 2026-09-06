//! Automatically rewritten from C to Rust
//! Source: drivers/input/touchscreen/ar1021_i2c.c
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
// Microchip AR1020 and AR1021 driver for I2C
//
// Author: Christian Gmeiner <christian.gmeiner@gmail.com>
//

pub const AR1021_TOUCH_PKG_SIZE: c_int = 5;
pub const AR1021_MAX_X: c_int = 4095;
pub const AR1021_MAX_Y: c_int = 4095;
pub const AR1021_CMD: c_uint = 0x55;
pub const AR1021_CMD_ENABLE_TOUCH: c_uint = 0x12;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ar1021_i2c {
    pub client: *mut i2c_client,
    pub input: *mut input_dev,
    pub data: [u8; AR1021_TOUCH_PKG_SIZE],
}

#[no_mangle]
unsafe extern "C" fn ar1021_i2c_irq(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t ar1021_i2c_irq(int irq, void *dev_id)
    {
    struct ar1021_i2c *ar1021 = dev_id;
    struct input_dev *input = ar1021.input;
    u8 *data = ar1021.data;
    unsigned int x, y, button;
    int retval;
    retval = i2c_master_recv(ar1021.client,
    ar1021.data, sizeof(ar1021.data));
    if (retval != sizeof(ar1021.data))
    goto out;
// sync bit set ?
    if (!(data[0] & BIT(7)))
    goto out;
    button = data[0] & BIT(0);
    x = ((data[2] & 0x1f) << 7) | (data[1] & 0x7f);
    y = ((data[4] & 0x1f) << 7) | (data[3] & 0x7f);
    input_report_abs(input, ABS_X, x);
    input_report_abs(input, ABS_Y, y);
    input_report_key(input, BTN_TOUCH, button);
    input_sync(input);
    out:
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn ar1021_i2c_open(dev: *mut input_dev) -> c_int {
    static int ar1021_i2c_open(struct input_dev *dev)
    {
    static const u8 cmd_enable_touch[] = {
    AR1021_CMD,
    0x01, /* number of bytes after this */
    AR1021_CMD_ENABLE_TOUCH
    };
    struct ar1021_i2c *ar1021 = input_get_drvdata(dev);
    struct i2c_client *client = ar1021.client;
    int error;
    error = i2c_master_send(ar1021.client, cmd_enable_touch,
    sizeof(cmd_enable_touch));
    if (error < 0)
    return error;
    enable_irq(client.irq);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ar1021_i2c_close(dev: *mut input_dev) {
    static void ar1021_i2c_close(struct input_dev *dev)
    {
    struct ar1021_i2c *ar1021 = input_get_drvdata(dev);
    struct i2c_client *client = ar1021.client;
    disable_irq(client.irq);
    }
#[no_mangle]
unsafe extern "C" fn ar1021_i2c_probe(client: *mut i2c_client) -> c_int {
    static int ar1021_i2c_probe(struct i2c_client *client)
    {
    struct ar1021_i2c *ar1021;
    struct input_dev *input;
    int error;
    if (!i2c_check_functionality(client.adapter, I2C_FUNC_I2C)) {
    dev_err(&client.dev, "i2c_check_functionality error\n");
    return -ENXIO;
    }
    ar1021 = devm_kzalloc(&client.dev, sizeof(*ar1021), GFP_KERNEL);
    if (!ar1021)
    return -ENOMEM;
    input = devm_input_allocate_device(&client.dev);
    if (!input)
    return -ENOMEM;
    ar1021.client = client;
    ar1021.input = input;
    input.name = "ar1021 I2C Touchscreen";
    input.id.bustype = BUS_I2C;
    input.dev.parent = &client.dev;
    input.open = ar1021_i2c_open;
    input.close = ar1021_i2c_close;
    __set_bit(INPUT_PROP_DIRECT, input.propbit);
    input_set_capability(input, EV_KEY, BTN_TOUCH);
    input_set_abs_params(input, ABS_X, 0, AR1021_MAX_X, 0, 0);
    input_set_abs_params(input, ABS_Y, 0, AR1021_MAX_Y, 0, 0);
    input_set_drvdata(input, ar1021);
    error = devm_request_threaded_irq(&client.dev, client.irq,
    core::ptr::null_mut(), ar1021_i2c_irq,
    IRQF_ONESHOT | IRQF_NO_AUTOEN,
    "ar1021_i2c", ar1021);
    if (error) {
    dev_err(&client.dev,
    "Failed to enable IRQ, error: %d\n", error);
    return error;
    }
    error = input_register_device(ar1021.input);
    if (error) {
    dev_err(&client.dev,
    "Failed to register input device, error: %d\n", error);
    return error;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ar1021_i2c_suspend(dev: *mut device) -> c_int {
    static int ar1021_i2c_suspend(struct device *dev)
    {
    struct i2c_client *client = to_i2c_client(dev);
    disable_irq(client.irq);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ar1021_i2c_resume(dev: *mut device) -> c_int {
    static int ar1021_i2c_resume(struct device *dev)
    {
    struct i2c_client *client = to_i2c_client(dev);
    enable_irq(client.irq);
    return 0;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(ar1021_i2c_pm,
    ar1021_i2c_suspend, ar1021_i2c_resume);
    static const struct i2c_device_id ar1021_i2c_id[] = {
    { .name = "ar1021" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, ar1021_i2c_id);
    static const struct of_device_id ar1021_i2c_of_match[] = {
    { .compatible = "microchip,ar1021-i2c", },
    { }
    };
    MODULE_DEVICE_TABLE(of, ar1021_i2c_of_match);
    static struct i2c_driver ar1021_i2c_driver = {
    .driver	= {
    .name	= "ar1021_i2c",
    .pm	= pm_sleep_ptr(&ar1021_i2c_pm),
    .of_match_table = ar1021_i2c_of_match,
    },
    .probe		= ar1021_i2c_probe,
    .id_table	= ar1021_i2c_id,
    };
    module_i2c_driver(ar1021_i2c_driver);
    MODULE_AUTHOR("Christian Gmeiner <christian.gmeiner@gmail.com>");
    MODULE_DESCRIPTION("Microchip AR1020 and AR1021 I2C Driver");
    MODULE_LICENSE("GPL");
