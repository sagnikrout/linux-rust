//! Automatically rewritten from C to Rust
//! Source: drivers/hid/hid-xinmo.c
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
// HID driver for Xin-Mo devices, currently only the Dual Arcade controller.
// Fixes the negative axis event values (the devices sends -2) to match the
// logical axis minimum of the HID report descriptor (the report announces
// -1). It is needed because hid-input discards out of bounds values.
// (This module is based on "hid-saitek" and "hid-lg".)
//
// Copyright (c) 2013 Olivier Scherler
//

//
// Fix negative events that are out of bounds.
//
    static int xinmo_event(struct hid_device *hdev, struct hid_field *field,
    struct hid_usage *usage, __s32 value)
    {
    switch (usage.code) {
    case ABS_X:
    case ABS_Y:
    case ABS_Z:
    case ABS_RX:
    if (value < -1) {
    input_event(field.hidinput.input, usage.type,
    usage.code, -1);
    return 1;
    }
    break;
    }
    return 0;
    }
    static const struct hid_device_id xinmo_devices[] = {
    { HID_USB_DEVICE(USB_VENDOR_ID_XIN_MO, USB_DEVICE_ID_XIN_MO_DUAL_ARCADE) },
    { HID_USB_DEVICE(USB_VENDOR_ID_XIN_MO, USB_DEVICE_ID_THT_2P_ARCADE) },
    { }
    };
    MODULE_DEVICE_TABLE(hid, xinmo_devices);
    static struct hid_driver xinmo_driver = {
    .name = "xinmo",
    .id_table = xinmo_devices,
    .event = xinmo_event
    };
    module_hid_driver(xinmo_driver);
    MODULE_DESCRIPTION("HID driver for Xin-Mo devices");
    MODULE_LICENSE("GPL");
