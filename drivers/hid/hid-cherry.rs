//! Automatically rewritten from C to Rust
//! Source: drivers/hid/hid-cherry.c
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
// HID driver for some cherry "special" devices
//
// Copyright (c) 1999 Andreas Gal
// Copyright (c) 2000-2005 Vojtech Pavlik <vojtech@suse.cz>
// Copyright (c) 2005 Michael Haboustak <mike-@cinci.rr.com> for Concept2, Inc
// Copyright (c) 2006-2007 Jiri Kosina
// Copyright (c) 2008 Jiri Slaby
//

//
// Cherry Cymotion keyboard have an invalid HID report descriptor,
// that needs fixing before we can parse it.
//
    static const __u8 *ch_report_fixup(struct hid_device *hdev, __u8 *rdesc,
    unsigned int *rsize)
    {
    if (*rsize >= 18 && rdesc[11] == 0x3c && rdesc[12] == 0x02) {
    hid_info(hdev, "fixing up Cherry Cymotion report descriptor\n");
    rdesc[11] = rdesc[16] = 0xff;
    rdesc[12] = rdesc[17] = 0x03;
    }
    return rdesc;
    }

    EV_KEY, (c))
    static int ch_input_mapping(struct hid_device *hdev, struct hid_input *hi,
    struct hid_field *field, struct hid_usage *usage,
    unsigned long **bit, int *max)
    {
    if ((usage.hid & HID_USAGE_PAGE) != HID_UP_CONSUMER)
    return 0;
    switch (usage.hid & HID_USAGE) {
    case 0x301: ch_map_key_clear(KEY_PROG1);	break;
    case 0x302: ch_map_key_clear(KEY_PROG2);	break;
    case 0x303: ch_map_key_clear(KEY_PROG3);	break;
    default:
    return 0;
    }
    return 1;
    }
    static const struct hid_device_id ch_devices[] = {
    { HID_USB_DEVICE(USB_VENDOR_ID_CHERRY, USB_DEVICE_ID_CHERRY_CYMOTION) },
    { HID_USB_DEVICE(USB_VENDOR_ID_CHERRY, USB_DEVICE_ID_CHERRY_CYMOTION_SOLAR) },
    { }
    };
    MODULE_DEVICE_TABLE(hid, ch_devices);
    static struct hid_driver ch_driver = {
    .name = "cherry",
    .id_table = ch_devices,
    .report_fixup = ch_report_fixup,
    .input_mapping = ch_input_mapping,
    };
    module_hid_driver(ch_driver);
    MODULE_DESCRIPTION("HID driver for some cherry \"special\" devices");
    MODULE_LICENSE("GPL");
