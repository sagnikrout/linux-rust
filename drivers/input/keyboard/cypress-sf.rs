//! Automatically rewritten from C to Rust
//! Source: drivers/input/keyboard/cypress-sf.c
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
// Cypress StreetFighter Touchkey Driver
//
// Copyright (c) 2021 Yassine Oudjana <y.oudjana@protonmail.com>
//

pub const CYPRESS_SF_REG_BUTTON_STATUS: c_uint = 0x4a;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cypress_sf_data {
    pub client: *mut i2c_client,
    pub input_dev: *mut input_dev,
    pub regulators: [regulator_bulk_data; 2],
    pub keycodes: *mut u32,
    pub keystates: c_ulong,
    pub num_keys: c_int,
}

#[no_mangle]
unsafe extern "C" fn cypress_sf_irq_handler(irq: c_int, devid: *mut c_void) -> irqreturn_t {
    static irqreturn_t cypress_sf_irq_handler(int irq, void *devid)
    {
    struct cypress_sf_data *touchkey = devid;
    unsigned long keystates, changed;
    bool new_state;
    int val, key;
    val = i2c_smbus_read_byte_data(touchkey.client,
    CYPRESS_SF_REG_BUTTON_STATUS);
    if (val < 0) {
    dev_err(&touchkey.client.dev,
    "Failed to read button status: %d", val);
    return IRQ_NONE;
    }
    keystates = val;
    bitmap_xor(&changed, &keystates, &touchkey.keystates,
    touchkey.num_keys);
    for_each_set_bit(key, &changed, touchkey.num_keys) {
    new_state = keystates & BIT(key);
    dev_dbg(&touchkey.client.dev,
    "Key %d changed to %d", key, new_state);
    input_report_key(touchkey.input_dev,
    touchkey.keycodes[key], new_state);
    }
    input_sync(touchkey.input_dev);
    touchkey.keystates = keystates;
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn cypress_sf_disable_regulators(arg: *mut c_void) {
    static void cypress_sf_disable_regulators(void *arg)
    {
    struct cypress_sf_data *touchkey = arg;
    regulator_bulk_disable(ARRAY_SIZE(touchkey.regulators),
    touchkey.regulators);
    }
#[no_mangle]
unsafe extern "C" fn cypress_sf_probe(client: *mut i2c_client) -> c_int {
    static int cypress_sf_probe(struct i2c_client *client)
    {
    struct cypress_sf_data *touchkey;
    int key, error;
    touchkey = devm_kzalloc(&client.dev, sizeof(*touchkey), GFP_KERNEL);
    if (!touchkey)
    return -ENOMEM;
    touchkey.client = client;
    i2c_set_clientdata(client, touchkey);
    touchkey.regulators[0].supply = "vdd";
    touchkey.regulators[1].supply = "avdd";
    error = devm_regulator_bulk_get(&client.dev,
    ARRAY_SIZE(touchkey.regulators),
    touchkey.regulators);
    if (error) {
    dev_err(&client.dev, "Failed to get regulators: %d\n", error);
    return error;
    }
    touchkey.num_keys = device_property_read_u32_array(&client.dev,
    "linux,keycodes",
    core::ptr::null_mut(), 0);
    if (touchkey.num_keys < 0) {
// Default key count
    touchkey.num_keys = 2;
    }
    touchkey.keycodes = devm_kcalloc(&client.dev,
    touchkey.num_keys,
    sizeof(*touchkey.keycodes),
    GFP_KERNEL);
    if (!touchkey.keycodes)
    return -ENOMEM;
    error = device_property_read_u32_array(&client.dev, "linux,keycodes",
    touchkey.keycodes,
    touchkey.num_keys);
    if (error) {
    dev_warn(&client.dev,
    "Failed to read keycodes: %d, using defaults\n",
    error);
// Default keycodes
    touchkey.keycodes[0] = KEY_BACK;
    touchkey.keycodes[1] = KEY_MENU;
    }
    error = regulator_bulk_enable(ARRAY_SIZE(touchkey.regulators),
    touchkey.regulators);
    if (error) {
    dev_err(&client.dev,
    "Failed to enable regulators: %d\n", error);
    return error;
    }
    error = devm_add_action_or_reset(&client.dev,
    cypress_sf_disable_regulators,
    touchkey);
    if (error)
    return error;
    touchkey.input_dev = devm_input_allocate_device(&client.dev);
    if (!touchkey.input_dev) {
    dev_err(&client.dev, "Failed to allocate input device\n");
    return -ENOMEM;
    }
    touchkey.input_dev.name = CYPRESS_SF_DEV_NAME;
    touchkey.input_dev.id.bustype = BUS_I2C;
    for (key = 0; key < touchkey.num_keys; ++key)
    input_set_capability(touchkey.input_dev,
    EV_KEY, touchkey.keycodes[key]);
    error = input_register_device(touchkey.input_dev);
    if (error) {
    dev_err(&client.dev,
    "Failed to register input device: %d\n", error);
    return error;
    }
    error = devm_request_threaded_irq(&client.dev, client.irq,
    core::ptr::null_mut(), cypress_sf_irq_handler,
    IRQF_ONESHOT,
    CYPRESS_SF_DEV_NAME, touchkey);
    if (error) {
    dev_err(&client.dev,
    "Failed to register threaded irq: %d", error);
    return error;
    }
    return 0;
    };
#[no_mangle]
unsafe extern "C" fn cypress_sf_suspend(dev: *mut device) -> c_int {
    static int cypress_sf_suspend(struct device *dev)
    {
    struct i2c_client *client = to_i2c_client(dev);
    struct cypress_sf_data *touchkey = i2c_get_clientdata(client);
    int error;
    disable_irq(client.irq);
    error = regulator_bulk_disable(ARRAY_SIZE(touchkey.regulators),
    touchkey.regulators);
    if (error) {
    dev_err(dev, "Failed to disable regulators: %d", error);
    enable_irq(client.irq);
    return error;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cypress_sf_resume(dev: *mut device) -> c_int {
    static int cypress_sf_resume(struct device *dev)
    {
    struct i2c_client *client = to_i2c_client(dev);
    struct cypress_sf_data *touchkey = i2c_get_clientdata(client);
    int error;
    error = regulator_bulk_enable(ARRAY_SIZE(touchkey.regulators),
    touchkey.regulators);
    if (error) {
    dev_err(dev, "Failed to enable regulators: %d", error);
    return error;
    }
    enable_irq(client.irq);
    return 0;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(cypress_sf_pm_ops,
    cypress_sf_suspend, cypress_sf_resume);
    static const struct i2c_device_id cypress_sf_id_table[] = {
    { .name = CYPRESS_SF_DEV_NAME },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, cypress_sf_id_table);

    static const struct of_device_id cypress_sf_of_match[] = {
    { .compatible = "cypress,sf3155", },
    { },
    };
    MODULE_DEVICE_TABLE(of, cypress_sf_of_match);

    static struct i2c_driver cypress_sf_driver = {
    .driver = {
    .name = CYPRESS_SF_DEV_NAME,
    .pm = pm_sleep_ptr(&cypress_sf_pm_ops),
    .of_match_table = of_match_ptr(cypress_sf_of_match),
    },
    .id_table = cypress_sf_id_table,
    .probe = cypress_sf_probe,
    };
    module_i2c_driver(cypress_sf_driver);
    MODULE_AUTHOR("Yassine Oudjana <y.oudjana@protonmail.com>");
    MODULE_DESCRIPTION("Cypress StreetFighter Touchkey Driver");
    MODULE_LICENSE("GPL v2");
