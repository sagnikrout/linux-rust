//! Automatically rewritten from C to Rust
//! Source: drivers/video/backlight/jornada720_lcd.c
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
// LCD driver for HP Jornada 700 series (710/720/728)
// Copyright (C) 2006-2009 Kristoffer Ericson <kristoffer.ericson@gmail.com>
//

pub const LCD_MAX_CONTRAST: c_uint = 0xff;
pub const LCD_DEF_CONTRAST: c_uint = 0x80;
#[no_mangle]
unsafe extern "C" fn jornada_lcd_get_power(ld: *mut lcd_device) -> c_int {
    static int jornada_lcd_get_power(struct lcd_device *ld)
    {
    return PPSR & PPC_LDD2 ? LCD_POWER_ON : LCD_POWER_OFF;
    }
#[no_mangle]
unsafe extern "C" fn jornada_lcd_get_contrast(ld: *mut lcd_device) -> c_int {
    static int jornada_lcd_get_contrast(struct lcd_device *ld)
    {
    int ret;
    if (jornada_lcd_get_power(ld) != LCD_POWER_ON)
    return 0;
    jornada_ssp_start();
    if (jornada_ssp_byte(GETCONTRAST) == TXDUMMY) {
    ret = jornada_ssp_byte(TXDUMMY);
    goto success;
    }
    dev_err(&ld.dev, "failed to set contrast\n");
    ret = -ETIMEDOUT;
    success:
    jornada_ssp_end();
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn jornada_lcd_set_contrast(ld: *mut lcd_device, value: c_int) -> c_int {
    static int jornada_lcd_set_contrast(struct lcd_device *ld, int value)
    {
    let mut ret: c_int = 0;
    jornada_ssp_start();
// start by sending our set contrast cmd to mcu
    if (jornada_ssp_byte(SETCONTRAST) == TXDUMMY) {
// if successful push the new value
    if (jornada_ssp_byte(value) == TXDUMMY)
    goto success;
    }
    dev_err(&ld.dev, "failed to set contrast\n");
    ret = -ETIMEDOUT;
    success:
    jornada_ssp_end();
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn jornada_lcd_set_power(ld: *mut lcd_device, power: c_int) -> c_int {
    static int jornada_lcd_set_power(struct lcd_device *ld, int power)
    {
    if (power != LCD_POWER_ON) {
    PPSR &= ~PPC_LDD2;
    PPDR |= PPC_LDD2;
    } else {
    PPSR |= PPC_LDD2;
    }
    return 0;
    }
    static const struct lcd_ops jornada_lcd_props = {
    .get_contrast = jornada_lcd_get_contrast,
    .set_contrast = jornada_lcd_set_contrast,
    .get_power = jornada_lcd_get_power,
    .set_power = jornada_lcd_set_power,
    };
#[no_mangle]
unsafe extern "C" fn jornada_lcd_probe(pdev: *mut platform_device) -> c_int {
    static int jornada_lcd_probe(struct platform_device *pdev)
    {
    struct lcd_device *lcd_device;
    int ret;
    lcd_device = devm_lcd_device_register(&pdev.dev, S1D_DEVICENAME,
    &pdev.dev, core::ptr::null_mut(), &jornada_lcd_props);
    if (IS_ERR(lcd_device)) {
    ret = PTR_ERR(lcd_device);
    dev_err(&pdev.dev, "failed to register device\n");
    return ret;
    }
    platform_set_drvdata(pdev, lcd_device);
// lets set our default values
    jornada_lcd_set_contrast(lcd_device, LCD_DEF_CONTRAST);
    jornada_lcd_set_power(lcd_device, LCD_POWER_ON);
// give it some time to startup
    msleep(100);
    return 0;
    }
    static struct platform_driver jornada_lcd_driver = {
    .probe	= jornada_lcd_probe,
    .driver	= {
    .name	= "jornada_lcd",
    },
    };
    module_platform_driver(jornada_lcd_driver);
    MODULE_AUTHOR("Kristoffer Ericson <kristoffer.ericson@gmail.com>");
    MODULE_DESCRIPTION("HP Jornada 710/720/728 LCD driver");
    MODULE_LICENSE("GPL");
