//! Automatically rewritten from C to Rust
//! Source: drivers/video/backlight/sy7758.c
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
// Silergy SY7758 6-channel High Efficiency LED Driver
//
// Copyright (C) 2025 Kancy Joe <kancy2333@outlook.com>
// Copyright (C) 2026 Linaro Limited
// Author: Neil Armstrong <neil.armstrong@linaro.org>
//

pub const DEFAULT_BRIGHTNESS: c_int = 1024;
pub const MAX_BRIGHTNESS: c_int = 4080;
pub const REG_MAX: c_uint = 0xAE;
// Registers
pub const REG_DEV_CTL: c_uint = 0x01;
pub const REG_DEV_ID: c_uint = 0x03;
pub const REG_BRT_12BIT_L: c_uint = 0x10;
pub const REG_BRT_12BIT_H: c_uint = 0x11;
// OTP memory
pub const REG_OTP_CFG0: c_uint = 0xA0;
pub const REG_OTP_CFG1: c_uint = 0xA1;
pub const REG_OTP_CFG2: c_uint = 0xA2;
pub const REG_OTP_CFG5: c_uint = 0xA5;
pub const REG_OTP_CFG9: c_uint = 0xA9;
// Fields

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sy7758 {
    pub client: *mut i2c_client,
    pub regmap: *mut regmap,
    pub gpio: *mut gpio_desc,
    pub bl: *mut backlight_device,
}

    static const struct regmap_config sy7758_regmap_config = {
    .reg_bits = 8,
    .val_bits = 8,
    .max_register = REG_MAX,
    };
