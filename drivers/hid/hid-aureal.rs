//! Automatically rewritten from C to Rust
//! Source: drivers/hid/hid-aureal.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// HID driver for Aureal Cy se W-01RN USB_V3.1 devices
//
// Copyright (c) 2010 Franco Catrin <fcatrin@gmail.com>
// Copyright (c) 2010 Ben Cropley <bcropley@internode.on.net>
//
// Based on HID sunplus driver by
// Copyright (c) 1999 Andreas Gal
// Copyright (c) 2000-2005 Vojtech Pavlik <vojtech@suse.cz>
// Copyright (c) 2005 Michael Haboustak <mike-@cinci.rr.com> for Concept2, Inc
// Copyright (c) 2006-2007 Jiri Kosina
// Copyright (c) 2008 Jiri Slaby
//

    static const __u8 *aureal_report_fixup(struct hid_device *hdev, __u8 *rdesc,
    unsigned int *rsize)
    {
    if (*rsize >= 54 && rdesc[52] == 0x25 && rdesc[53] == 0x01) {
    dev_info(&hdev.dev, "fixing Aureal Cy se W-01RN USB_V3.1 report descriptor.\n");
    rdesc[53] = 0x65;
    }
    return rdesc;
    }
    static const struct hid_device_id aureal_devices[] = {
    { HID_USB_DEVICE(USB_VENDOR_ID_AUREAL, USB_DEVICE_ID_AUREAL_W01RN) },
    { }
    };
    MODULE_DEVICE_TABLE(hid, aureal_devices);
    static struct hid_driver aureal_driver = {
    .name = "aureal",
    .id_table = aureal_devices,
    .report_fixup = aureal_report_fixup,
    };
    module_hid_driver(aureal_driver);
    MODULE_DESCRIPTION("HID driver for Aureal Cy se W-01RN USB_V3.1 devices");
    MODULE_LICENSE("GPL");
