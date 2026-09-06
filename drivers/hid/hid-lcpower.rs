//! Automatically rewritten from C to Rust
//! Source: drivers/hid/hid-lcpower.c
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
// HID driver for LC Power Model RC1000MCE
//
// Copyright (c) 2011 Chris Schlund
// based on hid-topseed module
//

    EV_KEY, (c))
    static int ts_input_mapping(struct hid_device *hdev, struct hid_input *hi,
    struct hid_field *field, struct hid_usage *usage,
    unsigned long **bit, int *max)
    {
    if ((usage.hid & HID_USAGE_PAGE) != HID_UP_LOGIVENDOR)
    return 0;
    switch (usage.hid & HID_USAGE) {
    case 0x046: ts_map_key_clear(KEY_YELLOW);         break;
    case 0x047: ts_map_key_clear(KEY_GREEN);          break;
    case 0x049: ts_map_key_clear(KEY_BLUE);           break;
    case 0x04a: ts_map_key_clear(KEY_RED);		  break;
    case 0x00d: ts_map_key_clear(KEY_HOME);           break;
    case 0x025: ts_map_key_clear(KEY_TV);             break;
    case 0x048: ts_map_key_clear(KEY_VCR);            break;
    case 0x024: ts_map_key_clear(KEY_MENU);           break;
    default:
    return 0;
    }
    return 1;
    }
    static const struct hid_device_id ts_devices[] = {
    { HID_USB_DEVICE( USB_VENDOR_ID_LCPOWER, USB_DEVICE_ID_LCPOWER_LC1000) },
    { }
    };
    MODULE_DEVICE_TABLE(hid, ts_devices);
    static struct hid_driver ts_driver = {
    .name = "LC RC1000MCE",
    .id_table = ts_devices,
    .input_mapping = ts_input_mapping,
    };
    module_hid_driver(ts_driver);
    MODULE_DESCRIPTION("HID driver for LC Power Model RC1000MCE");
    MODULE_LICENSE("GPL");
