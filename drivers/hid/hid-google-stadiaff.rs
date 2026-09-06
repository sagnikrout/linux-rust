//! Automatically rewritten from C to Rust
//! Source: drivers/hid/hid-google-stadiaff.c
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
// Stadia controller rumble support.
//
// Copyright 2023 Google LLC
//

pub const STADIA_FF_REPORT_ID: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stadiaff_device {
    pub hid: *mut hid_device,
    pub report: *mut hid_report,
    pub magnitudes: u32,
    pub work: work_struct,
}

#[no_mangle]
unsafe extern "C" fn stadiaff_work(work: *mut work_struct) {
    static void stadiaff_work(struct work_struct *work)
    {
    struct stadiaff_device *stadiaff =
    container_of(work, struct stadiaff_device, work);
    struct hid_field *rumble_field = stadiaff.report.field[0];
    let mut mags: u32 = READ_ONCE(stadiaff.magnitudes);
    rumble_field.value[0] = mags & 0xffff;
    rumble_field.value[1] = (mags >> 16) & 0xffff;
    hid_hw_request(stadiaff.hid, stadiaff.report, HID_REQ_SET_REPORT);
    }
    static int stadiaff_play(struct input_dev *dev, void *data,
    struct ff_effect *effect)
    {
    struct hid_device *hid = input_get_drvdata(dev);
    struct stadiaff_device *stadiaff = hid_get_drvdata(hid);
    u32 mags = (u32)effect.u.rumble.strong_magnitude |
    ((u32)effect.u.rumble.weak_magnitude << 16);
    WRITE_ONCE(stadiaff.magnitudes, mags);
    schedule_work(&stadiaff.work);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn stadia_input_open(dev: *mut input_dev) -> c_int {
    static int stadia_input_open(struct input_dev *dev)
    {
    struct hid_device *hid = input_get_drvdata(dev);
    struct stadiaff_device *stadiaff = hid_get_drvdata(hid);
    int error;
    error = hid_hw_open(hid);
    if (error)
    return error;
    enable_work(&stadiaff.work);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn stadia_input_close(dev: *mut input_dev) {
    static void stadia_input_close(struct input_dev *dev)
    {
    struct hid_device *hid = input_get_drvdata(dev);
    struct stadiaff_device *stadiaff = hid_get_drvdata(hid);
    WRITE_ONCE(stadiaff.magnitudes, 0);
    stadiaff_work(&stadiaff.work);
    disable_work_sync(&stadiaff.work);
    hid_hw_close(hid);
    }
#[no_mangle]
unsafe extern "C" fn stadia_input_configured(hid: *mut hid_device, hidinput: *mut hid_input) -> c_int {
    static int stadia_input_configured(struct hid_device *hid, struct hid_input *hidinput)
    {
    struct stadiaff_device *stadiaff;
    struct hid_report *report;
    struct input_dev *dev = hidinput.input;
    int error;
    if (!list_is_first(&hidinput.list, &hid.inputs))
    return 0;
    report = hid_validate_values(hid, HID_OUTPUT_REPORT,
    STADIA_FF_REPORT_ID, 0, 2);
    if (!report)
    return -ENODEV;
    stadiaff = devm_kzalloc(&hid.dev, sizeof(struct stadiaff_device),
    GFP_KERNEL);
    if (!stadiaff)
    return -ENOMEM;
    hid_set_drvdata(hid, stadiaff);
    input_set_capability(dev, EV_FF, FF_RUMBLE);
    error = input_ff_create_memless(dev, core::ptr::null_mut(), stadiaff_play);
    if (error)
    return error;
    stadiaff.hid = hid;
    stadiaff.report = report;
    INIT_WORK(&stadiaff.work, stadiaff_work);
    disable_work_sync(&stadiaff.work);
    dev.open = stadia_input_open;
    dev.close = stadia_input_close;
    hid_info(hid, "Force Feedback for Google Stadia controller\n");
    return 0;
    }
    static const struct hid_device_id stadia_devices[] = {
    { HID_USB_DEVICE(USB_VENDOR_ID_GOOGLE, USB_DEVICE_ID_GOOGLE_STADIA) },
    { HID_BLUETOOTH_DEVICE(USB_VENDOR_ID_GOOGLE, USB_DEVICE_ID_GOOGLE_STADIA) },
    { }
    };
    MODULE_DEVICE_TABLE(hid, stadia_devices);
    static struct hid_driver stadia_driver = {
    .name = "stadia",
    .id_table = stadia_devices,
    .input_configured = stadia_input_configured,
    };
    module_hid_driver(stadia_driver);
    MODULE_DESCRIPTION("Google Stadia controller rumble support.");
    MODULE_LICENSE("GPL");
