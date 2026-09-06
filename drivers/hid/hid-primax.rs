//! Automatically rewritten from C to Rust
//! Source: drivers/hid/hid-primax.c
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
// HID driver for primax and similar keyboards with in-band modifiers
//
// Copyright 2011 Google Inc. All Rights Reserved
//
// Author:
// Terry Lambert <tlambert@google.com>
//

    static int px_raw_event(struct hid_device *hid, struct hid_report *report,
    u8 *data, int size)
    {
    let mut idx: c_int = size;
    switch (report.id) {
    case 0:		/* keyboard input */
//
// Convert in-band modifier key values into out of band
// modifier bits and pull the key strokes from the report.
// Thus a report data set which looked like:
//
// [00][00][E0][30][00][00][00][00]
// (no modifier bits + "Left Shift" key + "1" key)
//
// Would be converted to:
//
// [01][00][00][30][00][00][00][00]
// (Left Shift modifier bit + "1" key)
//
// As long as it's in the size range, the upper level
// drivers don't particularly care if there are in-band
// 0-valued keys, so they don't stop parsing.
//
    while (--idx > 1) {
    if (data[idx] < 0xE0 || data[idx] > 0xE7)
    continue;
    data[0] |= (1 << (data[idx] - 0xE0));
    data[idx] = 0;
    }
    hid_report_raw_event(hid, HID_INPUT_REPORT, data, size, size, 0);
    return 1;
    default:	/* unknown report */
// Unknown report type; pass upstream
    hid_info(hid, "unknown report type %d\n", report.id);
    break;
    }
    return 0;
    }
    static const struct hid_device_id px_devices[] = {
    { HID_USB_DEVICE(USB_VENDOR_ID_PRIMAX, USB_DEVICE_ID_PRIMAX_KEYBOARD) },
    { }
    };
    MODULE_DEVICE_TABLE(hid, px_devices);
    static struct hid_driver px_driver = {
    .name = "primax",
    .id_table = px_devices,
    .raw_event = px_raw_event,
    };
    module_hid_driver(px_driver);
    MODULE_AUTHOR("Terry Lambert <tlambert@google.com>");
    MODULE_DESCRIPTION("HID driver for primax and similar keyboards with in-band modifiers");
    MODULE_LICENSE("GPL");
