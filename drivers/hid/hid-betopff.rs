//! Automatically rewritten from C to Rust
//! Source: drivers/hid/hid-betopff.c
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
// Force feedback support for Betop based devices
//
// The devices are distributed under various names and the same USB device ID
// can be used in both adapters and actual game controllers.
//
// 0x11c2:0x2208 "BTP2185 BFM mode Joystick"
// - tested with BTP2185 BFM Mode.
//
// 0x11C0:0x5506 "BTP2185 PC mode Joystick"
// - tested with BTP2185 PC Mode.
//
// 0x8380:0x1850 "BTP2185 V2 PC mode USB Gamepad"
// - tested with BTP2185 PC Mode with another version.
//
// 0x20bc:0x5500 "BTP2185 V2 BFM mode Joystick"
// - tested with BTP2171s.
// Copyright (c) 2014 Huang Bo <huangbobupt@163.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct betopff_device {
    pub report: *mut hid_report,
}

    static int hid_betopff_play(struct input_dev *dev, void *data,
    struct ff_effect *effect)
    {
    struct hid_device *hid = input_get_drvdata(dev);
    struct betopff_device *betopff = data;
    __u16 left, right;
    left = effect.u.rumble.strong_magnitude;
    right = effect.u.rumble.weak_magnitude;
    betopff.report.field[2].value[0] = left / 256;
    betopff.report.field[3].value[0] = right / 256;
    hid_hw_request(hid, betopff.report, HID_REQ_SET_REPORT);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn betop_input_configured(hid: *mut hid_device, hidinput: *mut hid_input) -> c_int {
    static int betop_input_configured(struct hid_device *hid, struct hid_input *hidinput)
    {
    struct betopff_device *betopff;
    struct hid_report *report;
    struct list_head *report_list =
    &hid.report_enum[HID_OUTPUT_REPORT].report_list;
    struct input_dev *dev = hidinput.input;
    int error;
    int i, j;
    if (!list_is_first(&hidinput.list, &hid.inputs))
    return 0;
    report = list_first_entry_or_null(report_list, struct hid_report, list);
    if (!report) {
    hid_err(hid, "no output reports found\n");
    return -ENODEV;
    }
//
// Actually there are 4 fields for 4 Bytes as below:
// -----------------------------------------
// Byte0  Byte1  Byte2	  Byte3
// 0x00   0x00   left_motor right_motor
// -----------------------------------------
// Do init them with default value.
//
    if (report.maxfield < 4) {
    hid_err(hid, "not enough fields in the report: %d\n",
    report.maxfield);
    return -ENODEV;
    }
    for (i = 0; i < report.maxfield; i++) {
    if (report.field[i].report_count < 1) {
    hid_err(hid, "no values in the field\n");
    return -ENODEV;
    }
    for (j = 0; j < report.field[i].report_count; j++) {
    report.field[i].value[j] = 0x00;
    }
    }
    betopff = kzalloc_obj(*betopff);
    if (!betopff)
    return -ENOMEM;
    betopff.report = report;
    set_bit(FF_RUMBLE, dev.ffbit);
    error = input_ff_create_memless(dev, betopff, hid_betopff_play);
    if (error) {
    kfree(betopff);
    return error;
    }
    hid_hw_request(hid, betopff.report, HID_REQ_SET_REPORT);
    hid_info(hid, "Force feedback for betop devices by huangbo <huangbobupt@163.com>\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn betop_probe(hdev: *mut hid_device, id: *const hid_device_id) -> c_int {
    static int betop_probe(struct hid_device *hdev, const struct hid_device_id *id)
    {
    int ret;
    if (id.driver_data)
    hdev.quirks |= HID_QUIRK_MULTI_INPUT;
    ret = hid_parse(hdev);
    if (ret) {
    hid_err(hdev, "parse failed\n");
    return ret;
    }
    ret = hid_hw_start(hdev, HID_CONNECT_DEFAULT);
    if (ret) {
    hid_err(hdev, "hw start failed\n");
    return ret;
    }
    return 0;
    }
    static const struct hid_device_id betop_devices[] = {
    { HID_USB_DEVICE(USB_VENDOR_ID_BETOP_2185BFM, 0x2208) },
    { HID_USB_DEVICE(USB_VENDOR_ID_BETOP_2185PC, 0x5506) },
    { HID_USB_DEVICE(USB_VENDOR_ID_BETOP_2185V2PC, 0x1850) },
    { HID_USB_DEVICE(USB_VENDOR_ID_BETOP_2185V2BFM, 0x5500) },
    { }
    };
    MODULE_DEVICE_TABLE(hid, betop_devices);
    static struct hid_driver betop_driver = {
    .name = "betop",
    .id_table = betop_devices,
    .probe = betop_probe,
    .input_configured = betop_input_configured,
    };
    module_hid_driver(betop_driver);
    MODULE_DESCRIPTION("Force feedback support for Betop based devices");
    MODULE_LICENSE("GPL");