#[no_mangle]
unsafe extern "C" fn sy7758_backlight_update_status(backlight_dev: *mut backlight_device) -> c_int {
    static int sy7758_backlight_update_status(struct backlight_device *backlight_dev)
    {
    struct sy7758 *sydev = bl_get_data(backlight_dev);
    let mut brightness: c_uint = backlight_get_brightness(backlight_dev);
    int ret;
    ret = regmap_write(sydev.regmap, REG_BRT_12BIT_L,
    FIELD_PREP(MSK_BRT_12BIT_L,
    brightness & 0xff));
    if (ret)
    return ret;
    ret = regmap_write(sydev.regmap, REG_BRT_12BIT_H,
    FIELD_PREP(MSK_BRT_12BIT_H,
    (brightness >> 8) & 0xf));
    if (ret)
    return ret;
    return 0;
    }
    static const struct backlight_ops sy7758_backlight_ops = {
    .options = BL_CORE_SUSPENDRESUME,
    .update_status = sy7758_backlight_update_status,
    };
#[no_mangle]
unsafe extern "C" fn sy7758_init(sydev: *mut sy7758) -> c_int {
    static int sy7758_init(struct sy7758 *sydev)
    {
    let mut ret: c_int = 0;
    ret = regmap_write(sydev.regmap, REG_DEV_CTL,
    BIT_DEV_CTL_FAST | BIT_DEV_CTL_BL_CTLB |
    FIELD_PREP(MSK_DEV_CTL_BRT_MODE, 2));
    if (ret)
    return ret;
    ret = regmap_write(sydev.regmap, REG_BRT_12BIT_L,
    FIELD_PREP(MSK_BRT_12BIT_L,
    DEFAULT_BRIGHTNESS & 0xff));
    if (ret)
    return ret;
    ret = regmap_write(sydev.regmap, REG_BRT_12BIT_H,
    FIELD_PREP(MSK_BRT_12BIT_H,
    (DEFAULT_BRIGHTNESS >> 8)));
    if (ret)
    return ret;
    ret = regmap_write(sydev.regmap, REG_OTP_CFG5,
    FIELD_PREP(MSK_CFG5_PS_MODE, 6) |
    FIELD_PREP(MSK_CFG5_PWM_FREQ, 4));
    if (ret)
    return ret;
    ret = regmap_write(sydev.regmap, REG_OTP_CFG0,
    FIELD_PREP(MSK_CFG0_CURRENT_LOW, 85));
    if (ret)
    return ret;
    ret = regmap_write(sydev.regmap, REG_OTP_CFG1,
    BIT_CFG1_PDET_STDBY |
    FIELD_PREP(MSK_CFG1_CURRENT_MAX, 1) |
    FIELD_PREP(MSK_CFG1_CURRENT_HIGH, 10));
    if (ret)
    return ret;
    ret = regmap_write(sydev.regmap, REG_OTP_CFG9,
    FIELD_PREP(MSK_CFG9_VBST_MAX, 4));
    if (ret)
    return ret;
    ret = regmap_write(sydev.regmap, REG_OTP_CFG2,
    BIT_CFG2_BL_ON | BIT_CFG2_UVLO_EN);
    if (ret)
    return ret;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sy7758_probe(client: *mut i2c_client) -> c_int {
    static int sy7758_probe(struct i2c_client *client)
    {
    let mut props: backlight_properties = { };
    struct device *dev = &client.dev;
    struct sy7758 *sydev;
    unsigned int dev_id;
    int ret;
    sydev = devm_kzalloc(dev, sizeof(*sydev), GFP_KERNEL);
    if (!sydev)
    return -ENOMEM;
    i2c_set_clientdata(client, sydev);
// Initialize regmap
    sydev.client = client;
    sydev.regmap = devm_regmap_init_i2c(client, &sy7758_regmap_config);
    if (IS_ERR(sydev.regmap))
    return dev_err_probe(dev, PTR_ERR(sydev.regmap),
    "failed to init regmap\n");
// Get and enable regulator
    ret = devm_regulator_get_enable(dev, "vdd");
    if (ret)
    return dev_err_probe(dev, ret, "failed to get regulator\n");
    fsleep(100);
// Get enable GPIO and set to high
    sydev.gpio = devm_gpiod_get(dev, "enable", GPIOD_OUT_HIGH);
    if (IS_ERR(sydev.gpio))
    return dev_err_probe(dev, PTR_ERR(sydev.gpio),
    "failed to get enable GPIO\n");
// Let some time for HW to settle
    fsleep(10000);
// try read and check device id
    ret = regmap_read(sydev.regmap, REG_DEV_ID, &dev_id);
    if (ret < 0)
    return dev_err_probe(dev, ret, "failed to read device id\n");
    if (dev_id != 0x63) {
    dev_err(dev, "unexpected device id: 0x%02x\n", dev_id);
    return -ENODEV;
    }
// Initialize and set default brightness
    ret = sy7758_init(sydev);
    if (ret)
    return ret;
    props.type = BACKLIGHT_RAW;
    props.max_brightness = MAX_BRIGHTNESS;
    props.brightness = DEFAULT_BRIGHTNESS;
    props.scale = BACKLIGHT_SCALE_LINEAR;
    sydev.bl = devm_backlight_device_register(dev, "sy7758-backlight",
    dev, sydev, &sy7758_backlight_ops,
    &props);
    if (IS_ERR(sydev.bl))
    return dev_err_probe(dev, PTR_ERR(sydev.bl),
    "failed to register backlight device\n");
    return backlight_update_status(sydev.bl);
    }
#[no_mangle]
unsafe extern "C" fn sy7758_remove(client: *mut i2c_client) {
    static void sy7758_remove(struct i2c_client *client)
    {
    struct sy7758 *sydev = i2c_get_clientdata(client);
    backlight_disable(sydev.bl);
    }
    static const struct i2c_device_id sy7758_ids[] = {
    { "sy7758" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, sy7758_ids);
    static const struct of_device_id sy7758_match_table[] = {
    { .compatible = "silergy,sy7758", },
    { },
    };
    MODULE_DEVICE_TABLE(of, sy7758_match_table);
    static struct i2c_driver sy7758_driver = {
    .driver = {
    .name = "sy7758",
    .of_match_table = sy7758_match_table,
    },
    .probe = sy7758_probe,
    .remove = sy7758_remove,
    .id_table = sy7758_ids,
    };
    module_i2c_driver(sy7758_driver);
    MODULE_DESCRIPTION("Silergy SY7758 Backlight Driver");
    MODULE_AUTHOR("Kancy Joe <kancy2333@outlook.com>");
    MODULE_AUTHOR("Neil Armstrong <neil.armstrong@linaro.org>");
    MODULE_LICENSE("GPL");
