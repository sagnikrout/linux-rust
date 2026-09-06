//! Automatically rewritten from C to Rust
//! Source: drivers/hid/hid-nti.c
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
// USB HID quirks support for Network Technologies, Inc. "USB-SUN" USB
// adapter for pre-USB Sun keyboards
//
// Copyright (c) 2011 Google, Inc.
//
// Based on HID apple driver by
// Copyright (c) 1999 Andreas Gal
// Copyright (c) 2000-2005 Vojtech Pavlik <vojtech@suse.cz>
// Copyright (c) 2005 Michael Haboustak <mike-@cinci.rr.com> for Concept2, Inc
// Copyright (c) 2006-2007 Jiri Kosina
// Copyright (c) 2008 Jiri Slaby <jirislaby@gmail.com>
//

    MODULE_AUTHOR("Jonathan Klabunde Tomer <jktomer@google.com>");
    MODULE_DESCRIPTION("HID driver for Network Technologies USB-SUN keyboard adapter");
//
// NTI Sun keyboard adapter has wrong logical maximum in report descriptor
//
    static const __u8 *nti_usbsun_report_fixup(struct hid_device *hdev, __u8 *rdesc,
    unsigned int *rsize)
    {
    if (*rsize >= 60 && rdesc[53] == 0x65 && rdesc[59] == 0x65) {
    hid_info(hdev, "fixing up NTI USB-SUN keyboard adapter report descriptor\n");
    rdesc[53] = rdesc[59] = 0xe7;
    }
    return rdesc;
    }
    static const struct hid_device_id nti_devices[] = {
    { HID_USB_DEVICE(USB_VENDOR_ID_NTI, USB_DEVICE_ID_USB_SUN) },
    { }
    };
    MODULE_DEVICE_TABLE(hid, nti_devices);
    static struct hid_driver nti_driver = {
    .name = "nti",
    .id_table = nti_devices,
    .report_fixup = nti_usbsun_report_fixup
    };
    module_hid_driver(nti_driver);
    MODULE_LICENSE("GPL");
