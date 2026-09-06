//! Automatically rewritten from C to Rust
//! Source: drivers/hid/hid-zpff.c
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
// Force feedback support for Zeroplus based devices
//
// Copyright (c) 2005, 2006 Anssi Hannula <anssi.hannula@gmail.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct zpff_device {
    pub report: *mut hid_report,
}

    static int zpff_play(struct input_dev *dev, void *data,
    struct ff_effect *effect)
    {
    struct hid_device *hid = input_get_drvdata(dev);
    struct zpff_device *zpff = data;
    int left, right;
//
// The following is specified the other way around in the Zeroplus
// datasheet but the order below is correct for the XFX Executioner;
// however it is possible that the XFX Executioner is an exception
//
    left = effect.u.rumble.strong_magnitude;
    right = effect.u.rumble.weak_magnitude;
    dbg_hid("called with 0x%04x 0x%04x\n", left, right);
    left = left * 0x7f / 0xffff;
    right = right * 0x7f / 0xffff;
    zpff.report.field[2].value[0] = left;
    zpff.report.field[3].value[0] = right;
    dbg_hid("running with 0x%02x 0x%02x\n", left, right);
    hid_hw_request(hid, zpff.report, HID_REQ_SET_REPORT);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn zp_input_configured(hid: *mut hid_device, hidinput: *mut hid_input) -> c_int {
    static int zp_input_configured(struct hid_device *hid, struct hid_input *hidinput)
    {
    struct zpff_device *zpff;
    struct hid_report *report;
    struct input_dev *dev = hidinput.input;
    int i, error;
    if (!list_is_first(&hidinput.list, &hid.inputs))
    return 0;
    for (i = 0; i < 4; i++) {
    report = hid_validate_values(hid, HID_OUTPUT_REPORT, 0, i, 1);
    if (!report)
    return -ENODEV;
    }
    zpff = kzalloc_obj(struct zpff_device);
    if (!zpff)
    return -ENOMEM;
    zpff.report = report;
    set_bit(FF_RUMBLE, dev.ffbit);
    error = input_ff_create_memless(dev, zpff, zpff_play);
    if (error) {
    kfree(zpff);
    return error;
    }
    zpff.report.field[0].value[0] = 0x00;
    zpff.report.field[1].value[0] = 0x02;
    zpff.report.field[2].value[0] = 0x00;
    zpff.report.field[3].value[0] = 0x00;
    hid_hw_request(hid, zpff.report, HID_REQ_SET_REPORT);
    hid_info(hid, "force feedback for Zeroplus based devices by Anssi Hannula <anssi.hannula@gmail.com>\n");
    return 0;
    }

    static inline int zp_input_configured(struct hid_device *hid,
    struct hid_input *hidinput)
    {
    return 0;
    }

    static const struct hid_device_id zp_devices[] = {
    { HID_USB_DEVICE(USB_VENDOR_ID_ZEROPLUS, 0x0005) },
    { HID_USB_DEVICE(USB_VENDOR_ID_ZEROPLUS, 0x0030) },
    { }
    };
    MODULE_DEVICE_TABLE(hid, zp_devices);
    static struct hid_driver zp_driver = {
    .name = "zeroplus",
    .id_table = zp_devices,
    .input_configured = zp_input_configured,
    };
    module_hid_driver(zp_driver);
    MODULE_DESCRIPTION("Force feedback support for Zeroplus based devices");
    MODULE_LICENSE("GPL");
