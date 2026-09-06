//! Automatically rewritten from C to Rust
//! Source: drivers/hid/hid-lg3ff.c
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
// Force feedback support for Logitech Flight System G940
//
// Copyright (c) 2009 Gary Stein <LordCnidarian@gmail.com>
//

//
// G940 Theory of Operation (from experimentation)
//
// There are 63 fields (only 3 of them currently used)
// 0 - seems to be command field
// 1 - 30 deal with the x axis
// 31 -60 deal with the y axis
//
// Field 1 is x axis constant force
// Field 31 is y axis constant force
//
// other interesting fields 1,2,3,4 on x axis
// (same for 31,32,33,34 on y axis)
//
// 0 0 127 127 makes the joystick autocenter hard
//
// 127 0 127 127 makes the joystick loose on the right,
// but stops all movemnt left
//
// -127 0 -127 -127 makes the joystick loose on the left,
// but stops all movement right
//
// 0 0 -127 -127 makes the joystick rattle very hard
//
// I'm sure these are effects that I don't know enough about them
//
    static int hid_lg3ff_play(struct input_dev *dev, void *data,
    struct ff_effect *effect)
    {
    struct hid_device *hid = input_get_drvdata(dev);
    struct list_head *report_list = &hid.report_enum[HID_OUTPUT_REPORT].report_list;
    struct hid_report *report = list_entry(report_list.next, struct hid_report, list);
    int x, y;
//
// Available values in the field should always be 63, but we only use up to
// 35. Instead, clear the entire area, however big it is.
//
    memset(report.field[0].value, 0,
    sizeof(__s32) * report.field[0].report_count);
    switch (effect.type) {
    case FF_CONSTANT:
//
// Already clamped in ff_memless
// 0 is center (different then other logitech)
//
    x = effect.u.ramp.start_level;
    y = effect.u.ramp.end_level;
// send command byte
    report.field[0].value[0] = 0x51;
//
// Sign backwards from other Force3d pro
// which get recast here in two's complement 8 bits
//
    report.field[0].value[1] = (unsigned char)(-x);
    report.field[0].value[31] = (unsigned char)(-y);
    hid_hw_request(hid, report, HID_REQ_SET_REPORT);
    break;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hid_lg3ff_set_autocenter(dev: *mut input_dev, magnitude: u16) {
    static void hid_lg3ff_set_autocenter(struct input_dev *dev, u16 magnitude)
    {
    struct hid_device *hid = input_get_drvdata(dev);
    struct list_head *report_list = &hid.report_enum[HID_OUTPUT_REPORT].report_list;
    struct hid_report *report = list_entry(report_list.next, struct hid_report, list);
//
// Auto Centering probed from device
// NOTE: deadman's switch on G940 must be covered
// for effects to work
//
    report.field[0].value[0] = 0x51;
    report.field[0].value[1] = 0x00;
    report.field[0].value[2] = 0x00;
    report.field[0].value[3] = 0x7F;
    report.field[0].value[4] = 0x7F;
    report.field[0].value[31] = 0x00;
    report.field[0].value[32] = 0x00;
    report.field[0].value[33] = 0x7F;
    report.field[0].value[34] = 0x7F;
    hid_hw_request(hid, report, HID_REQ_SET_REPORT);
    }
    static const signed short ff3_joystick_ac[] = {
    FF_CONSTANT,
    FF_AUTOCENTER,
    -1
    };
#[no_mangle]
pub unsafe extern "C" fn lg3ff_init(hid: *mut hid_device) -> c_int {
    int lg3ff_init(struct hid_device *hid)
    {
    struct hid_input *hidinput;
    struct input_dev *dev;
    const signed short *ff_bits = ff3_joystick_ac;
    int error;
    int i;
    if (list_empty(&hid.inputs)) {
    hid_err(hid, "no inputs found\n");
    return -ENODEV;
    }
    hidinput = list_entry(hid.inputs.next, struct hid_input, list);
    dev = hidinput.input;
// Check that the report looks ok
    if (!hid_validate_values(hid, HID_OUTPUT_REPORT, 0, 0, 35))
    return -ENODEV;
// Assume single fixed device G940
    for (i = 0; ff_bits[i] >= 0; i++)
    set_bit(ff_bits[i], dev.ffbit);
    error = input_ff_create_memless(dev, core::ptr::null_mut(), hid_lg3ff_play);
    if (error)
    return error;
    if (test_bit(FF_AUTOCENTER, dev.ffbit))
    dev.ff.set_autocenter = hid_lg3ff_set_autocenter;
    hid_info(hid, "Force feedback for Logitech Flight System G940 by Gary Stein <LordCnidarian@gmail.com>\n");
    return 0;
    }
