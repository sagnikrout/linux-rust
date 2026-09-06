//! Automatically rewritten from C to Rust
//! Source: drivers/hid/hid-picolcd_backlight.c
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
// Copyright (C) 2010-2012 by Bruno Prémont <bonbons@linux-vserver.org>
//
// Based on Logitech G13 driver (v0.4)
// Copyright (C) 2009 by Rick L. Vinyard, Jr. <rvinyard@cs.nmsu.edu>
//

#[no_mangle]
unsafe extern "C" fn picolcd_get_brightness(bdev: *mut backlight_device) -> c_int {
    static int picolcd_get_brightness(struct backlight_device *bdev)
    {
    struct picolcd_data *data = bl_get_data(bdev);
    return data.lcd_brightness;
    }
#[no_mangle]
unsafe extern "C" fn picolcd_set_brightness(bdev: *mut backlight_device) -> c_int {
    static int picolcd_set_brightness(struct backlight_device *bdev)
    {
    struct picolcd_data *data = bl_get_data(bdev);
    struct hid_report *report = picolcd_out_report(REPORT_BRIGHTNESS, data.hdev);
    unsigned long flags;
    if (!report || report.maxfield != 1 || report.field[0].report_count != 1)
    return -ENODEV;
    data.lcd_brightness = bdev.props.brightness & 0x0ff;
    data.lcd_power      = bdev.props.power;
    spin_lock_irqsave(&data.lock, flags);
    hid_set_field(report.field[0], 0,
    data.lcd_power == BACKLIGHT_POWER_ON ? data.lcd_brightness : 0);
    if (!(data.status & PICOLCD_FAILED))
    hid_hw_request(data.hdev, report, HID_REQ_SET_REPORT);
    spin_unlock_irqrestore(&data.lock, flags);
    return 0;
    }
    static const struct backlight_ops picolcd_blops = {
    .update_status  = picolcd_set_brightness,
    .get_brightness = picolcd_get_brightness,
    };
#[no_mangle]
pub unsafe extern "C" fn picolcd_init_backlight(data: *mut picolcd_data, report: *mut hid_report) -> c_int {
    int picolcd_init_backlight(struct picolcd_data *data, struct hid_report *report)
    {
    struct device *dev = &data.hdev.dev;
    struct backlight_device *bdev;
    struct backlight_properties props;
    if (!report)
    return -ENODEV;
    if (report.maxfield != 1 || report.field[0].report_count != 1 ||
    report.field[0].report_size != 8) {
    dev_err(dev, "unsupported BRIGHTNESS report");
    return -EINVAL;
    }
    memset(&props, 0, sizeof(props));
    props.type = BACKLIGHT_RAW;
    props.max_brightness = 0xff;
    bdev = backlight_device_register(dev_name(dev), dev, data,
    &picolcd_blops, &props);
    if (IS_ERR(bdev)) {
    dev_err(dev, "failed to register backlight\n");
    return PTR_ERR(bdev);
    }
    bdev.props.brightness     = 0xff;
    data.lcd_brightness       = 0xff;
    data.backlight            = bdev;
    picolcd_set_brightness(bdev);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn picolcd_exit_backlight(data: *mut picolcd_data) {
    void picolcd_exit_backlight(struct picolcd_data *data)
    {
    struct backlight_device *bdev = data.backlight;
    data.backlight = core::ptr::null_mut();
    backlight_device_unregister(bdev);
    }
#[no_mangle]
pub unsafe extern "C" fn picolcd_resume_backlight(data: *mut picolcd_data) -> c_int {
    int picolcd_resume_backlight(struct picolcd_data *data)
    {
    if (!data.backlight)
    return 0;
    return picolcd_set_brightness(data.backlight);
    }

#[no_mangle]
pub unsafe extern "C" fn picolcd_suspend_backlight(data: *mut picolcd_data) {
    void picolcd_suspend_backlight(struct picolcd_data *data)
    {
    let mut bl_power: c_int = data.lcd_power;
    if (!data.backlight)
    return;
    data.backlight.props.power = BACKLIGHT_POWER_OFF;
    picolcd_set_brightness(data.backlight);
    data.lcd_power = data.backlight.props.power = bl_power;
    }
