//! Automatically rewritten from C to Rust
//! Source: drivers/hid/hid-gaff.c
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
// Force feedback support for GreenAsia (Product ID 0x12) based devices
//
// The devices are distributed under various names and the same USB device ID
// can be used in many game controllers.
//
// 0e8f:0012 "GreenAsia Inc.    USB Joystick     "
// - tested with MANTA Warior MM816 and SpeedLink Strike2 SL-6635.
//
// Copyright (c) 2008 Lukasz Lubojanski <lukasz@lubojanski.info>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gaff_device {
    pub report: *mut hid_report,
}

    static int hid_gaff_play(struct input_dev *dev, void *data,
    struct ff_effect *effect)
    {
    struct hid_device *hid = input_get_drvdata(dev);
    struct gaff_device *gaff = data;
    int left, right;
    left = effect.u.rumble.strong_magnitude;
    right = effect.u.rumble.weak_magnitude;
    dbg_hid("called with 0x%04x 0x%04x", left, right);
    left = left * 0xfe / 0xffff;
    right = right * 0xfe / 0xffff;
    gaff.report.field[0].value[0] = 0x51;
    gaff.report.field[0].value[1] = 0x0;
    gaff.report.field[0].value[2] = right;
    gaff.report.field[0].value[3] = 0;
    gaff.report.field[0].value[4] = left;
    gaff.report.field[0].value[5] = 0;
    dbg_hid("running with 0x%02x 0x%02x", left, right);
    hid_hw_request(hid, gaff.report, HID_REQ_SET_REPORT);
    gaff.report.field[0].value[0] = 0xfa;
    gaff.report.field[0].value[1] = 0xfe;
    gaff.report.field[0].value[2] = 0x0;
    gaff.report.field[0].value[4] = 0x0;
    hid_hw_request(hid, gaff.report, HID_REQ_SET_REPORT);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn gaff_input_configured(hid: *mut hid_device, hidinput: *mut hid_input) -> c_int {
    static int gaff_input_configured(struct hid_device *hid, struct hid_input *hidinput)
    {
    struct gaff_device *gaff;
    struct hid_report *report;
    struct list_head *report_list =
    &hid.report_enum[HID_OUTPUT_REPORT].report_list;
    struct input_dev *dev = hidinput.input;
    int error;
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
    if (report.field[0].report_count < 6) {
    hid_err(hid, "not enough values in the field\n");
    return -ENODEV;
    }
    gaff = kzalloc_obj(struct gaff_device);
    if (!gaff)
    return -ENOMEM;
    gaff.report = report;
    set_bit(FF_RUMBLE, dev.ffbit);
    error = input_ff_create_memless(dev, gaff, hid_gaff_play);
    if (error) {
    kfree(gaff);
    return error;
    }
    gaff.report.field[0].value[0] = 0x51;
    gaff.report.field[0].value[1] = 0x00;
    gaff.report.field[0].value[2] = 0x00;
    gaff.report.field[0].value[3] = 0x00;
    hid_hw_request(hid, gaff.report, HID_REQ_SET_REPORT);
    gaff.report.field[0].value[0] = 0xfa;
    gaff.report.field[0].value[1] = 0xfe;
    hid_hw_request(hid, gaff.report, HID_REQ_SET_REPORT);
    hid_info(hid, "Force Feedback for GreenAsia 0x12 devices by Lukasz Lubojanski <lukasz@lubojanski.info>\n");
    return 0;
    }

    static inline int gaff_input_configured(struct hid_device *hdev,
    struct hid_input *hidinput)
    {
    return 0;
    }

    static const struct hid_device_id ga_devices[] = {
    { HID_USB_DEVICE(USB_VENDOR_ID_GREENASIA, 0x0012),  },
    { }
    };
    MODULE_DEVICE_TABLE(hid, ga_devices);
    static struct hid_driver ga_driver = {
    .name = "greenasia",
    .id_table = ga_devices,
    .input_configured = gaff_input_configured,
    };
    module_hid_driver(ga_driver);
    MODULE_DESCRIPTION("Force feedback support for GreenAsia (Product ID 0x12) based devices");
    MODULE_LICENSE("GPL");
