//! Automatically rewritten from C to Rust
//! Source: drivers/input/keyboard/mpr121_touchkey.c
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
// Touchkey driver for Freescale MPR121 Controllor
//
// Copyright (C) 2011 Freescale Semiconductor, Inc.
// Author: Zhang Jiejing <jiejing.zhang@freescale.com>
//
// Based on mcs_touchkey.c
//

// Register definitions
pub const ELE_TOUCH_STATUS_0_ADDR: c_uint = 0x0;

pub const MHD_RISING_ADDR: c_uint = 0x2b;
pub const NHD_RISING_ADDR: c_uint = 0x2c;
pub const NCL_RISING_ADDR: c_uint = 0x2d;
pub const FDL_RISING_ADDR: c_uint = 0x2e;
pub const MHD_FALLING_ADDR: c_uint = 0x2f;
pub const NHD_FALLING_ADDR: c_uint = 0x30;
pub const NCL_FALLING_ADDR: c_uint = 0x31;
pub const FDL_FALLING_ADDR: c_uint = 0x32;
pub const ELE0_TOUCH_THRESHOLD_ADDR: c_uint = 0x41;
pub const ELE0_RELEASE_THRESHOLD_ADDR: c_uint = 0x42;
pub const AFE_CONF_ADDR: c_uint = 0x5c;
pub const FILTER_CONF_ADDR: c_uint = 0x5d;
//
// ELECTRODE_CONF_ADDR: This register configures the number of
// enabled capacitance sensing inputs and its run/suspend mode.
//
pub const ELECTRODE_CONF_ADDR: c_uint = 0x5e;
pub const ELECTRODE_CONF_QUICK_CHARGE: c_uint = 0x80;
pub const AUTO_CONFIG_CTRL_ADDR: c_uint = 0x7b;
pub const AUTO_CONFIG_USL_ADDR: c_uint = 0x7d;
pub const AUTO_CONFIG_LSL_ADDR: c_uint = 0x7e;
pub const AUTO_CONFIG_TL_ADDR: c_uint = 0x7f;
// Threshold of touch/release trigger
pub const TOUCH_THRESHOLD: c_uint = 0x08;
pub const RELEASE_THRESHOLD: c_uint = 0x05;
// Masks for touch and release triggers
pub const TOUCH_STATUS_MASK: c_uint = 0xfff;
// MPR121 has 12 keys
pub const MPR121_MAX_KEY_COUNT: c_int = 12;
pub const MPR121_MIN_POLL_INTERVAL: c_int = 10;
pub const MPR121_MAX_POLL_INTERVAL: c_int = 200;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpr121_touchkey {
    pub client: *mut i2c_client,
    pub input_dev: *mut input_dev,
    pub statusbits: c_uint,
    pub keycount: c_uint,
    pub keycodes: [u32; MPR121_MAX_KEY_COUNT],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpr121_init_register {
    pub addr: c_int,
    pub val: u8,
}

    static const struct mpr121_init_register init_reg_table[] = {
    { MHD_RISING_ADDR,	0x1 },
    { NHD_RISING_ADDR,	0x1 },
    { MHD_FALLING_ADDR,	0x1 },
    { NHD_FALLING_ADDR,	0x1 },
    { NCL_FALLING_ADDR,	0xff },
    { FDL_FALLING_ADDR,	0x02 },
    { FILTER_CONF_ADDR,	0x04 },
    { AFE_CONF_ADDR,	0x0b },
    { AUTO_CONFIG_CTRL_ADDR, 0x0b },
    };
#[no_mangle]
unsafe extern "C" fn mpr_touchkey_report(dev: *mut input_dev) {
    static void mpr_touchkey_report(struct input_dev *dev)
    {
    struct mpr121_touchkey *mpr121 = input_get_drvdata(dev);
    struct input_dev *input = mpr121.input_dev;
    struct i2c_client *client = mpr121.client;
    unsigned long bit_changed;
    unsigned int key_num;
    int reg;
    reg = i2c_smbus_read_byte_data(client, ELE_TOUCH_STATUS_1_ADDR);
    if (reg < 0) {
    dev_err(&client.dev, "i2c read error [%d]\n", reg);
    return;
    }
    reg <<= 8;
    reg |= i2c_smbus_read_byte_data(client, ELE_TOUCH_STATUS_0_ADDR);
    if (reg < 0) {
    dev_err(&client.dev, "i2c read error [%d]\n", reg);
    return;
    }
    reg &= TOUCH_STATUS_MASK;
// use old press bit to figure out which bit changed
    bit_changed = reg ^ mpr121.statusbits;
    mpr121.statusbits = reg;
    for_each_set_bit(key_num, &bit_changed, mpr121.keycount) {
    unsigned int key_val, pressed;
    pressed = reg & BIT(key_num);
    key_val = mpr121.keycodes[key_num];
    input_event(input, EV_MSC, MSC_SCAN, key_num);
    input_report_key(input, key_val, pressed);
    dev_dbg(&client.dev, "key %d %d %s\n", key_num, key_val,
    pressed ? "pressed" : "released");
    }
    input_sync(input);
    }
#[no_mangle]
unsafe extern "C" fn mpr_touchkey_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t mpr_touchkey_interrupt(int irq, void *dev_id)
    {
    struct mpr121_touchkey *mpr121 = dev_id;
    mpr_touchkey_report(mpr121.input_dev);
    return IRQ_HANDLED;
    }
    static int mpr121_phys_init(struct mpr121_touchkey *mpr121,
    struct i2c_client *client, int vdd_uv)
    {
    const struct mpr121_init_register *reg;
    unsigned char usl, lsl, tl, eleconf;
    int i, t, vdd, ret;
// Set up touch/release threshold for ele0-ele11
    for (i = 0; i <= MPR121_MAX_KEY_COUNT; i++) {
    t = ELE0_TOUCH_THRESHOLD_ADDR + (i * 2);
    ret = i2c_smbus_write_byte_data(client, t, TOUCH_THRESHOLD);
    if (ret < 0)
    goto err_i2c_write;
    ret = i2c_smbus_write_byte_data(client, t + 1,
    RELEASE_THRESHOLD);
    if (ret < 0)
    goto err_i2c_write;
    }
// Set up init register
    for (i = 0; i < ARRAY_SIZE(init_reg_table); i++) {
    reg = &init_reg_table[i];
    ret = i2c_smbus_write_byte_data(client, reg.addr, reg.val);
    if (ret < 0)
    goto err_i2c_write;
    }
//
// Capacitance on sensing input varies and needs to be compensated.
// The internal MPR121-auto-configuration can do this if it's
// registers are set properly (based on vdd_uv).
//
    vdd = vdd_uv / 1000;
    usl = ((vdd - 700) * 256) / vdd;
    lsl = (usl * 65) / 100;
    tl = (usl * 90) / 100;
    ret = i2c_smbus_write_byte_data(client, AUTO_CONFIG_USL_ADDR, usl);
    ret |= i2c_smbus_write_byte_data(client, AUTO_CONFIG_LSL_ADDR, lsl);
    ret |= i2c_smbus_write_byte_data(client, AUTO_CONFIG_TL_ADDR, tl);
//
// Quick charge bit will let the capacitive charge to ready
// state quickly, or the buttons may not function after system
// boot.
//
    eleconf = mpr121.keycount | ELECTRODE_CONF_QUICK_CHARGE;
    ret |= i2c_smbus_write_byte_data(client, ELECTRODE_CONF_ADDR,
    eleconf);
    if (ret != 0)
    goto err_i2c_write;
    dev_dbg(&client.dev, "set up with %x keys.\n", mpr121.keycount);
    return 0;
    err_i2c_write:
    dev_err(&client.dev, "i2c write error: %d\n", ret);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mpr_touchkey_probe(client: *mut i2c_client) -> c_int {
    static int mpr_touchkey_probe(struct i2c_client *client)
    {
    struct device *dev = &client.dev;
    int vdd_uv;
    struct mpr121_touchkey *mpr121;
    struct input_dev *input_dev;
    let mut poll_interval: u32 = 0;
    int error;
    int i;
    vdd_uv = devm_regulator_get_enable_read_voltage(dev, "vdd");
    if (vdd_uv < 0)
    return dev_err_probe(dev, vdd_uv, "failed to get vdd voltage\n");
    mpr121 = devm_kzalloc(dev, sizeof(*mpr121), GFP_KERNEL);
    if (!mpr121)
    return -ENOMEM;
    input_dev = devm_input_allocate_device(dev);
    if (!input_dev)
    return -ENOMEM;
    mpr121.client = client;
    mpr121.input_dev = input_dev;
    mpr121.keycount = device_property_count_u32(dev, "linux,keycodes");
    if (mpr121.keycount > MPR121_MAX_KEY_COUNT) {
    dev_err(dev, "too many keys defined (%d)\n", mpr121.keycount);
    return -EINVAL;
    }
    error = device_property_read_u32_array(dev, "linux,keycodes",
    mpr121.keycodes,
    mpr121.keycount);
    if (error) {
    dev_err(dev,
    "failed to read linux,keycode property: %d\n", error);
    return error;
    }
    input_dev.name = "Freescale MPR121 Touchkey";
    input_dev.id.bustype = BUS_I2C;
    input_dev.dev.parent = dev;
    if (device_property_read_bool(dev, "autorepeat"))
    __set_bit(EV_REP, input_dev.evbit);
    input_set_capability(input_dev, EV_MSC, MSC_SCAN);
    input_set_drvdata(input_dev, mpr121);
    input_dev.keycode = mpr121.keycodes;
    input_dev.keycodesize = sizeof(mpr121.keycodes[0]);
    input_dev.keycodemax = mpr121.keycount;
    for (i = 0; i < mpr121.keycount; i++)
    input_set_capability(input_dev, EV_KEY, mpr121.keycodes[i]);
    error = mpr121_phys_init(mpr121, client, vdd_uv);
    if (error) {
    dev_err(dev, "Failed to init register\n");
    return error;
    }
    device_property_read_u32(dev, "poll-interval", &poll_interval);
    if (client.irq) {
    error = devm_request_threaded_irq(dev, client.irq, core::ptr::null_mut(),
    mpr_touchkey_interrupt,
    IRQF_TRIGGER_FALLING |
    IRQF_ONESHOT,
    dev.driver.name, mpr121);
    if (error) {
    dev_err(dev, "Failed to register interrupt\n");
    return error;
    }
    } else if (poll_interval) {
    if (poll_interval < MPR121_MIN_POLL_INTERVAL)
    return -EINVAL;
    if (poll_interval > MPR121_MAX_POLL_INTERVAL)
    return -EINVAL;
    error = input_setup_polling(input_dev, mpr_touchkey_report);
    if (error) {
    dev_err(dev, "Failed to setup polling\n");
    return error;
    }
    input_set_poll_interval(input_dev, poll_interval);
    input_set_min_poll_interval(input_dev,
    MPR121_MIN_POLL_INTERVAL);
    input_set_max_poll_interval(input_dev,
    MPR121_MAX_POLL_INTERVAL);
    } else {
    dev_err(dev,
    "invalid IRQ number and polling not configured\n");
    return -EINVAL;
    }
    error = input_register_device(input_dev);
    if (error)
    return error;
    i2c_set_clientdata(client, mpr121);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mpr_suspend(dev: *mut device) -> c_int {
    static int mpr_suspend(struct device *dev)
    {
    struct i2c_client *client = to_i2c_client(dev);
    i2c_smbus_write_byte_data(client, ELECTRODE_CONF_ADDR, 0x00);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mpr_resume(dev: *mut device) -> c_int {
    static int mpr_resume(struct device *dev)
    {
    struct i2c_client *client = to_i2c_client(dev);
    struct mpr121_touchkey *mpr121 = i2c_get_clientdata(client);
    i2c_smbus_write_byte_data(client, ELECTRODE_CONF_ADDR,
    mpr121.keycount);
    return 0;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(mpr121_touchkey_pm_ops, mpr_suspend, mpr_resume);
    static const struct i2c_device_id mpr121_id[] = {
    { .name = "mpr121_touchkey" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, mpr121_id);

    static const struct of_device_id mpr121_touchkey_dt_match_table[] = {
    { .compatible = "fsl,mpr121-touchkey" },
    { },
    };
    MODULE_DEVICE_TABLE(of, mpr121_touchkey_dt_match_table);

    static struct i2c_driver mpr_touchkey_driver = {
    .driver = {
    .name	= "mpr121",
    .pm	= pm_sleep_ptr(&mpr121_touchkey_pm_ops),
    .of_match_table = of_match_ptr(mpr121_touchkey_dt_match_table),
    },
    .id_table	= mpr121_id,
    .probe		= mpr_touchkey_probe,
    };
    module_i2c_driver(mpr_touchkey_driver);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Zhang Jiejing <jiejing.zhang@freescale.com>");
    MODULE_DESCRIPTION("Touch Key driver for Freescale MPR121 Chip");
