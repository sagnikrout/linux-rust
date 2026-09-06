//! Automatically rewritten from C to Rust
//! Source: drivers/hid/hid-lg2ff.c
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
// Force feedback support for Logitech RumblePad and Rumblepad 2
//
// Copyright (c) 2008 Anssi Hannula <anssi.hannula@gmail.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lg2ff_device {
    pub report: *mut hid_report,
}

    static int play_effect(struct input_dev *dev, void *data,
    struct ff_effect *effect)
    {
    struct hid_device *hid = input_get_drvdata(dev);
    struct lg2ff_device *lg2ff = data;
    int weak, strong;
    strong = effect.u.rumble.strong_magnitude;
    weak = effect.u.rumble.weak_magnitude;
    if (weak || strong) {
    weak = weak * 0xff / 0xffff;
    strong = strong * 0xff / 0xffff;
    lg2ff.report.field[0].value[0] = 0x51;
    lg2ff.report.field[0].value[2] = weak;
    lg2ff.report.field[0].value[4] = strong;
    } else {
    lg2ff.report.field[0].value[0] = 0xf3;
    lg2ff.report.field[0].value[2] = 0x00;
    lg2ff.report.field[0].value[4] = 0x00;
    }
    hid_hw_request(hid, lg2ff.report, HID_REQ_SET_REPORT);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn lg2ff_init(hid: *mut hid_device) -> c_int {
    int lg2ff_init(struct hid_device *hid)
    {
    struct lg2ff_device *lg2ff;
    struct hid_report *report;
    struct hid_input *hidinput;
    struct input_dev *dev;
    int error;
    if (list_empty(&hid.inputs)) {
    hid_err(hid, "no inputs found\n");
    return -ENODEV;
    }
    hidinput = list_entry(hid.inputs.next, struct hid_input, list);
    dev = hidinput.input;
// Check that the report looks ok
    report = hid_validate_values(hid, HID_OUTPUT_REPORT, 0, 0, 7);
    if (!report)
    return -ENODEV;
    lg2ff = kmalloc_obj(struct lg2ff_device);
    if (!lg2ff)
    return -ENOMEM;
    set_bit(FF_RUMBLE, dev.ffbit);
    error = input_ff_create_memless(dev, lg2ff, play_effect);
    if (error) {
    kfree(lg2ff);
    return error;
    }
    lg2ff.report = report;
    report.field[0].value[0] = 0xf3;
    report.field[0].value[1] = 0x00;
    report.field[0].value[2] = 0x00;
    report.field[0].value[3] = 0x00;
    report.field[0].value[4] = 0x00;
    report.field[0].value[5] = 0x00;
    report.field[0].value[6] = 0x00;
    hid_hw_request(hid, report, HID_REQ_SET_REPORT);
    hid_info(hid, "Force feedback for Logitech variant 2 rumble devices by Anssi Hannula <anssi.hannula@gmail.com>\n");
    return 0;
    }
