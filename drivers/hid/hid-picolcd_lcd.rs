//! Automatically rewritten from C to Rust
//! Source: drivers/hid/hid-picolcd_lcd.c
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

//
// lcd class device
//
#[no_mangle]
unsafe extern "C" fn picolcd_get_contrast(ldev: *mut lcd_device) -> c_int {
    static int picolcd_get_contrast(struct lcd_device *ldev)
    {
    struct picolcd_data *data = lcd_get_data(ldev);
    return data.lcd_contrast;
    }
#[no_mangle]
unsafe extern "C" fn picolcd_set_contrast(ldev: *mut lcd_device, contrast: c_int) -> c_int {
    static int picolcd_set_contrast(struct lcd_device *ldev, int contrast)
    {
    struct picolcd_data *data = lcd_get_data(ldev);
    struct hid_report *report = picolcd_out_report(REPORT_CONTRAST, data.hdev);
    unsigned long flags;
    if (!report || report.maxfield != 1 || report.field[0].report_count != 1)
    return -ENODEV;
    data.lcd_contrast = contrast & 0x0ff;
    spin_lock_irqsave(&data.lock, flags);
    hid_set_field(report.field[0], 0, data.lcd_contrast);
    if (!(data.status & PICOLCD_FAILED))
    hid_hw_request(data.hdev, report, HID_REQ_SET_REPORT);
    spin_unlock_irqrestore(&data.lock, flags);
    return 0;
    }
    static const struct lcd_ops picolcd_lcdops = {
    .get_contrast   = picolcd_get_contrast,
    .set_contrast   = picolcd_set_contrast,
    };
#[no_mangle]
pub unsafe extern "C" fn picolcd_init_lcd(data: *mut picolcd_data, report: *mut hid_report) -> c_int {
    int picolcd_init_lcd(struct picolcd_data *data, struct hid_report *report)
    {
    struct device *dev = &data.hdev.dev;
    struct lcd_device *ldev;
    if (!report)
    return -ENODEV;
    if (report.maxfield != 1 || report.field[0].report_count != 1 ||
    report.field[0].report_size != 8) {
    dev_err(dev, "unsupported CONTRAST report");
    return -EINVAL;
    }
    ldev = lcd_device_register(dev_name(dev), dev, data, &picolcd_lcdops);
    if (IS_ERR(ldev)) {
    dev_err(dev, "failed to register LCD\n");
    return PTR_ERR(ldev);
    }
    ldev.props.max_contrast = 0x0ff;
    data.lcd_contrast = 0xe5;
    data.lcd = ldev;
    picolcd_set_contrast(ldev, 0xe5);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn picolcd_exit_lcd(data: *mut picolcd_data) {
    void picolcd_exit_lcd(struct picolcd_data *data)
    {
    struct lcd_device *ldev = data.lcd;
    data.lcd = core::ptr::null_mut();
    lcd_device_unregister(ldev);
    }
#[no_mangle]
pub unsafe extern "C" fn picolcd_resume_lcd(data: *mut picolcd_data) -> c_int {
    int picolcd_resume_lcd(struct picolcd_data *data)
    {
    if (!data.lcd)
    return 0;
    return picolcd_set_contrast(data.lcd, data.lcd_contrast);
    }
