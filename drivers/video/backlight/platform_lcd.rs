//! Automatically rewritten from C to Rust
//! Source: drivers/video/backlight/platform_lcd.c
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
// drivers/video/backlight/platform_lcd.c
//
// Copyright 2008 Simtec Electronics
// Ben Dooks <ben@simtec.co.uk>
//
// Generic platform-device LCD power control interface.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct platform_lcd {
    pub us: *mut device,
    pub lcd: *mut lcd_device,
    pub pdata: *mut plat_lcd_data,
    pub power: c_uint,
    pub suspended:1: c_uint,
}

    static inline struct platform_lcd *to_our_lcd(struct lcd_device *lcd)
    {
    return lcd_get_data(lcd);
    }
#[no_mangle]
unsafe extern "C" fn platform_lcd_get_power(lcd: *mut lcd_device) -> c_int {
    static int platform_lcd_get_power(struct lcd_device *lcd)
    {
    struct platform_lcd *plcd = to_our_lcd(lcd);
    return plcd.power;
    }
#[no_mangle]
unsafe extern "C" fn platform_lcd_set_power(lcd: *mut lcd_device, power: c_int) -> c_int {
    static int platform_lcd_set_power(struct lcd_device *lcd, int power)
    {
    struct platform_lcd *plcd = to_our_lcd(lcd);
    let mut lcd_power: c_int = 1;
    if (power == LCD_POWER_OFF || plcd.suspended)
    lcd_power = 0;
    plcd.pdata.set_power(plcd.pdata, lcd_power);
    plcd.power = power;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn platform_lcd_controls_device(lcd: *mut lcd_device, display_device: *mut device) -> bool {
    static bool platform_lcd_controls_device(struct lcd_device *lcd, struct device *display_device)
    {
    struct platform_lcd *plcd = to_our_lcd(lcd);
    return plcd.us.parent == display_device;
    }
    static const struct lcd_ops platform_lcd_ops = {
    .get_power		= platform_lcd_get_power,
    .set_power		= platform_lcd_set_power,
    .controls_device	= platform_lcd_controls_device,
    };
#[no_mangle]
unsafe extern "C" fn platform_lcd_probe(pdev: *mut platform_device) -> c_int {
    static int platform_lcd_probe(struct platform_device *pdev)
    {
    struct plat_lcd_data *pdata;
    struct platform_lcd *plcd;
    struct device *dev = &pdev.dev;
    int err;
    pdata = dev_get_platdata(&pdev.dev);
    if (!pdata) {
    dev_err(dev, "no platform data supplied\n");
    return -EINVAL;
    }
    if (pdata.probe) {
    err = pdata.probe(pdata);
    if (err)
    return err;
    }
    plcd = devm_kzalloc(&pdev.dev, sizeof(struct platform_lcd),
    GFP_KERNEL);
    if (!plcd)
    return -ENOMEM;
    plcd.us = dev;
    plcd.pdata = pdata;
    plcd.lcd = devm_lcd_device_register(&pdev.dev, dev_name(dev), dev,
    plcd, &platform_lcd_ops);
    if (IS_ERR(plcd.lcd)) {
    dev_err(dev, "cannot register lcd device\n");
    return PTR_ERR(plcd.lcd);
    }
    platform_set_drvdata(pdev, plcd);
    platform_lcd_set_power(plcd.lcd, LCD_POWER_REDUCED);
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn platform_lcd_suspend(dev: *mut device) -> c_int {
    static int platform_lcd_suspend(struct device *dev)
    {
    struct platform_lcd *plcd = dev_get_drvdata(dev);
    plcd.suspended = 1;
    platform_lcd_set_power(plcd.lcd, plcd.power);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn platform_lcd_resume(dev: *mut device) -> c_int {
    static int platform_lcd_resume(struct device *dev)
    {
    struct platform_lcd *plcd = dev_get_drvdata(dev);
    plcd.suspended = 0;
    platform_lcd_set_power(plcd.lcd, plcd.power);
    return 0;
    }

    static SIMPLE_DEV_PM_OPS(platform_lcd_pm_ops, platform_lcd_suspend,
    platform_lcd_resume);
    static struct platform_driver platform_lcd_driver = {
    .driver		= {
    .name	= "platform-lcd",
    .pm	= &platform_lcd_pm_ops,
    },
    .probe		= platform_lcd_probe,
    };
    module_platform_driver(platform_lcd_driver);
    MODULE_AUTHOR("Ben Dooks <ben-linux@fluff.org>");
    MODULE_DESCRIPTION("Generic platform-device LCD power control interface");
    MODULE_LICENSE("GPL v2");
    MODULE_ALIAS("platform:platform-lcd");
