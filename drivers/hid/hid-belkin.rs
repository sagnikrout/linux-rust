//! Automatically rewritten from C to Rust
//! Source: drivers/hid/hid-belkin.c
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
// HID driver for some belkin "special" devices
//
// Copyright (c) 1999 Andreas Gal
// Copyright (c) 2000-2005 Vojtech Pavlik <vojtech@suse.cz>
// Copyright (c) 2005 Michael Haboustak <mike-@cinci.rr.com> for Concept2, Inc
// Copyright (c) 2006-2007 Jiri Kosina
// Copyright (c) 2008 Jiri Slaby
//

pub const BELKIN_HIDDEV: c_uint = 0x01;
pub const BELKIN_WKBD: c_uint = 0x02;

    EV_KEY, (c))
    static int belkin_input_mapping(struct hid_device *hdev, struct hid_input *hi,
    struct hid_field *field, struct hid_usage *usage,
    unsigned long **bit, int *max)
    {
    const struct hid_device_id *id = hid_get_drvdata(hdev);
    let mut quirks: c_ulong = id.driver_data;
    if ((usage.hid & HID_USAGE_PAGE) != HID_UP_CONSUMER ||
    !(quirks & BELKIN_WKBD))
    return 0;
    switch (usage.hid & HID_USAGE) {
    case 0x03a: belkin_map_key_clear(KEY_SOUND);		break;
    case 0x03b: belkin_map_key_clear(KEY_CAMERA);		break;
    case 0x03c: belkin_map_key_clear(KEY_DOCUMENTS);	break;
    default:
    return 0;
    }
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn belkin_probe(hdev: *mut hid_device, id: *const hid_device_id) -> c_int {
    static int belkin_probe(struct hid_device *hdev, const struct hid_device_id *id)
    {
    let mut quirks: c_ulong = id.driver_data;
    int ret;
    hid_set_drvdata(hdev, (void *)id);
    ret = hid_parse(hdev);
    if (ret) {
    hid_err(hdev, "parse failed\n");
    goto err_free;
    }
    ret = hid_hw_start(hdev, HID_CONNECT_DEFAULT |
    ((quirks & BELKIN_HIDDEV) ? HID_CONNECT_HIDDEV_FORCE : 0));
    if (ret) {
    hid_err(hdev, "hw start failed\n");
    goto err_free;
    }
    return 0;
    err_free:
    return ret;
    }
    static const struct hid_device_id belkin_devices[] = {
    { HID_USB_DEVICE(USB_VENDOR_ID_BELKIN, USB_DEVICE_ID_FLIP_KVM),
    .driver_data = BELKIN_HIDDEV },
    { HID_USB_DEVICE(USB_VENDOR_ID_LABTEC, USB_DEVICE_ID_LABTEC_WIRELESS_KEYBOARD),
    .driver_data = BELKIN_WKBD },
    { }
    };
    MODULE_DEVICE_TABLE(hid, belkin_devices);
    static struct hid_driver belkin_driver = {
    .name = "belkin",
    .id_table = belkin_devices,
    .input_mapping = belkin_input_mapping,
    .probe = belkin_probe,
    };
    module_hid_driver(belkin_driver);
    MODULE_DESCRIPTION("HID driver for some belkin \"special\" devices");
    MODULE_LICENSE("GPL");
