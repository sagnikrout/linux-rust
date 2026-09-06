//! Automatically rewritten from C to Rust
//! Source: drivers/hid/hid-kensington.c
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
// HID driver for Kensington Slimblade Trackball
//
// Copyright (c) 2009 Jiri Kosina
//

    static int ks_input_mapping(struct hid_device *hdev, struct hid_input *hi,
    struct hid_field *field, struct hid_usage *usage,
    unsigned long **bit, int *max)
    {
    if ((usage.hid & HID_USAGE_PAGE) != HID_UP_MSVENDOR)
    return 0;
    switch (usage.hid & HID_USAGE) {
    case 0x01: ks_map_key(BTN_MIDDLE);	break;
    case 0x02: ks_map_key(BTN_SIDE);	break;
    default:
    return 0;
    }
    return 1;
    }
    static const struct hid_device_id ks_devices[] = {
    { HID_USB_DEVICE(USB_VENDOR_ID_KENSINGTON, USB_DEVICE_ID_KS_SLIMBLADE) },
    { }
    };
    MODULE_DEVICE_TABLE(hid, ks_devices);
    static struct hid_driver ks_driver = {
    .name = "kensington",
    .id_table = ks_devices,
    .input_mapping = ks_input_mapping,
    };
    module_hid_driver(ks_driver);
    MODULE_DESCRIPTION("HID driver for Kensington Slimblade Trackball");
    MODULE_LICENSE("GPL");
