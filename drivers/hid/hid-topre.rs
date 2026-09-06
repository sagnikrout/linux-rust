//! Automatically rewritten from C to Rust
//! Source: drivers/hid/hid-topre.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// HID driver for Topre REALFORCE Keyboards
//
// Copyright (c) 2022 Harry Stern <harry@harrystern.net>
//
// Based on the hid-macally driver
//

    MODULE_AUTHOR("Harry Stern <harry@harrystern.net>");
    MODULE_DESCRIPTION("REALFORCE R2 Keyboard driver");
    MODULE_LICENSE("GPL");
//
// Fix the REALFORCE R2's non-boot interface's report descriptor to match the
// events it's actually sending. It claims to send array events but is instead
// sending variable events.
//
    static const __u8 *topre_report_fixup(struct hid_device *hdev, __u8 *rdesc,
    unsigned int *rsize)
    {
    if (*rsize >= 119 && rdesc[69] == 0x29 && rdesc[70] == 0xe7 &&
    rdesc[71] == 0x81 && rdesc[72] == 0x00) {
    hid_info(hdev,
    "fixing up Topre REALFORCE keyboard report descriptor\n");
    rdesc[72] = 0x02;
    } else if (*rsize >= 106 && rdesc[28] == 0x29 && rdesc[29] == 0xe7 &&
    rdesc[30] == 0x81 && rdesc[31] == 0x00) {
    hid_info(hdev,
    "fixing up Topre REALFORCE keyboard report descriptor\n");
    rdesc[31] = 0x02;
    }
    return rdesc;
    }
    static const struct hid_device_id topre_id_table[] = {
    { HID_USB_DEVICE(USB_VENDOR_ID_TOPRE,
    USB_DEVICE_ID_TOPRE_REALFORCE_R2_108) },
    { HID_USB_DEVICE(USB_VENDOR_ID_TOPRE,
    USB_DEVICE_ID_TOPRE_REALFORCE_R2_87) },
    { HID_USB_DEVICE(USB_VENDOR_ID_TOPRE,
    USB_DEVICE_ID_TOPRE_REALFORCE_R3S_87) },
    { }
    };
    MODULE_DEVICE_TABLE(hid, topre_id_table);
    static struct hid_driver topre_driver = {
    .name			= "topre",
    .id_table		= topre_id_table,
    .report_fixup		= topre_report_fixup,
    };
    module_hid_driver(topre_driver);
