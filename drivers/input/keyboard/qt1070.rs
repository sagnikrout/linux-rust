//! Automatically rewritten from C to Rust
//! Source: drivers/input/keyboard/qt1070.c
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
// Atmel AT42QT1070 QTouch Sensor Controller
//
// Copyright (C) 2011 Atmel
//
// Authors: Bo Shen <voice.shen@atmel.com>
//
// Base on AT42QT2160 driver by:
// Raphael Derosso Pereira <raphaelpereira@gmail.com>
// Copyright (C) 2009
//

// Address for each register
pub const CHIP_ID: c_uint = 0x00;
pub const QT1070_CHIP_ID: c_uint = 0x2E;
pub const FW_VERSION: c_uint = 0x01;
pub const QT1070_FW_VERSION: c_uint = 0x15;
pub const DET_STATUS: c_uint = 0x02;
pub const KEY_STATUS: c_uint = 0x03;
// Calibrate
pub const CALIBRATE_CMD: c_uint = 0x38;
pub const QT1070_CAL_TIME: c_int = 200;
// Reset
pub const RESET: c_uint = 0x39;
pub const QT1070_RESET_TIME: c_int = 255;
// AT42QT1070 support up to 7 keys
    static const unsigned short qt1070_key2code[] = {
    KEY_0, KEY_1, KEY_2, KEY_3,
    KEY_4, KEY_5, KEY_6,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qt1070_data {
    pub client: *mut i2c_client,
    pub input: *mut input_dev,
    pub irq: c_uint,
    pub keycodes: [c_ushort; ARRAY_SIZE(qt1070_key2code)],
    pub last_keys: u8,
}

#[no_mangle]
unsafe extern "C" fn qt1070_read(client: *mut i2c_client, reg: u8) -> c_int {
    static int qt1070_read(struct i2c_client *client, u8 reg)
    {
    int ret;
    ret = i2c_smbus_read_byte_data(client, reg);
    if (ret < 0)
    dev_err(&client.dev,
    "can not read register, returned %d\n", ret);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn qt1070_write(client: *mut i2c_client, reg: u8, data: u8) -> c_int {
    static int qt1070_write(struct i2c_client *client, u8 reg, u8 data)
    {
    int ret;
    ret = i2c_smbus_write_byte_data(client, reg, data);
    if (ret < 0)
    dev_err(&client.dev,
    "can not write register, returned %d\n", ret);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn qt1070_identify(client: *mut i2c_client) -> bool {
    static bool qt1070_identify(struct i2c_client *client)
    {
    int id, ver;
// Read Chip ID
    id = qt1070_read(client, CHIP_ID);
    if (id != QT1070_CHIP_ID) {
    dev_err(&client.dev, "ID %d not supported\n", id);
    return false;
    }
// Read firmware version
    ver = qt1070_read(client, FW_VERSION);
    if (ver < 0) {
    dev_err(&client.dev, "could not read the firmware version\n");
    return false;
    }
    dev_info(&client.dev, "AT42QT1070 firmware version %x\n", ver);
    return true;
    }
#[no_mangle]
unsafe extern "C" fn qt1070_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t qt1070_interrupt(int irq, void *dev_id)
    {
    struct qt1070_data *data = dev_id;
    struct i2c_client *client = data.client;
    struct input_dev *input = data.input;
    int i;
    u8 new_keys, keyval, mask = 0x01;
// Read the detected status register, thus clearing interrupt
    qt1070_read(client, DET_STATUS);
// Read which key changed
    new_keys = qt1070_read(client, KEY_STATUS);
    for (i = 0; i < ARRAY_SIZE(qt1070_key2code); i++) {
    keyval = new_keys & mask;
    if ((data.last_keys & mask) != keyval)
    input_report_key(input, data.keycodes[i], keyval);
    mask <<= 1;
    }
    input_sync(input);
    data.last_keys = new_keys;
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn qt1070_probe(client: *mut i2c_client) -> c_int {
    static int qt1070_probe(struct i2c_client *client)
    {
    struct qt1070_data *data;
    struct input_dev *input;
    int i;
    int err;
    if (!i2c_check_functionality(client.adapter, I2C_FUNC_SMBUS_BYTE)) {
    dev_err(&client.dev, "%s adapter not supported\n",
    dev_driver_string(&client.adapter.dev));
    return -ENODEV;
    }
    if (!client.irq) {
    dev_err(&client.dev, "please assign the irq to this device\n");
    return -EINVAL;
    }
// Identify the qt1070 chip
    if (!qt1070_identify(client))
    return -ENODEV;
    data = devm_kzalloc(&client.dev, sizeof(struct qt1070_data),
    GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    input = devm_input_allocate_device(&client.dev);
    if (!input)
    return -ENOMEM;
    data.client = client;
    data.input = input;
    data.irq = client.irq;
    input.name = "AT42QT1070 QTouch Sensor";
    input.id.bustype = BUS_I2C;
// Add the keycode
    input.keycode = data.keycodes;
    input.keycodesize = sizeof(data.keycodes[0]);
    input.keycodemax = ARRAY_SIZE(qt1070_key2code);
    __set_bit(EV_KEY, input.evbit);
    for (i = 0; i < ARRAY_SIZE(qt1070_key2code); i++) {
    data.keycodes[i] = qt1070_key2code[i];
    __set_bit(qt1070_key2code[i], input.keybit);
    }
// Calibrate device
    qt1070_write(client, CALIBRATE_CMD, 1);
    msleep(QT1070_CAL_TIME);
// Soft reset
    qt1070_write(client, RESET, 1);
    msleep(QT1070_RESET_TIME);
    err = devm_request_threaded_irq(&client.dev, client.irq,
    core::ptr::null_mut(), qt1070_interrupt,
    IRQF_TRIGGER_NONE | IRQF_ONESHOT,
    client.dev.driver.name, data);
    if (err) {
    dev_err(&client.dev, "fail to request irq\n");
    return err;
    }
// Register the input device
    err = input_register_device(data.input);
    if (err) {
    dev_err(&client.dev, "Failed to register input device\n");
    return err;
    }
    i2c_set_clientdata(client, data);
// Read to clear the chang line
    qt1070_read(client, DET_STATUS);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn qt1070_suspend(dev: *mut device) -> c_int {
    static int qt1070_suspend(struct device *dev)
    {
    struct i2c_client *client = to_i2c_client(dev);
    struct qt1070_data *data = i2c_get_clientdata(client);
    if (device_may_wakeup(dev))
    enable_irq_wake(data.irq);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn qt1070_resume(dev: *mut device) -> c_int {
    static int qt1070_resume(struct device *dev)
    {
    struct i2c_client *client = to_i2c_client(dev);
    struct qt1070_data *data = i2c_get_clientdata(client);
    if (device_may_wakeup(dev))
    disable_irq_wake(data.irq);
    return 0;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(qt1070_pm_ops, qt1070_suspend, qt1070_resume);
    static const struct i2c_device_id qt1070_id[] = {
    { .name = "qt1070" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, qt1070_id);

    static const struct of_device_id qt1070_of_match[] = {
    { .compatible = "qt1070", },
    { },
    };
    MODULE_DEVICE_TABLE(of, qt1070_of_match);

    static struct i2c_driver qt1070_driver = {
    .driver	= {
    .name	= "qt1070",
    .of_match_table = of_match_ptr(qt1070_of_match),
    .pm	= pm_sleep_ptr(&qt1070_pm_ops),
    },
    .id_table	= qt1070_id,
    .probe		= qt1070_probe,
    };
    module_i2c_driver(qt1070_driver);
    MODULE_AUTHOR("Bo Shen <voice.shen@atmel.com>");
    MODULE_DESCRIPTION("Driver for AT42QT1070 QTouch sensor");
    MODULE_LICENSE("GPL");
