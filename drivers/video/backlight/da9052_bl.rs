//! Automatically rewritten from C to Rust
//! Source: drivers/video/backlight/da9052_bl.c
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
// Backlight Driver for Dialog DA9052 PMICs
//
// Copyright(c) 2012 Dialog Semiconductor Ltd.
//
// Author: David Dajun Chen <dchen@diasemi.com>
//

pub const DA9052_MAX_BRIGHTNESS: c_uint = 0xFF;
    enum {
    DA9052_WLEDS_OFF,
    DA9052_WLEDS_ON,
    };
    enum {
    DA9052_TYPE_WLED1,
    DA9052_TYPE_WLED2,
    DA9052_TYPE_WLED3,
    };
    static const unsigned char wled_bank[] = {
    DA9052_LED1_CONF_REG,
    DA9052_LED2_CONF_REG,
    DA9052_LED3_CONF_REG,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct da9052_bl {
    pub da9052: *mut da9052,
    pub brightness: c_uint,
    pub state: c_uint,
    pub led_reg: c_uint,
}

#[no_mangle]
unsafe extern "C" fn da9052_adjust_wled_brightness(wleds: *mut da9052_bl) -> c_int {
    static int da9052_adjust_wled_brightness(struct da9052_bl *wleds)
    {
    unsigned char boost_en;
    unsigned char i_sink;
    int ret;
    boost_en = 0x3F;
    i_sink = 0xFF;
    if (wleds.state == DA9052_WLEDS_OFF) {
    boost_en = 0x00;
    i_sink = 0x00;
    }
    ret = da9052_reg_write(wleds.da9052, DA9052_BOOST_REG, boost_en);
    if (ret < 0)
    return ret;
    ret = da9052_reg_write(wleds.da9052, DA9052_LED_CONT_REG, i_sink);
    if (ret < 0)
    return ret;
    ret = da9052_reg_write(wleds.da9052, wled_bank[wleds.led_reg], 0x0);
    if (ret < 0)
    return ret;
    usleep_range(10000, 11000);
    if (wleds.brightness) {
    ret = da9052_reg_write(wleds.da9052, wled_bank[wleds.led_reg],
    wleds.brightness);
    if (ret < 0)
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn da9052_backlight_update_status(bl: *mut backlight_device) -> c_int {
    static int da9052_backlight_update_status(struct backlight_device *bl)
    {
    let mut brightness: c_int = bl.props.brightness;
    struct da9052_bl *wleds = bl_get_data(bl);
    wleds.brightness = brightness;
    wleds.state = DA9052_WLEDS_ON;
    return da9052_adjust_wled_brightness(wleds);
    }
#[no_mangle]
unsafe extern "C" fn da9052_backlight_get_brightness(bl: *mut backlight_device) -> c_int {
    static int da9052_backlight_get_brightness(struct backlight_device *bl)
    {
    struct da9052_bl *wleds = bl_get_data(bl);
    return wleds.brightness;
    }
    static const struct backlight_ops da9052_backlight_ops = {
    .update_status = da9052_backlight_update_status,
    .get_brightness = da9052_backlight_get_brightness,
    };
#[no_mangle]
unsafe extern "C" fn da9052_backlight_probe(pdev: *mut platform_device) -> c_int {
    static int da9052_backlight_probe(struct platform_device *pdev)
    {
    struct backlight_device *bl;
    struct backlight_properties props;
    struct da9052_bl *wleds;
    wleds = devm_kzalloc(&pdev.dev, sizeof(struct da9052_bl), GFP_KERNEL);
    if (!wleds)
    return -ENOMEM;
    wleds.da9052 = dev_get_drvdata(pdev.dev.parent);
    wleds.brightness = 0;
    wleds.led_reg = platform_get_device_id(pdev).driver_data;
    wleds.state = DA9052_WLEDS_OFF;
    memset(&props, 0, sizeof(struct backlight_properties));
    props.type = BACKLIGHT_RAW;
    props.max_brightness = DA9052_MAX_BRIGHTNESS;
    bl = devm_backlight_device_register(&pdev.dev, pdev.name,
    wleds.da9052.dev, wleds,
    &da9052_backlight_ops, &props);
    if (IS_ERR(bl)) {
    dev_err(&pdev.dev, "Failed to register backlight\n");
    return PTR_ERR(bl);
    }
    bl.props.max_brightness = DA9052_MAX_BRIGHTNESS;
    bl.props.brightness = 0;
    platform_set_drvdata(pdev, bl);
    return da9052_adjust_wled_brightness(wleds);
    }
#[no_mangle]
unsafe extern "C" fn da9052_backlight_remove(pdev: *mut platform_device) {
    static void da9052_backlight_remove(struct platform_device *pdev)
    {
    struct backlight_device *bl = platform_get_drvdata(pdev);
    struct da9052_bl *wleds = bl_get_data(bl);
    wleds.brightness = 0;
    wleds.state = DA9052_WLEDS_OFF;
    da9052_adjust_wled_brightness(wleds);
    }
    static const struct platform_device_id da9052_wled_ids[] = {
    {
    .name		= "da9052-wled1",
    .driver_data	= DA9052_TYPE_WLED1,
    },
    {
    .name		= "da9052-wled2",
    .driver_data	= DA9052_TYPE_WLED2,
    },
    {
    .name		= "da9052-wled3",
    .driver_data	= DA9052_TYPE_WLED3,
    },
    { },
    };
    MODULE_DEVICE_TABLE(platform, da9052_wled_ids);
    static struct platform_driver da9052_wled_driver = {
    .probe		= da9052_backlight_probe,
    .remove		= da9052_backlight_remove,
    .id_table	= da9052_wled_ids,
    .driver	= {
    .name	= "da9052-wled",
    },
    };
    module_platform_driver(da9052_wled_driver);
    MODULE_AUTHOR("David Dajun Chen <dchen@diasemi.com>");
    MODULE_DESCRIPTION("Backlight driver for DA9052 PMIC");
    MODULE_LICENSE("GPL");
