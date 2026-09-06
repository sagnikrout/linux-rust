//! Automatically rewritten from C to Rust
//! Source: drivers/hid/hid-redragon.c
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


//
// HID driver for Redragon keyboards
//
// Copyright (c) 2017 Robert Munteanu
// SPDX-License-Identifier: GPL-2.0+
//
// This program is free software; you can redistribute it and/or modify it
// under the terms of the GNU General Public License as published by the Free
// Software Foundation; either version 2 of the License, or (at your option)
// any later version.
//

//
// The Redragon Asura keyboard sends an incorrect HID descriptor.
// At byte 100 it contains
//
// 0x81, 0x00
//
// which is Input (Data, Arr, Abs), but it should be
//
// 0x81, 0x02
//
// which is Input (Data, Var, Abs), which is consistent with the way
// key codes are generated.
//
    static const __u8 *redragon_report_fixup(struct hid_device *hdev, __u8 *rdesc,
    unsigned int *rsize)
    {
    if (*rsize >= 102 && rdesc[100] == 0x81 && rdesc[101] == 0x00) {
    dev_info(&hdev.dev, "Fixing Redragon ASURA report descriptor.\n");
    rdesc[101] = 0x02;
    }
    return rdesc;
    }
    static const struct hid_device_id redragon_devices[] = {
    {HID_USB_DEVICE(USB_VENDOR_ID_JESS, USB_DEVICE_ID_REDRAGON_ASURA)},
    {}
    };
    MODULE_DEVICE_TABLE(hid, redragon_devices);
    static struct hid_driver redragon_driver = {
    .name = "redragon",
    .id_table = redragon_devices,
    .report_fixup = redragon_report_fixup
    };
    module_hid_driver(redragon_driver);
    MODULE_DESCRIPTION("HID driver for Redragon keyboards");
    MODULE_LICENSE("GPL");
