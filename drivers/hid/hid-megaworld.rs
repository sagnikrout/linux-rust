//! Automatically rewritten from C to Rust
//! Source: drivers/hid/hid-megaworld.c
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
// Vibration support for Mega World controllers
//
// Copyright 2022 Frank Zago
//
// Derived from hid-zpff.c:
// Copyright (c) 2005, 2006 Anssi Hannula <anssi.hannula@gmail.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mwctrl_device {
    pub report: *mut hid_report,
    pub weak: *mut i32,
    pub strong: *mut i32,
}

    static int mwctrl_play(struct input_dev *dev, void *data,
    struct ff_effect *effect)
    {
    struct hid_device *hid = input_get_drvdata(dev);
    struct mwctrl_device *mwctrl = data;
// mwctrl->strong = effect->u.rumble.strong_magnitude >> 8;
// mwctrl->weak = effect->u.rumble.weak_magnitude >> 8;
    hid_hw_request(hid, mwctrl.report, HID_REQ_SET_REPORT);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mwctrl_input_configured(hid: *mut hid_device, hidinput: *mut hid_input) -> c_int {
    static int mwctrl_input_configured(struct hid_device *hid, struct hid_input *hidinput)
    {
    struct mwctrl_device *mwctrl;
    struct hid_report *report;
    struct input_dev *dev = hidinput.input;
    int error;
    int i;
    if (!list_is_first(&hidinput.list, &hid.inputs))
    return 0;
    for (i = 0; i < 4; i++) {
    report = hid_validate_values(hid, HID_OUTPUT_REPORT, 0, i, 1);
    if (!report)
    return -ENODEV;
    }
    mwctrl = kzalloc_obj(struct mwctrl_device);
    if (!mwctrl)
    return -ENOMEM;
    mwctrl.report = report;
// Field 0 is always 2, and field 1 is always 0. The original
// windows driver has a 5 bytes command, where the 5th byte is
// a repeat of the 3rd byte, however the device has only 4
// fields. It could be a bug in the driver, or there is a
// different device that needs it.
//
    report.field[0].value[0] = 0x02;
    mwctrl.strong = &report.field[2].value[0];
    mwctrl.weak = &report.field[3].value[0];
    set_bit(FF_RUMBLE, dev.ffbit);
    error = input_ff_create_memless(dev, mwctrl, mwctrl_play);
    if (error) {
    kfree(mwctrl);
    return error;
    }
    return 0;
    }
    static const struct hid_device_id mwctrl_devices[] = {
    { HID_USB_DEVICE(USB_VENDOR_MEGAWORLD,
    USB_DEVICE_ID_MEGAWORLD_GAMEPAD) },
    { }
    };
    MODULE_DEVICE_TABLE(hid, mwctrl_devices);
    static struct hid_driver mwctrl_driver = {
    .name = "megaworld",
    .id_table = mwctrl_devices,
    .input_configured = mwctrl_input_configured,
    };
    module_hid_driver(mwctrl_driver);
    MODULE_DESCRIPTION("Vibration support for Mega World controllers");
    MODULE_LICENSE("GPL");
