//! Automatically rewritten from C to Rust
//! Source: drivers/hid/hid-lgff.c
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
// Force feedback support for hid-compliant for some of the devices from
// Logitech, namely:
// - WingMan Cordless RumblePad
// - WingMan Force 3D
//
// Copyright (c) 2002-2004 Johann Deneux
// Copyright (c) 2006 Anssi Hannula <anssi.hannula@gmail.com>
//
// Should you need to contact me, the author, you can do so by
// e-mail - mail your message to <johann.deneux@it.uu.se>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dev_type {
    pub idVendor: u16,
    pub idProduct: u16,
    pub ff: *const signed short,
}

    static const signed short ff_rumble[] = {
    FF_RUMBLE,
    -1
    };
    static const signed short ff_joystick[] = {
    FF_CONSTANT,
    -1
    };
    static const signed short ff_joystick_ac[] = {
    FF_CONSTANT,
    FF_AUTOCENTER,
    -1
    };
    static const struct dev_type devices[] = {
    { 0x046d, 0xc211, ff_rumble },
    { 0x046d, 0xc219, ff_rumble },
    { 0x046d, 0xc283, ff_joystick },
    { 0x046d, 0xc286, ff_joystick_ac },
    { 0x046d, 0xc287, ff_joystick_ac },
    { 0x046d, 0xc293, ff_joystick },
    { 0x046d, 0xc295, ff_joystick },
    };
#[no_mangle]
unsafe extern "C" fn hid_lgff_play(dev: *mut input_dev, data: *mut c_void, effect: *mut ff_effect) -> c_int {
    static int hid_lgff_play(struct input_dev *dev, void *data, struct ff_effect *effect)
    {
    struct hid_device *hid = input_get_drvdata(dev);
    struct list_head *report_list = &hid.report_enum[HID_OUTPUT_REPORT].report_list;
    struct hid_report *report = list_entry(report_list.next, struct hid_report, list);
    int x, y;
    unsigned int left, right;

    switch (effect.type) {
    case FF_CONSTANT:
    x = effect.u.ramp.start_level + 0x7f;	/* 0x7f is center */
    y = effect.u.ramp.end_level + 0x7f;
    CLAMP(x);
    CLAMP(y);
    report.field[0].value[0] = 0x51;
    report.field[0].value[1] = 0x08;
    report.field[0].value[2] = x;
    report.field[0].value[3] = y;
    dbg_hid("(x, y)=(%04x, %04x)\n", x, y);
    hid_hw_request(hid, report, HID_REQ_SET_REPORT);
    break;
    case FF_RUMBLE:
    right = effect.u.rumble.strong_magnitude;
    left = effect.u.rumble.weak_magnitude;
    right = right * 0xff / 0xffff;
    left = left * 0xff / 0xffff;
    CLAMP(left);
    CLAMP(right);
    report.field[0].value[0] = 0x42;
    report.field[0].value[1] = 0x00;
    report.field[0].value[2] = left;
    report.field[0].value[3] = right;
    dbg_hid("(left, right)=(%04x, %04x)\n", left, right);
    hid_hw_request(hid, report, HID_REQ_SET_REPORT);
    break;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hid_lgff_set_autocenter(dev: *mut input_dev, magnitude: u16) {
    static void hid_lgff_set_autocenter(struct input_dev *dev, u16 magnitude)
    {
    struct hid_device *hid = input_get_drvdata(dev);
    struct list_head *report_list = &hid.report_enum[HID_OUTPUT_REPORT].report_list;
    struct hid_report *report = list_entry(report_list.next, struct hid_report, list);
    __s32 *value = report.field[0].value;
    magnitude = (magnitude >> 12) & 0xf;
// value++ = 0xfe;
// value++ = 0x0d;
// value++ = magnitude;   /* clockwise strength
// value++ = magnitude;   /* counter-clockwise strength
// value++ = 0x80;
// value++ = 0x00;
// value = 0x00;
    hid_hw_request(hid, report, HID_REQ_SET_REPORT);
    }
#[no_mangle]
pub unsafe extern "C" fn lgff_init(hid: *mut *mut hid_device) -> c_int {
    int lgff_init(struct hid_device* hid)
    {
    struct hid_input *hidinput;
    struct input_dev *dev;
    const signed short *ff_bits = ff_joystick;
    int error;
    int i;
    if (list_empty(&hid.inputs)) {
    hid_err(hid, "no inputs found\n");
    return -ENODEV;
    }
    hidinput = list_entry(hid.inputs.next, struct hid_input, list);
    dev = hidinput.input;
// Check that the report looks ok
    if (!hid_validate_values(hid, HID_OUTPUT_REPORT, 0, 0, 7))
    return -ENODEV;
    for (i = 0; i < ARRAY_SIZE(devices); i++) {
    if (dev.id.vendor == devices[i].idVendor &&
    dev.id.product == devices[i].idProduct) {
    ff_bits = devices[i].ff;
    break;
    }
    }
    for (i = 0; ff_bits[i] >= 0; i++)
    set_bit(ff_bits[i], dev.ffbit);
    error = input_ff_create_memless(dev, core::ptr::null_mut(), hid_lgff_play);
    if (error)
    return error;
    if ( test_bit(FF_AUTOCENTER, dev.ffbit) )
    dev.ff.set_autocenter = hid_lgff_set_autocenter;
    pr_info("Force feedback for Logitech force feedback devices by Johann Deneux <johann.deneux@it.uu.se>\n");
    return 0;
    }
