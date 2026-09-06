//! Automatically rewritten from C to Rust
//! Source: drivers/hid/hid-monterey.c
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
// HID driver for some monterey "special" devices
//
// Copyright (c) 1999 Andreas Gal
// Copyright (c) 2000-2005 Vojtech Pavlik <vojtech@suse.cz>
// Copyright (c) 2005 Michael Haboustak <mike-@cinci.rr.com> for Concept2, Inc
// Copyright (c) 2006-2007 Jiri Kosina
// Copyright (c) 2008 Jiri Slaby
//

    static const __u8 *mr_report_fixup(struct hid_device *hdev, __u8 *rdesc,
    unsigned int *rsize)
    {
    if (*rsize >= 31 && rdesc[29] == 0x05 && rdesc[30] == 0x09) {
    hid_info(hdev, "fixing up button/consumer in HID report descriptor\n");
    rdesc[30] = 0x0c;
    }
    return rdesc;
    }

    EV_KEY, (c))
    static int mr_input_mapping(struct hid_device *hdev, struct hid_input *hi,
    struct hid_field *field, struct hid_usage *usage,
    unsigned long **bit, int *max)
    {
    if ((usage.hid & HID_USAGE_PAGE) != HID_UP_CONSUMER)
    return 0;
    switch (usage.hid & HID_USAGE) {
    case 0x156: mr_map_key_clear(KEY_WORDPROCESSOR);	break;
    case 0x157: mr_map_key_clear(KEY_SPREADSHEET);		break;
    case 0x158: mr_map_key_clear(KEY_PRESENTATION);		break;
    case 0x15c: mr_map_key_clear(KEY_STOP);			break;
    default:
    return 0;
    }
    return 1;
    }
    static const struct hid_device_id mr_devices[] = {
    { HID_USB_DEVICE(USB_VENDOR_ID_MONTEREY, USB_DEVICE_ID_GENIUS_KB29E) },
    { }
    };
    MODULE_DEVICE_TABLE(hid, mr_devices);
    static struct hid_driver mr_driver = {
    .name = "monterey",
    .id_table = mr_devices,
    .report_fixup = mr_report_fixup,
    .input_mapping = mr_input_mapping,
    };
    module_hid_driver(mr_driver);
    MODULE_DESCRIPTION("HID driver for some monterey \"special\" devices");
    MODULE_LICENSE("GPL");
