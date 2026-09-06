//! Automatically rewritten from C to Rust
//! Source: drivers/input/misc/drv2665.c
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
// DRV2665 haptics driver family
//
// Author: Dan Murphy <dmurphy@ti.com>
//
// Copyright: (C) 2015 Texas Instruments, Inc.
//

// Control registers
pub const DRV2665_STATUS: c_uint = 0x00;
pub const DRV2665_CTRL_1: c_uint = 0x01;
pub const DRV2665_CTRL_2: c_uint = 0x02;
pub const DRV2665_FIFO: c_uint = 0x0b;
// Status Register

// Control 1 Register
pub const DRV2665_25_VPP_GAIN: c_uint = 0x00;
pub const DRV2665_50_VPP_GAIN: c_uint = 0x01;
pub const DRV2665_75_VPP_GAIN: c_uint = 0x02;
pub const DRV2665_100_VPP_GAIN: c_uint = 0x03;
pub const DRV2665_DIGITAL_IN: c_uint = 0xfc;

// Control 2 Register

pub const DRV2665_5_MS_IDLE_TOUT: c_uint = 0x00;
pub const DRV2665_10_MS_IDLE_TOUT: c_uint = 0x04;
pub const DRV2665_15_MS_IDLE_TOUT: c_uint = 0x08;
pub const DRV2665_20_MS_IDLE_TOUT: c_uint = 0x0c;
//
// struct drv2665_data -
// @input_dev: Pointer to the input device
// @client: Pointer to the I2C client
// @regmap: Register map of the device
// @work: Work item used to off load the enable/disable of the vibration
// @regulator: Pointer to the regulator for the IC
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drv2665_data {
    pub input_dev: *mut input_dev,
    pub client: *mut i2c_client,
    pub regmap: *mut regmap,
    pub work: work_struct,
    pub regulator: *mut regulator,
}

// 8kHz Sine wave to stream to the FIFO
    static const u8 drv2665_sine_wave_form[] = {
    0x00, 0x10, 0x20, 0x2e, 0x3c, 0x48, 0x53, 0x5b, 0x61, 0x65, 0x66,
    0x65, 0x61, 0x5b, 0x53, 0x48, 0x3c, 0x2e, 0x20, 0x10,
    0x00, 0xf0, 0xe0, 0xd2, 0xc4, 0xb8, 0xad, 0xa5, 0x9f, 0x9b, 0x9a,
    0x9b, 0x9f, 0xa5, 0xad, 0xb8, 0xc4, 0xd2, 0xe0, 0xf0, 0x00,
    };
    static const struct reg_default drv2665_reg_defs[] = {
    { DRV2665_STATUS, 0x02 },
    { DRV2665_CTRL_1, 0x28 },
    { DRV2665_CTRL_2, 0x40 },
    { DRV2665_FIFO, 0x00 },
    };
