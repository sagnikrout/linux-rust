//! Automatically rewritten from C to Rust
//! Source: drivers/hid/hid-macally.c
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
// HID driver for quirky Macally devices
//
// Copyright (c) 2019 Alex Henrie <alexhenrie24@gmail.com>
//

    MODULE_AUTHOR("Alex Henrie <alexhenrie24@gmail.com>");
    MODULE_DESCRIPTION("Macally devices");
    MODULE_LICENSE("GPL");
//
// The Macally ikey keyboard says that its logical and usage maximums are both
// 101, but the power key is 102 and the equals key is 103
//
    static const __u8 *macally_report_fixup(struct hid_device *hdev, __u8 *rdesc,
    unsigned int *rsize)
    {
    if (*rsize >= 60 && rdesc[53] == 0x65 && rdesc[59] == 0x65) {
    hid_info(hdev,
    "fixing up Macally ikey keyboard report descriptor\n");
    rdesc[53] = rdesc[59] = 0x67;
    }
    return rdesc;
    }
    static const struct hid_device_id macally_id_table[] = {
    { HID_USB_DEVICE(USB_VENDOR_ID_SOLID_YEAR,
    USB_DEVICE_ID_MACALLY_IKEY_KEYBOARD) },
    { }
    };
    MODULE_DEVICE_TABLE(hid, macally_id_table);
    static struct hid_driver macally_driver = {
    .name			= "macally",
    .id_table		= macally_id_table,
    .report_fixup		= macally_report_fixup,
    };
    module_hid_driver(macally_driver);
