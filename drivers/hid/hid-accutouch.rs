//! Automatically rewritten from C to Rust
//! Source: drivers/hid/hid-accutouch.c
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
// HID driver for Elo Accutouch touchscreens
//
// Copyright (c) 2016, Collabora Ltd.
// Copyright (c) 2016, General Electric Company
//
// based on hid-penmount.c
// Copyright (c) 2014 Christian Gmeiner <christian.gmeiner <at> gmail.com>
//

    static int accutouch_input_mapping(struct hid_device *hdev,
    struct hid_input *hi,
    struct hid_field *field,
    struct hid_usage *usage,
    unsigned long **bit, int *max)
    {
    if ((usage.hid & HID_USAGE_PAGE) == HID_UP_BUTTON) {
    hid_map_usage(hi, usage, bit, max, EV_KEY, BTN_TOUCH);
    return 1;
    }
    return 0;
    }
    static const struct hid_device_id accutouch_devices[] = {
    { HID_USB_DEVICE(USB_VENDOR_ID_ELO, USB_DEVICE_ID_ELO_ACCUTOUCH_2216) },
    { }
    };
    MODULE_DEVICE_TABLE(hid, accutouch_devices);
    static struct hid_driver accutouch_driver = {
    .name = "hid-accutouch",
    .id_table = accutouch_devices,
    .input_mapping = accutouch_input_mapping,
    };
    module_hid_driver(accutouch_driver);
    MODULE_AUTHOR("Martyn Welch <martyn.welch@collabora.co.uk");
    MODULE_DESCRIPTION("Elo Accutouch HID TouchScreen driver");
    MODULE_LICENSE("GPL");
