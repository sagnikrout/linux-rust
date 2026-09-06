//! Automatically rewritten from C to Rust
//! Source: drivers/input/misc/atmel_captouch.c
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
// Atmel Atmegaxx Capacitive Touch Button Driver
//
// Copyright (C) 2016 Google, inc.
//
// It's irrelevant that the HW used to develop captouch driver is based
// on Atmega88PA part and uses QtouchADC parts for sensing touch.
// Calling this driver "captouch" is an arbitrary way to distinguish
// the protocol this driver supported by other atmel/qtouch drivers.
//
// Captouch driver supports a newer/different version of the I2C
// registers/commands than the qt1070.c driver.
// Don't let the similarity of the general driver structure fool you.
//
// For raw i2c access from userspace, use i2cset/i2cget
// to poke at /dev/i2c-N devices.
//

// Maximum number of buttons supported
pub const MAX_NUM_OF_BUTTONS: c_int = 8;
// Registers
pub const REG_KEY1_THRESHOLD: c_uint = 0x02;
pub const REG_KEY2_THRESHOLD: c_uint = 0x03;
pub const REG_KEY3_THRESHOLD: c_uint = 0x04;
pub const REG_KEY4_THRESHOLD: c_uint = 0x05;
pub const REG_KEY1_REF_H: c_uint = 0x20;
pub const REG_KEY1_REF_L: c_uint = 0x21;
pub const REG_KEY2_REF_H: c_uint = 0x22;
pub const REG_KEY2_REF_L: c_uint = 0x23;
pub const REG_KEY3_REF_H: c_uint = 0x24;
pub const REG_KEY3_REF_L: c_uint = 0x25;
pub const REG_KEY4_REF_H: c_uint = 0x26;
pub const REG_KEY4_REF_L: c_uint = 0x27;
pub const REG_KEY1_DLT_H: c_uint = 0x30;
pub const REG_KEY1_DLT_L: c_uint = 0x31;
pub const REG_KEY2_DLT_H: c_uint = 0x32;
pub const REG_KEY2_DLT_L: c_uint = 0x33;
pub const REG_KEY3_DLT_H: c_uint = 0x34;
pub const REG_KEY3_DLT_L: c_uint = 0x35;
pub const REG_KEY4_DLT_H: c_uint = 0x36;
pub const REG_KEY4_DLT_L: c_uint = 0x37;
pub const REG_KEY_STATE: c_uint = 0x3C;
//
// @i2c_client: I2C slave device client pointer
// @input: Input device pointer
// @num_btn: Number of buttons
// @keycodes: map of button# to KeyCode
// @prev_btn: Previous key state to detect button "press" or "release"
// @xfer_buf: I2C transfer buffer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atmel_captouch_device {
    pub client: *mut i2c_client,
    pub input: *mut input_dev,
    pub num_btn: u32,
    pub keycodes: [u32; MAX_NUM_OF_BUTTONS],
    pub prev_btn: u8,
    pub ____cacheline_aligned: u8 xfer_buf[8],
}

//
// Read from I2C slave device
// The protocol is that the client has to provide both the register address
// and the length, and while reading back the device would prepend the data
// with address and length for verification.
//
    static int atmel_read(struct atmel_captouch_device *capdev,
    u8 reg, u8 *data, size_t len)
    {
    struct i2c_client *client = capdev.client;
    struct device *dev = &client.dev;
    struct i2c_msg msg[2];
    int err;
    if (len > sizeof(capdev.xfer_buf) - 2)
    return -EINVAL;
    capdev.xfer_buf[0] = reg;
    capdev.xfer_buf[1] = len;
    msg[0].addr = client.addr;
    msg[0].flags = 0;
    msg[0].buf = capdev.xfer_buf;
    msg[0].len = 2;
    msg[1].addr = client.addr;
    msg[1].flags = I2C_M_RD;
    msg[1].buf = capdev.xfer_buf;
    msg[1].len = len + 2;
    err = i2c_transfer(client.adapter, msg, ARRAY_SIZE(msg));
    if (err != ARRAY_SIZE(msg))
    return err < 0 ? err : -EIO;
    if (capdev.xfer_buf[0] != reg) {
    dev_err(dev,
    "I2C read error: register address does not match (%#02x vs %02x)\n",
    capdev.xfer_buf[0], reg);
    return -ECOMM;
    }
    memcpy(data, &capdev.xfer_buf[2], len);
    return 0;
    }
