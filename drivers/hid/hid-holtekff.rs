//! Automatically rewritten from C to Rust
//! Source: drivers/hid/hid-holtekff.c
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
// Force feedback support for Holtek On Line Grip based gamepads
//
// These include at least a Brazilian "Clone Joypad Super Power Fire"
// which uses vendor ID 0x1241 and identifies as "HOLTEK On Line Grip".
//
// Copyright (c) 2011 Anssi Hannula <anssi.hannula@iki.fi>
//

//
// These commands and parameters are currently known:
//
// byte 0: command id:
// 01  set effect parameters
// 02  play specified effect
// 03  stop specified effect
// 04  stop all effects
// 06  stop all effects
// (the difference between 04 and 06 isn't known; win driver
// sends 06,04 on application init, and 06 otherwise)
//
// Commands 01 and 02 need to be sent as pairs, i.e. you need to send 01
// before each 02.
//
// The rest of the bytes are parameters. Command 01 takes all of them, and
// commands 02,03 take only the effect id.
//
// byte 1:
// bits 0-3: effect id:
// 1: very strong rumble
// 2: periodic rumble, short intervals
// 3: very strong rumble
// 4: periodic rumble, long intervals
// 5: weak periodic rumble, long intervals
// 6: weak periodic rumble, short intervals
// 7: periodic rumble, short intervals
// 8: strong periodic rumble, short intervals
// 9: very strong rumble
// a: causes an error
// b: very strong periodic rumble, very short intervals
// c-f: nothing
// bit 6: right (weak) motor enabled
// bit 7: left (strong) motor enabled
//
// bytes 2-3:  time in milliseconds, big-endian
// bytes 5-6:  unknown (win driver seems to use at least 10e0 with effect 1
// and 0014 with effect 6)
// byte 7:
// bits 0-3: effect magnitude
//
pub const HOLTEKFF_MSG_LENGTH: c_int = 7;
    static const u8 start_effect_1[] = { 0x02, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00 };
    static const u8 stop_all4[] =	   { 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00 };
    static const u8 stop_all6[] =	   { 0x06, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00 };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct holtekff_device {
    pub field: *mut hid_field,
}

    static void holtekff_send(struct holtekff_device *holtekff,
    struct hid_device *hid,
    const u8 data[HOLTEKFF_MSG_LENGTH])
    {
    int i;
    for (i = 0; i < HOLTEKFF_MSG_LENGTH; i++) {
    holtekff.field.value[i] = data[i];
    }
    dbg_hid("sending %7ph\n", data);
    hid_hw_request(hid, holtekff.field.report, HID_REQ_SET_REPORT);
    }
    static int holtekff_play(struct input_dev *dev, void *data,
    struct ff_effect *effect)
    {
    struct hid_device *hid = input_get_drvdata(dev);
    struct holtekff_device *holtekff = data;
    int left, right;
// effect type 1, length 65535 msec
    u8 buf[HOLTEKFF_MSG_LENGTH] =
    { 0x01, 0x01, 0xff, 0xff, 0x10, 0xe0, 0x00 };
    left = effect.u.rumble.strong_magnitude;
    right = effect.u.rumble.weak_magnitude;
    dbg_hid("called with 0x%04x 0x%04x\n", left, right);
    if (!left && !right) {
    holtekff_send(holtekff, hid, stop_all6);
    return 0;
    }
    if (left)
    buf[1] |= 0x80;
    if (right)
    buf[1] |= 0x40;
// The device takes a single magnitude, so we just sum them up.
    buf[6] = min(0xf, (left >> 12) + (right >> 12));
    holtekff_send(holtekff, hid, buf);
    holtekff_send(holtekff, hid, start_effect_1);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn holtek_input_configured(hid: *mut hid_device, hidinput: *mut hid_input) -> c_int {
    static int holtek_input_configured(struct hid_device *hid, struct hid_input *hidinput)
    {
    struct holtekff_device *holtekff;
    struct hid_report *report;
    struct list_head *report_list =
    &hid.report_enum[HID_OUTPUT_REPORT].report_list;
    struct input_dev *dev = hidinput.input;
    int error;
    if (!list_is_first(&hidinput.list, &hid.inputs))
    return 0;
    report = list_first_entry_or_null(report_list, struct hid_report, list);
    if (!report) {
    hid_err(hid, "no output report found\n");
    return -ENODEV;
    }
    if (report.maxfield < 1 || report.field[0].report_count != 7) {
    hid_err(hid, "unexpected output report layout\n");
    return -ENODEV;
    }
    holtekff = kzalloc_obj(*holtekff);
    if (!holtekff)
    return -ENOMEM;
    set_bit(FF_RUMBLE, dev.ffbit);
    holtekff.field = report.field[0];
// initialize the same way as win driver does
    holtekff_send(holtekff, hid, stop_all4);
    holtekff_send(holtekff, hid, stop_all6);
    error = input_ff_create_memless(dev, holtekff, holtekff_play);
    if (error) {
    kfree(holtekff);
    return error;
    }
    hid_info(hid, "Force feedback for Holtek On Line Grip based devices by Anssi Hannula <anssi.hannula@iki.fi>\n");
    return 0;
    }

    static inline int holtek_input_configured(struct hid_device *hid,
    struct hid_input *hidinput)
    {
    return 0;
    }

    static const struct hid_device_id holtek_devices[] = {
    { HID_USB_DEVICE(USB_VENDOR_ID_HOLTEK, USB_DEVICE_ID_HOLTEK_ON_LINE_GRIP) },
    { }
    };
    MODULE_DEVICE_TABLE(hid, holtek_devices);
    static struct hid_driver holtek_driver = {
    .name = "holtek",
    .id_table = holtek_devices,
    .input_configured = holtek_input_configured,
    };
    module_hid_driver(holtek_driver);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Anssi Hannula <anssi.hannula@iki.fi>");
    MODULE_DESCRIPTION("Force feedback support for Holtek On Line Grip based devices");
