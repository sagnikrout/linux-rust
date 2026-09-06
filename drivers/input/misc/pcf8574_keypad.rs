//! Automatically rewritten from C to Rust
//! Source: drivers/input/misc/pcf8574_keypad.c
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
// Driver for a keypad w/16 buttons connected to a PCF8574 I2C I/O expander
//
// Copyright 2005-2008 Analog Devices Inc.
//

    static const unsigned char pcf8574_kp_btncode[] = {
    [0] = KEY_RESERVED,
    [1] = KEY_ENTER,
    [2] = KEY_BACKSLASH,
    [3] = KEY_0,
    [4] = KEY_RIGHTBRACE,
    [5] = KEY_C,
    [6] = KEY_9,
    [7] = KEY_8,
    [8] = KEY_7,
    [9] = KEY_B,
    [10] = KEY_6,
    [11] = KEY_5,
    [12] = KEY_4,
    [13] = KEY_A,
    [14] = KEY_3,
    [15] = KEY_2,
    [16] = KEY_1
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kp_data {
    pub btncode: [c_ushort; ARRAY_SIZE(pcf8574_kp_btncode)],
    pub idev: *mut input_dev,
    pub client: *mut i2c_client,
    pub name: [c_char; 64],
    pub phys: [c_char; 32],
    pub laststate: c_uchar,
}

#[no_mangle]
unsafe extern "C" fn read_state(lp: *mut kp_data) -> c_short {
    static short read_state(struct kp_data *lp)
    {
    unsigned char x, y, a, b;
    i2c_smbus_write_byte(lp.client, 240);
    x = 0xF & (~(i2c_smbus_read_byte(lp.client) >> 4));
    i2c_smbus_write_byte(lp.client, 15);
    y = 0xF & (~i2c_smbus_read_byte(lp.client));
    for (a = 0; x > 0; a++)
    x = x >> 1;
    for (b = 0; y > 0; b++)
    y = y >> 1;
    return ((a - 1) * 4) + b;
    }
#[no_mangle]
unsafe extern "C" fn pcf8574_kp_irq_handler(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t pcf8574_kp_irq_handler(int irq, void *dev_id)
    {
    struct kp_data *lp = dev_id;
    let mut nextstate: c_uchar = read_state(lp);
    if (lp.laststate != nextstate) {
    let mut key_down: c_int = nextstate < ARRAY_SIZE(lp.btncode);
    unsigned short keycode = key_down ?
    lp.btncode[nextstate] : lp.btncode[lp.laststate];
    input_report_key(lp.idev, keycode, key_down);
    input_sync(lp.idev);
    lp.laststate = nextstate;
    }
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn pcf8574_kp_probe(client: *mut i2c_client) -> c_int {
    static int pcf8574_kp_probe(struct i2c_client *client)
    {
    int i, ret;
    struct input_dev *idev;
    struct kp_data *lp;
    if (i2c_smbus_write_byte(client, 240) < 0) {
    dev_err(&client.dev, "probe: write fail\n");
    return -ENODEV;
    }
    lp = kzalloc_obj(*lp);
    if (!lp)
    return -ENOMEM;
    idev = input_allocate_device();
    if (!idev) {
    dev_err(&client.dev, "Can't allocate input device\n");
    ret = -ENOMEM;
    goto fail_allocate;
    }
    lp.idev = idev;
    lp.client = client;
    idev.evbit[0] = BIT_MASK(EV_KEY);
    idev.keycode = lp.btncode;
    idev.keycodesize = sizeof(lp.btncode[0]);
    idev.keycodemax = ARRAY_SIZE(lp.btncode);
    for (i = 0; i < ARRAY_SIZE(pcf8574_kp_btncode); i++) {
    if (lp.btncode[i] <= KEY_MAX) {
    lp.btncode[i] = pcf8574_kp_btncode[i];
    __set_bit(lp.btncode[i], idev.keybit);
    }
    }
    __clear_bit(KEY_RESERVED, idev.keybit);
    sprintf(lp.name, DRV_NAME);
    sprintf(lp.phys, "kp_data/input0");
    idev.name = lp.name;
    idev.phys = lp.phys;
    idev.id.bustype = BUS_I2C;
    idev.id.vendor = 0x0001;
    idev.id.product = 0x0001;
    idev.id.version = 0x0100;
    lp.laststate = read_state(lp);
    ret = request_threaded_irq(client.irq, core::ptr::null_mut(), pcf8574_kp_irq_handler,
    IRQF_TRIGGER_LOW | IRQF_ONESHOT,
    DRV_NAME, lp);
    if (ret) {
    dev_err(&client.dev, "IRQ %d is not free\n", client.irq);
    goto fail_free_device;
    }
    ret = input_register_device(idev);
    if (ret) {
    dev_err(&client.dev, "input_register_device() failed\n");
    goto fail_free_irq;
    }
    i2c_set_clientdata(client, lp);
    return 0;
    fail_free_irq:
    free_irq(client.irq, lp);
    fail_free_device:
    input_free_device(idev);
    fail_allocate:
    kfree(lp);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn pcf8574_kp_remove(client: *mut i2c_client) {
    static void pcf8574_kp_remove(struct i2c_client *client)
    {
    struct kp_data *lp = i2c_get_clientdata(client);
    free_irq(client.irq, lp);
    input_unregister_device(lp.idev);
    kfree(lp);
    }
#[no_mangle]
unsafe extern "C" fn pcf8574_kp_resume(dev: *mut device) -> c_int {
    static int pcf8574_kp_resume(struct device *dev)
    {
    struct i2c_client *client = to_i2c_client(dev);
    enable_irq(client.irq);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pcf8574_kp_suspend(dev: *mut device) -> c_int {
    static int pcf8574_kp_suspend(struct device *dev)
    {
    struct i2c_client *client = to_i2c_client(dev);
    disable_irq(client.irq);
    return 0;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(pcf8574_kp_pm_ops,
    pcf8574_kp_suspend, pcf8574_kp_resume);
    static const struct i2c_device_id pcf8574_kp_id[] = {
    { .name = DRV_NAME },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, pcf8574_kp_id);
    static struct i2c_driver pcf8574_kp_driver = {
    .driver = {
    .name  = DRV_NAME,
    .pm = pm_sleep_ptr(&pcf8574_kp_pm_ops),
    },
    .probe    = pcf8574_kp_probe,
    .remove   = pcf8574_kp_remove,
    .id_table = pcf8574_kp_id,
    };
    module_i2c_driver(pcf8574_kp_driver);
    MODULE_AUTHOR("Michael Hennerich");
    MODULE_DESCRIPTION("Keypad input driver for 16 keys connected to PCF8574");
    MODULE_LICENSE("GPL");