#[no_mangle]
unsafe extern "C" fn drv2665_worker(work: *mut work_struct) {
    static void drv2665_worker(struct work_struct *work)
    {
    struct drv2665_data *haptics =
    container_of(work, struct drv2665_data, work);
    unsigned int read_buf;
    int error;
    error = regmap_read(haptics.regmap, DRV2665_STATUS, &read_buf);
    if (error) {
    dev_err(&haptics.client.dev,
    "Failed to read status: %d\n", error);
    return;
    }
    if (read_buf & DRV2665_FIFO_EMPTY) {
    error = regmap_bulk_write(haptics.regmap,
    DRV2665_FIFO,
    drv2665_sine_wave_form,
    ARRAY_SIZE(drv2665_sine_wave_form));
    if (error) {
    dev_err(&haptics.client.dev,
    "Failed to write FIFO: %d\n", error);
    return;
    }
    }
    }
    static int drv2665_haptics_play(struct input_dev *input, void *data,
    struct ff_effect *effect)
    {
    struct drv2665_data *haptics = input_get_drvdata(input);
    schedule_work(&haptics.work);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn drv2665_close(input: *mut input_dev) {
    static void drv2665_close(struct input_dev *input)
    {
    struct drv2665_data *haptics = input_get_drvdata(input);
    int error;
    cancel_work_sync(&haptics.work);
    error = regmap_update_bits(haptics.regmap, DRV2665_CTRL_2,
    DRV2665_STANDBY, DRV2665_STANDBY);
    if (error)
    dev_err(&haptics.client.dev,
    "Failed to enter standby mode: %d\n", error);
    }
    static const struct reg_sequence drv2665_init_regs[] = {
    { DRV2665_CTRL_2, 0 | DRV2665_10_MS_IDLE_TOUT },
    { DRV2665_CTRL_1, DRV2665_25_VPP_GAIN },
    };
#[no_mangle]
unsafe extern "C" fn drv2665_init(haptics: *mut drv2665_data) -> c_int {
    static int drv2665_init(struct drv2665_data *haptics)
    {
    int error;
    error = regmap_register_patch(haptics.regmap,
    drv2665_init_regs,
    ARRAY_SIZE(drv2665_init_regs));
    if (error) {
    dev_err(&haptics.client.dev,
    "Failed to write init registers: %d\n",
    error);
    return error;
    }
    return 0;
    }
    static const struct regmap_config drv2665_regmap_config = {
    .reg_bits = 8,
    .val_bits = 8,
    .max_register = DRV2665_FIFO,
    .reg_defaults = drv2665_reg_defs,
    .num_reg_defaults = ARRAY_SIZE(drv2665_reg_defs),
    .cache_type = REGCACHE_NONE,
    };
#[no_mangle]
unsafe extern "C" fn drv2665_probe(client: *mut i2c_client) -> c_int {
    static int drv2665_probe(struct i2c_client *client)
    {
    struct drv2665_data *haptics;
    int error;
    haptics = devm_kzalloc(&client.dev, sizeof(*haptics), GFP_KERNEL);
    if (!haptics)
    return -ENOMEM;
    haptics.regulator = devm_regulator_get(&client.dev, "vbat");
    if (IS_ERR(haptics.regulator)) {
    error = PTR_ERR(haptics.regulator);
    dev_err(&client.dev,
    "unable to get regulator, error: %d\n", error);
    return error;
    }
    haptics.input_dev = devm_input_allocate_device(&client.dev);
    if (!haptics.input_dev) {
    dev_err(&client.dev, "Failed to allocate input device\n");
    return -ENOMEM;
    }
    haptics.input_dev.name = "drv2665:haptics";
    haptics.input_dev.dev.parent = client.dev.parent;
    haptics.input_dev.close = drv2665_close;
    input_set_drvdata(haptics.input_dev, haptics);
    input_set_capability(haptics.input_dev, EV_FF, FF_RUMBLE);
    error = input_ff_create_memless(haptics.input_dev, core::ptr::null_mut(),
    drv2665_haptics_play);
    if (error) {
    dev_err(&client.dev, "input_ff_create() failed: %d\n",
    error);
    return error;
    }
    INIT_WORK(&haptics.work, drv2665_worker);
    haptics.client = client;
    i2c_set_clientdata(client, haptics);
    haptics.regmap = devm_regmap_init_i2c(client, &drv2665_regmap_config);
    if (IS_ERR(haptics.regmap)) {
    error = PTR_ERR(haptics.regmap);
    dev_err(&client.dev, "Failed to allocate register map: %d\n",
    error);
    return error;
    }
    error = drv2665_init(haptics);
    if (error) {
    dev_err(&client.dev, "Device init failed: %d\n", error);
    return error;
    }
    error = input_register_device(haptics.input_dev);
    if (error) {
    dev_err(&client.dev, "couldn't register input device: %d\n",
    error);
    return error;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn drv2665_suspend(dev: *mut device) -> c_int {
    static int drv2665_suspend(struct device *dev)
    {
    struct drv2665_data *haptics = dev_get_drvdata(dev);
    int error;
    guard(mutex)(&haptics.input_dev.mutex);
    if (input_device_enabled(haptics.input_dev)) {
    error = regmap_update_bits(haptics.regmap, DRV2665_CTRL_2,
    DRV2665_STANDBY, DRV2665_STANDBY);
    if (error) {
    dev_err(dev, "Failed to set standby mode\n");
    regulator_disable(haptics.regulator);
    return error;
    }
    error = regulator_disable(haptics.regulator);
    if (error) {
    dev_err(dev, "Failed to disable regulator\n");
    regmap_update_bits(haptics.regmap,
    DRV2665_CTRL_2,
    DRV2665_STANDBY, 0);
    return error;
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn drv2665_resume(dev: *mut device) -> c_int {
    static int drv2665_resume(struct device *dev)
    {
    struct drv2665_data *haptics = dev_get_drvdata(dev);
    int error;
    guard(mutex)(&haptics.input_dev.mutex);
    if (input_device_enabled(haptics.input_dev)) {
    error = regulator_enable(haptics.regulator);
    if (error) {
    dev_err(dev, "Failed to enable regulator\n");
    return error;
    }
    error = regmap_update_bits(haptics.regmap, DRV2665_CTRL_2,
    DRV2665_STANDBY, 0);
    if (error) {
    dev_err(dev, "Failed to unset standby mode\n");
    regulator_disable(haptics.regulator);
    return error;
    }
    }
    return 0;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(drv2665_pm_ops, drv2665_suspend, drv2665_resume);
    static const struct i2c_device_id drv2665_id[] = {
    { .name = "drv2665" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, drv2665_id);

    static const struct of_device_id drv2665_of_match[] = {
    { .compatible = "ti,drv2665", },
    { }
    };
    MODULE_DEVICE_TABLE(of, drv2665_of_match);

    static struct i2c_driver drv2665_driver = {
    .probe		= drv2665_probe,
    .driver		= {
    .name	= "drv2665-haptics",
    .of_match_table = of_match_ptr(drv2665_of_match),
    .pm	= pm_sleep_ptr(&drv2665_pm_ops),
    },
    .id_table = drv2665_id,
    };
    module_i2c_driver(drv2665_driver);
    MODULE_DESCRIPTION("TI DRV2665 haptics driver");
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Dan Murphy <dmurphy@ti.com>");
