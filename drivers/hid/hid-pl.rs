//! Automatically rewritten from C to Rust
//! Source: drivers/hid/hid-pl.c
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
// Force feedback support for PantherLord/GreenAsia based devices
//
// The devices are distributed under various names and the same USB device ID
// can be used in both adapters and actual game controllers.
//
// 0810:0001 "Twin USB Joystick"
// - tested with PantherLord USB/PS2 2in1 Adapter
// - contains two reports, one for each port (HID_QUIRK_MULTI_INPUT)
//
// 0e8f:0003 "GreenAsia Inc.    USB Joystick     "
// - tested with König Gaming gamepad
//
// 0e8f:0003 "GASIA USB Gamepad"
// - another version of the König gamepad
//
// 0f30:0111 "Saitek Color Rumble Pad"
//
// Copyright (c) 2007, 2009 Anssi Hannula <anssi.hannula@gmail.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct plff_device {
    pub report: *mut hid_report,
    pub maxval: i32,
    pub strong: *mut i32,
    pub weak: *mut i32,
}

    static int hid_plff_play(struct input_dev *dev, void *data,
    struct ff_effect *effect)
    {
    struct hid_device *hid = input_get_drvdata(dev);
    struct plff_device *plff = data;
    int left, right;
    left = effect.u.rumble.strong_magnitude;
    right = effect.u.rumble.weak_magnitude;
    hid_dbg(dev, "called with 0x%04x 0x%04x", left, right);
    left = left * plff.maxval / 0xffff;
    right = right * plff.maxval / 0xffff;
// plff->strong = left;
// plff->weak = right;
    hid_dbg(dev, "running with 0x%02x 0x%02x", left, right);
    hid_hw_request(hid, plff.report, HID_REQ_SET_REPORT);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pl_input_configured(hid: *mut hid_device, hidinput: *mut hid_input) -> c_int {
    static int pl_input_configured(struct hid_device *hid, struct hid_input *hidinput)
    {
    struct plff_device *plff;
    struct hid_report *report;
    struct list_head *report_list =
    &hid.report_enum[HID_OUTPUT_REPORT].report_list;
    struct input_dev *dev = hidinput.input;
    int error;
    s32 maxval;
    s32 *strong;
    s32 *weak;
// The device contains one output report per physical device, all
    containing 1 field, which contains 4 ff00.0002 usages and 4 16bit
    absolute values.
    The input reports also contain a field which contains
    8 ff00.0001 usages and 8 boolean values. Their meaning is
    currently unknown.
    A version of the 0e8f:0003 exists that has all the values in
    separate fields and misses the extra input field, thus resembling
    Zeroplus (hid-zpff) devices.
//
    if (!list_is_first(&hidinput.list, &hid.inputs))
    return 0;
    report = list_first_entry_or_null(report_list, struct hid_report, list);
    if (!report) {
    hid_err(hid, "no output reports found\n");
    return -ENODEV;
    }
    if (report.maxfield < 1) {
    hid_err(hid, "no fields in the report\n");
    return -ENODEV;
    }
    maxval = 0x7f;
    if (report.field[0].report_count >= 4) {
    report.field[0].value[0] = 0x00;
    report.field[0].value[1] = 0x00;
    strong = &report.field[0].value[2];
    weak = &report.field[0].value[3];
    hid_dbg(hid, "detected single-field device");
    } else if (report.field[0].maxusage == 1 &&
    report.field[0].usage[0].hid ==
    (HID_UP_LED | 0x43) &&
    report.maxfield >= 4 &&
    report.field[0].report_count >= 1 &&
    report.field[1].report_count >= 1 &&
    report.field[2].report_count >= 1 &&
    report.field[3].report_count >= 1) {
    report.field[0].value[0] = 0x00;
    report.field[1].value[0] = 0x00;
    strong = &report.field[2].value[0];
    weak = &report.field[3].value[0];
    if (hid.vendor == USB_VENDOR_ID_JESS2)
    maxval = 0xff;
    hid_dbg(hid, "detected 4-field device");
    } else {
    hid_err(hid, "not enough fields or values\n");
    return -ENODEV;
    }
    plff = kzalloc_obj(struct plff_device);
    if (!plff)
    return -ENOMEM;
    dev = hidinput.input;
    set_bit(FF_RUMBLE, dev.ffbit);
    error = input_ff_create_memless(dev, plff, hid_plff_play);
    if (error) {
    kfree(plff);
    return error;
    }
    plff.report = report;
    plff.strong = strong;
    plff.weak = weak;
    plff.maxval = maxval;
// strong = 0x00;
// weak = 0x00;
    hid_hw_request(hid, plff.report, HID_REQ_SET_REPORT);
    return 0;
    }

    static inline int pl_input_configured(struct hid_device *hid,
    struct hid_input *hidinput)
    {
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn pl_probe(hdev: *mut hid_device, id: *const hid_device_id) -> c_int {
    static int pl_probe(struct hid_device *hdev, const struct hid_device_id *id)
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
    static const struct hid_device_id pl_devices[] = {
    { HID_USB_DEVICE(USB_VENDOR_ID_GAMERON, USB_DEVICE_ID_GAMERON_DUAL_PSX_ADAPTOR),
    .driver_data = 1 }, /* Twin USB Joystick */
    { HID_USB_DEVICE(USB_VENDOR_ID_GAMERON, USB_DEVICE_ID_GAMERON_DUAL_PCS_ADAPTOR),
    .driver_data = 1 }, /* Twin USB Joystick */
    { HID_USB_DEVICE(USB_VENDOR_ID_GREENASIA, 0x0003), },
    { HID_USB_DEVICE(USB_VENDOR_ID_JESS2, USB_DEVICE_ID_JESS2_COLOR_RUMBLE_PAD), },
    { }
    };
    MODULE_DEVICE_TABLE(hid, pl_devices);
    static struct hid_driver pl_driver = {
    .name = "pantherlord",
    .id_table = pl_devices,
    .probe = pl_probe,
    .input_configured = pl_input_configured,
    };
    module_hid_driver(pl_driver);
    MODULE_DESCRIPTION("Force feedback support for PantherLord/GreenAsia based devices");
    MODULE_LICENSE("GPL");
