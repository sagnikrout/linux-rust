//! Automatically rewritten from C to Rust
//! Source: drivers/hid/hid-emsff.c
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
// Force feedback support for EMS Trio Linker Plus II
//
// Copyright (c) 2010 Ignaz Forster <ignaz.forster@gmx.de>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct emsff_device {
    pub report: *mut hid_report,
}

    static int emsff_play(struct input_dev *dev, void *data,
    struct ff_effect *effect)
    {
    struct hid_device *hid = input_get_drvdata(dev);
    struct emsff_device *emsff = data;
    int weak, strong;
    weak = effect.u.rumble.weak_magnitude;
    strong = effect.u.rumble.strong_magnitude;
    dbg_hid("called with 0x%04x 0x%04x\n", strong, weak);
    weak = weak * 0xff / 0xffff;
    strong = strong * 0xff / 0xffff;
    emsff.report.field[0].value[1] = weak;
    emsff.report.field[0].value[2] = strong;
    dbg_hid("running with 0x%02x 0x%02x\n", strong, weak);
    hid_hw_request(hid, emsff.report, HID_REQ_SET_REPORT);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ems_input_configured(hid: *mut hid_device, hidinput: *mut hid_input) -> c_int {
    static int ems_input_configured(struct hid_device *hid, struct hid_input *hidinput)
    {
    struct emsff_device *emsff;
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
    if (report.field[0].report_count < 7) {
    hid_err(hid, "not enough values in the field\n");
    return -ENODEV;
    }
    emsff = kzalloc_obj(struct emsff_device);
    if (!emsff)
    return -ENOMEM;
    emsff.report = report;
    set_bit(FF_RUMBLE, dev.ffbit);
    error = input_ff_create_memless(dev, emsff, emsff_play);
    if (error) {
    kfree(emsff);
    return error;
    }
    emsff.report.field[0].value[0] = 0x01;
    emsff.report.field[0].value[1] = 0x00;
    emsff.report.field[0].value[2] = 0x00;
    emsff.report.field[0].value[3] = 0x00;
    emsff.report.field[0].value[4] = 0x00;
    emsff.report.field[0].value[5] = 0x00;
    emsff.report.field[0].value[6] = 0x00;
    hid_hw_request(hid, emsff.report, HID_REQ_SET_REPORT);
    hid_info(hid, "force feedback for EMS based devices by Ignaz Forster <ignaz.forster@gmx.de>\n");
    return 0;
    }
    static const struct hid_device_id ems_devices[] = {
    { HID_USB_DEVICE(USB_VENDOR_ID_EMS, USB_DEVICE_ID_EMS_TRIO_LINKER_PLUS_II) },
    { }
    };
    MODULE_DEVICE_TABLE(hid, ems_devices);
    static struct hid_driver ems_driver = {
    .name = "hkems",
    .id_table = ems_devices,
    .input_configured = ems_input_configured,
    };
    module_hid_driver(ems_driver);
    MODULE_DESCRIPTION("Force feedback support for EMS Trio Linker Plus II");
    MODULE_LICENSE("GPL");
