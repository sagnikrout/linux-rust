//! Automatically rewritten from C to Rust
//! Source: drivers/hid/hid-sunplus.c
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
// HID driver for some sunplus "special" devices
//
// Copyright (c) 1999 Andreas Gal
// Copyright (c) 2000-2005 Vojtech Pavlik <vojtech@suse.cz>
// Copyright (c) 2005 Michael Haboustak <mike-@cinci.rr.com> for Concept2, Inc
// Copyright (c) 2006-2007 Jiri Kosina
// Copyright (c) 2008 Jiri Slaby
//

    static const __u8 *sp_report_fixup(struct hid_device *hdev, __u8 *rdesc,
    unsigned int *rsize)
    {
    if (*rsize >= 112 && rdesc[104] == 0x26 && rdesc[105] == 0x80 &&
    rdesc[106] == 0x03) {
    hid_info(hdev, "fixing up Sunplus Wireless Desktop report descriptor\n");
    rdesc[105] = rdesc[110] = 0x03;
    rdesc[106] = rdesc[111] = 0x21;
    }
    return rdesc;
    }

    EV_KEY, (c))
    static int sp_input_mapping(struct hid_device *hdev, struct hid_input *hi,
    struct hid_field *field, struct hid_usage *usage,
    unsigned long **bit, int *max)
    {
    if ((usage.hid & HID_USAGE_PAGE) != HID_UP_CONSUMER)
    return 0;
    switch (usage.hid & HID_USAGE) {
    case 0x2003: sp_map_key_clear(KEY_ZOOMIN);		break;
    case 0x2103: sp_map_key_clear(KEY_ZOOMOUT);	break;
    default:
    return 0;
    }
    return 1;
    }
    static const struct hid_device_id sp_devices[] = {
    { HID_USB_DEVICE(USB_VENDOR_ID_SUNPLUS, USB_DEVICE_ID_SUNPLUS_WDESKTOP) },
    { }
    };
    MODULE_DEVICE_TABLE(hid, sp_devices);
    static struct hid_driver sp_driver = {
    .name = "sunplus",
    .id_table = sp_devices,
    .report_fixup = sp_report_fixup,
    .input_mapping = sp_input_mapping,
    };
    module_hid_driver(sp_driver);
    MODULE_DESCRIPTION("HID driver for some sunplus \"special\" devices");
    MODULE_LICENSE("GPL");