//
// Handle interrupt and report the key changes to the input system.
// Multi-touch can be supported; however, it really depends on whether
// the device can multi-touch.
//
#[no_mangle]
unsafe extern "C" fn atmel_captouch_isr(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t atmel_captouch_isr(int irq, void *data)
    {
    struct atmel_captouch_device *capdev = data;
    struct device *dev = &capdev.client.dev;
    int error;
    int i;
    u8 new_btn;
    u8 changed_btn;
    error = atmel_read(capdev, REG_KEY_STATE, &new_btn, 1);
    if (error) {
    dev_err(dev, "failed to read button state: %d\n", error);
    goto out;
    }
    dev_dbg(dev, "%s: button state %#02x\n", __func__, new_btn);
    changed_btn = new_btn ^ capdev.prev_btn;
    capdev.prev_btn = new_btn;
    for (i = 0; i < capdev.num_btn; i++) {
    if (changed_btn & BIT(i))
    input_report_key(capdev.input,
    capdev.keycodes[i],
    new_btn & BIT(i));
    }
    input_sync(capdev.input);
    out:
    return IRQ_HANDLED;
    }
//
// Probe function to setup the device, input system and interrupt
//
#[no_mangle]
unsafe extern "C" fn atmel_captouch_probe(client: *mut i2c_client) -> c_int {
    static int atmel_captouch_probe(struct i2c_client *client)
    {
    struct atmel_captouch_device *capdev;
    struct device *dev = &client.dev;
    struct device_node *node;
    int i;
    int err;
    if (!i2c_check_functionality(client.adapter,
    I2C_FUNC_SMBUS_BYTE_DATA |
    I2C_FUNC_SMBUS_WORD_DATA |
    I2C_FUNC_SMBUS_I2C_BLOCK)) {
    dev_err(dev, "needed i2c functionality is not supported\n");
    return -EINVAL;
    }
    capdev = devm_kzalloc(dev, sizeof(*capdev), GFP_KERNEL);
    if (!capdev)
    return -ENOMEM;
    capdev.client = client;
    err = atmel_read(capdev, REG_KEY_STATE,
    &capdev.prev_btn, sizeof(capdev.prev_btn));
    if (err) {
    dev_err(dev, "failed to read initial button state: %d\n", err);
    return err;
    }
    capdev.input = devm_input_allocate_device(dev);
    if (!capdev.input) {
    dev_err(dev, "failed to allocate input device\n");
    return -ENOMEM;
    }
    capdev.input.id.bustype = BUS_I2C;
    capdev.input.id.product = 0x880A;
    capdev.input.id.version = 0;
    capdev.input.name = "ATMegaXX Capacitive Button Controller";
    __set_bit(EV_KEY, capdev.input.evbit);
    node = dev.of_node;
    if (!node) {
    dev_err(dev, "failed to find matching node in device tree\n");
    return -EINVAL;
    }
    if (of_property_read_bool(node, "autorepeat"))
    __set_bit(EV_REP, capdev.input.evbit);
    capdev.num_btn = of_property_count_u32_elems(node, "linux,keymap");
    if (capdev.num_btn > MAX_NUM_OF_BUTTONS)
    capdev.num_btn = MAX_NUM_OF_BUTTONS;
    err = of_property_read_u32_array(node, "linux,keycodes",
    capdev.keycodes,
    capdev.num_btn);
    if (err) {
    dev_err(dev,
    "failed to read linux,keycode property: %d\n", err);
    return err;
    }
    for (i = 0; i < capdev.num_btn; i++)
    __set_bit(capdev.keycodes[i], capdev.input.keybit);
    capdev.input.keycode = capdev.keycodes;
    capdev.input.keycodesize = sizeof(capdev.keycodes[0]);
    capdev.input.keycodemax = capdev.num_btn;
    err = input_register_device(capdev.input);
    if (err)
    return err;
    err = devm_request_threaded_irq(dev, client.irq,
    core::ptr::null_mut(), atmel_captouch_isr,
    IRQF_ONESHOT,
    "atmel_captouch", capdev);
    if (err) {
    dev_err(dev, "failed to request irq %d: %d\n",
    client.irq, err);
    return err;
    }
    return 0;
    }
    static const struct of_device_id atmel_captouch_of_id[] = {
    {
    .compatible = "atmel,captouch",
    },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, atmel_captouch_of_id);
    static const struct i2c_device_id atmel_captouch_id[] = {
    { .name = "atmel_captouch" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, atmel_captouch_id);
    static struct i2c_driver atmel_captouch_driver = {
    .probe		= atmel_captouch_probe,
    .id_table	= atmel_captouch_id,
    .driver		= {
    .name	= "atmel_captouch",
    .of_match_table = atmel_captouch_of_id,
    },
    };
    module_i2c_driver(atmel_captouch_driver);
// Module information
    MODULE_AUTHOR("Hung-yu Wu <hywu@google.com>");
    MODULE_DESCRIPTION("Atmel ATmegaXX Capacitance Touch Sensor I2C Driver");
    MODULE_LICENSE("GPL v2");
