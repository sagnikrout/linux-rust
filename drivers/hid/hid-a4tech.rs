//! Automatically rewritten from C to Rust
//! Source: drivers/hid/hid-a4tech.c
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
// HID driver for some a4tech "special" devices
//
// Copyright (c) 1999 Andreas Gal
// Copyright (c) 2000-2005 Vojtech Pavlik <vojtech@suse.cz>
// Copyright (c) 2005 Michael Haboustak <mike-@cinci.rr.com> for Concept2, Inc
// Copyright (c) 2006-2007 Jiri Kosina
// Copyright (c) 2008 Jiri Slaby
//

pub const A4_2WHEEL_MOUSE_HACK_7: c_uint = 0x01;
pub const A4_2WHEEL_MOUSE_HACK_B8: c_uint = 0x02;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct a4tech_sc {
    pub quirks: c_ulong,
    pub hw_wheel: c_uint,
    pub delayed_value: __s32,
}

    static int a4_input_mapping(struct hid_device *hdev, struct hid_input *hi,
    struct hid_field *field, struct hid_usage *usage,
    unsigned long **bit, int *max)
    {
    struct a4tech_sc *a4 = hid_get_drvdata(hdev);
    if (a4.quirks & A4_2WHEEL_MOUSE_HACK_B8 &&
    usage.hid == A4_WHEEL_ORIENTATION) {
//
// We do not want to have this usage mapped to anything as it's
// nonstandard and doesn't really behave like an HID report.
// It's only selecting the orientation (vertical/horizontal) of
// the previous mouse wheel report. The input_events will be
// generated once both reports are recorded in a4_event().
//
    return -1;
    }
    return 0;
    }
    static int a4_input_mapped(struct hid_device *hdev, struct hid_input *hi,
    struct hid_field *field, struct hid_usage *usage,
    unsigned long **bit, int *max)
    {
    struct a4tech_sc *a4 = hid_get_drvdata(hdev);
    if (usage.type == EV_REL && usage.code == REL_WHEEL_HI_RES) {
    set_bit(REL_HWHEEL, *bit);
    set_bit(REL_HWHEEL_HI_RES, *bit);
    }
    if ((a4.quirks & A4_2WHEEL_MOUSE_HACK_7) && usage.hid == 0x00090007)
    return -1;
    return 0;
    }
    static int a4_event(struct hid_device *hdev, struct hid_field *field,
    struct hid_usage *usage, __s32 value)
    {
    struct a4tech_sc *a4 = hid_get_drvdata(hdev);
    struct input_dev *input;
    if (!(hdev.claimed & HID_CLAIMED_INPUT) || !field.hidinput)
    return 0;
    input = field.hidinput.input;
    if (a4.quirks & A4_2WHEEL_MOUSE_HACK_B8) {
    if (usage.type == EV_REL && usage.code == REL_WHEEL_HI_RES) {
    a4.delayed_value = value;
    return 1;
    }
    if (usage.hid == A4_WHEEL_ORIENTATION) {
    input_event(input, EV_REL, value ? REL_HWHEEL :
    REL_WHEEL, a4.delayed_value);
    input_event(input, EV_REL, value ? REL_HWHEEL_HI_RES :
    REL_WHEEL_HI_RES, a4.delayed_value * 120);
    return 1;
    }
    }
    if ((a4.quirks & A4_2WHEEL_MOUSE_HACK_7) && usage.hid == 0x00090007) {
    a4.hw_wheel = !!value;
    return 1;
    }
    if (usage.code == REL_WHEEL_HI_RES && a4.hw_wheel) {
    input_event(input, usage.type, REL_HWHEEL, value);
    input_event(input, usage.type, REL_HWHEEL_HI_RES, value * 120);
    return 1;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn a4_probe(hdev: *mut hid_device, id: *const hid_device_id) -> c_int {
    static int a4_probe(struct hid_device *hdev, const struct hid_device_id *id)
    {
    struct a4tech_sc *a4;
    int ret;
    a4 = devm_kzalloc(&hdev.dev, sizeof(*a4), GFP_KERNEL);
    if (a4 == core::ptr::null_mut()) {
    hid_err(hdev, "can't alloc device descriptor\n");
    return -ENOMEM;
    }
    a4.quirks = id.driver_data;
    hid_set_drvdata(hdev, a4);
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
    static const struct hid_device_id a4_devices[] = {
    { HID_USB_DEVICE(USB_VENDOR_ID_A4TECH, USB_DEVICE_ID_A4TECH_WCP32PU),
    .driver_data = A4_2WHEEL_MOUSE_HACK_7 },
    { HID_USB_DEVICE(USB_VENDOR_ID_A4TECH, USB_DEVICE_ID_A4TECH_X5_005D),
    .driver_data = A4_2WHEEL_MOUSE_HACK_B8 },
    { HID_USB_DEVICE(USB_VENDOR_ID_A4TECH, USB_DEVICE_ID_A4TECH_RP_649),
    .driver_data = A4_2WHEEL_MOUSE_HACK_B8 },
    { HID_USB_DEVICE(USB_VENDOR_ID_A4TECH, USB_DEVICE_ID_A4TECH_NB_95),
    .driver_data = A4_2WHEEL_MOUSE_HACK_B8 },
    { }
    };
    MODULE_DEVICE_TABLE(hid, a4_devices);
    static struct hid_driver a4_driver = {
    .name = "a4tech",
    .id_table = a4_devices,
    .input_mapping = a4_input_mapping,
    .input_mapped = a4_input_mapped,
    .event = a4_event,
    .probe = a4_probe,
    };
    module_hid_driver(a4_driver);
    MODULE_DESCRIPTION("HID driver for some a4tech \"special\" devices");
    MODULE_LICENSE("GPL");
