//! Automatically rewritten from C to Rust
//! Source: drivers/video/backlight/cgbc_bl.c
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
// Congatec Board Controller (CGBC) Backlight Driver
//
// This driver provides backlight control for LCD displays connected to
// Congatec boards via the CGBC (Congatec Board Controller). It integrates
// with the Linux backlight subsystem and communicates with hardware through
// the cgbc-core module.
//
// Copyright (C) 2025 Novatron Oy
//
// Author: Petri Karhula <petri.karhula@novatron.fi>
//

// CGBC command for PWM brightness control
pub const CGBC_CMD_BLT0_PWM: c_uint = 0x75;
pub const CGBC_BL_MAX_BRIGHTNESS: c_int = 100;
//
// CGBC backlight driver data
// @dev: Pointer to the platform device
// @cgbc: Pointer to the parent CGBC device data
// @current_brightness: Current brightness level (0-100)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cgbc_bl_data {
    pub dev: *mut device,
    pub cgbc: *mut cgbc_device_data,
    pub current_brightness: c_uint,
}

#[no_mangle]
unsafe extern "C" fn cgbc_bl_read_brightness(bl_data: *mut cgbc_bl_data) -> c_int {
    static int cgbc_bl_read_brightness(struct cgbc_bl_data *bl_data)
    {
    u8 cmd_buf[4] = { CGBC_CMD_BLT0_PWM };
    u8 reply_buf[3];
    int ret;
    ret = cgbc_command(bl_data.cgbc, cmd_buf, sizeof(cmd_buf),
    reply_buf, sizeof(reply_buf), core::ptr::null_mut());
    if (ret < 0)
    return ret;
//
// Get only PWM duty factor percentage,
// ignore polarity inversion bit (bit 7)
//
    bl_data.current_brightness = FIELD_GET(BLT_PWM_DUTY_MASK, reply_buf[0]);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cgbc_bl_update_status(bl: *mut backlight_device) -> c_int {
    static int cgbc_bl_update_status(struct backlight_device *bl)
    {
    struct cgbc_bl_data *bl_data = bl_get_data(bl);
    u8 cmd_buf[4] = { CGBC_CMD_BLT0_PWM };
    u8 reply_buf[3];
    u8 brightness;
    int ret;
    brightness = backlight_get_brightness(bl);
    if (brightness != bl_data.current_brightness) {
// Read the current values
    ret = cgbc_command(bl_data.cgbc, cmd_buf, sizeof(cmd_buf), reply_buf,
    sizeof(reply_buf), core::ptr::null_mut());
    if (ret < 0) {
    dev_err(bl_data.dev, "Failed to read PWM settings: %d\n", ret);
    return ret;
    }
//
// Prepare command buffer for writing new settings. Only 2nd byte is changed
// to set new brightness (PWM duty cycle %). Other values (polarity, frequency)
// are preserved from the read values.
//
    cmd_buf[1] = (reply_buf[0] & ~BLT_PWM_DUTY_MASK) |
    FIELD_PREP(BLT_PWM_DUTY_MASK, brightness);
    cmd_buf[2] = reply_buf[1];
    cmd_buf[3] = reply_buf[2];
    ret = cgbc_command(bl_data.cgbc, cmd_buf, sizeof(cmd_buf), reply_buf,
    sizeof(reply_buf), core::ptr::null_mut());
    if (ret < 0) {
    dev_err(bl_data.dev, "Failed to set brightness: %d\n", ret);
    return ret;
    }
    bl_data.current_brightness = reply_buf[0] & BLT_PWM_DUTY_MASK;
// Verify the setting was applied correctly
    if (bl_data.current_brightness != brightness) {
    dev_err(bl_data.dev,
    "Brightness setting verification failed (got %u, expected %u)\n",
    bl_data.current_brightness, (unsigned int)brightness);
    return -EIO;
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cgbc_bl_get_brightness(bl: *mut backlight_device) -> c_int {
    static int cgbc_bl_get_brightness(struct backlight_device *bl)
    {
    struct cgbc_bl_data *bl_data = bl_get_data(bl);
    int ret;
    ret = cgbc_bl_read_brightness(bl_data);
    if (ret < 0) {
    dev_err(bl_data.dev, "Failed to read brightness: %d\n", ret);
    return ret;
    }
    return bl_data.current_brightness;
    }
    static const struct backlight_ops cgbc_bl_ops = {
    .options = BL_CORE_SUSPENDRESUME,
    .update_status = cgbc_bl_update_status,
    .get_brightness = cgbc_bl_get_brightness,
    };
#[no_mangle]
unsafe extern "C" fn cgbc_bl_probe(pdev: *mut platform_device) -> c_int {
    static int cgbc_bl_probe(struct platform_device *pdev)
    {
    struct cgbc_device_data *cgbc = dev_get_drvdata(pdev.dev.parent);
    let mut props: backlight_properties = { };
    struct backlight_device *bl_dev;
    struct cgbc_bl_data *bl_data;
    int ret;
    bl_data = devm_kzalloc(&pdev.dev, sizeof(*bl_data), GFP_KERNEL);
    if (!bl_data)
    return -ENOMEM;
    bl_data.dev = &pdev.dev;
    bl_data.cgbc = cgbc;
    ret = cgbc_bl_read_brightness(bl_data);
    if (ret < 0)
    return dev_err_probe(&pdev.dev, ret,
    "Failed to read initial brightness\n");
    props.type = BACKLIGHT_PLATFORM;
    props.max_brightness = CGBC_BL_MAX_BRIGHTNESS;
    props.brightness = bl_data.current_brightness;
    props.scale = BACKLIGHT_SCALE_LINEAR;
    bl_dev = devm_backlight_device_register(&pdev.dev, "cgbc-backlight",
    &pdev.dev, bl_data,
    &cgbc_bl_ops, &props);
    if (IS_ERR(bl_dev))
    return dev_err_probe(&pdev.dev, PTR_ERR(bl_dev),
    "Failed to register backlight device\n");
    platform_set_drvdata(pdev, bl_data);
    return 0;
    }
    static struct platform_driver cgbc_bl_driver = {
    .driver = {
    .name = "cgbc-backlight",
    },
    .probe = cgbc_bl_probe,
    };
    module_platform_driver(cgbc_bl_driver);
    MODULE_AUTHOR("Petri Karhula <petri.karhula@novatron.fi>");
    MODULE_DESCRIPTION("Congatec Board Controller (CGBC) Backlight Driver");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:cgbc-backlight");
