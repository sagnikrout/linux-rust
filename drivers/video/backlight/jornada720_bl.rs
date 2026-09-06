//! Automatically rewritten from C to Rust
//! Source: drivers/video/backlight/jornada720_bl.c
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
// Backlight driver for HP Jornada 700 series (710/720/728)
// Copyright (C) 2006-2009 Kristoffer Ericson <kristoffer.ericson@gmail.com>
//

pub const BL_MAX_BRIGHT: c_int = 255;
pub const BL_DEF_BRIGHT: c_int = 25;
#[no_mangle]
unsafe extern "C" fn jornada_bl_get_brightness(bd: *mut backlight_device) -> c_int {
    static int jornada_bl_get_brightness(struct backlight_device *bd)
    {
    int ret;
// check if backlight is on
    if (!(PPSR & PPC_LDD1))
    return 0;
    jornada_ssp_start();
// cmd should return txdummy
    ret = jornada_ssp_byte(GETBRIGHTNESS);
    if (jornada_ssp_byte(GETBRIGHTNESS) != TXDUMMY) {
    dev_err(&bd.dev, "get brightness timeout\n");
    jornada_ssp_end();
    return -ETIMEDOUT;
    }
// exchange txdummy for value
    ret = jornada_ssp_byte(TXDUMMY);
    jornada_ssp_end();
    return BL_MAX_BRIGHT - ret;
    }
#[no_mangle]
unsafe extern "C" fn jornada_bl_update_status(bd: *mut backlight_device) -> c_int {
    static int jornada_bl_update_status(struct backlight_device *bd)
    {
    let mut ret: c_int = 0;
    jornada_ssp_start();
// If backlight is off then really turn it off
    if (backlight_is_blank(bd)) {
    ret = jornada_ssp_byte(BRIGHTNESSOFF);
    if (ret != TXDUMMY) {
    dev_info(&bd.dev, "brightness off timeout\n");
// turn off backlight
    PPSR &= ~PPC_LDD1;
    PPDR |= PPC_LDD1;
    ret = -ETIMEDOUT;
    }
    } else  /* turn on backlight */
    PPSR |= PPC_LDD1;
// send command to our mcu
    if (jornada_ssp_byte(SETBRIGHTNESS) != TXDUMMY) {
    dev_info(&bd.dev, "failed to set brightness\n");
    ret = -ETIMEDOUT;
    goto out;
    }
//
// at this point we expect that the mcu has accepted
// our command and is waiting for our new value
// please note that maximum brightness is 255,
// but due to physical layout it is equal to 0, so we simply
// invert the value (MAX VALUE - NEW VALUE).
//
    if (jornada_ssp_byte(BL_MAX_BRIGHT - bd.props.brightness)
    != TXDUMMY) {
    dev_err(&bd.dev, "set brightness failed\n");
    ret = -ETIMEDOUT;
    }
//
// If infact we get an TXDUMMY as output we are happy and dont
// make any further comments about it
//
    out:
    jornada_ssp_end();
    return ret;
    }
    static const struct backlight_ops jornada_bl_ops = {
    .get_brightness = jornada_bl_get_brightness,
    .update_status = jornada_bl_update_status,
    .options = BL_CORE_SUSPENDRESUME,
    };
#[no_mangle]
unsafe extern "C" fn jornada_bl_probe(pdev: *mut platform_device) -> c_int {
    static int jornada_bl_probe(struct platform_device *pdev)
    {
    struct backlight_properties props;
    int ret;
    struct backlight_device *bd;
    memset(&props, 0, sizeof(struct backlight_properties));
    props.type = BACKLIGHT_RAW;
    props.max_brightness = BL_MAX_BRIGHT;
    bd = devm_backlight_device_register(&pdev.dev, S1D_DEVICENAME,
    &pdev.dev, core::ptr::null_mut(), &jornada_bl_ops,
    &props);
    if (IS_ERR(bd)) {
    ret = PTR_ERR(bd);
    dev_err(&pdev.dev, "failed to register device, err=%x\n", ret);
    return ret;
    }
    bd.props.power = BACKLIGHT_POWER_ON;
    bd.props.brightness = BL_DEF_BRIGHT;
//
// note. make sure max brightness is set otherwise
// you will get seemingly non-related errors when
// trying to change brightness
//
    jornada_bl_update_status(bd);
    platform_set_drvdata(pdev, bd);
    dev_info(&pdev.dev, "HP Jornada 700 series backlight driver\n");
    return 0;
    }
    static struct platform_driver jornada_bl_driver = {
    .probe		= jornada_bl_probe,
    .driver	= {
    .name	= "jornada_bl",
    },
    };
    module_platform_driver(jornada_bl_driver);
    MODULE_AUTHOR("Kristoffer Ericson <kristoffer.ericson>");
    MODULE_DESCRIPTION("HP Jornada 710/720/728 Backlight driver");
    MODULE_LICENSE("GPL");
