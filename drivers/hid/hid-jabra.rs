//! Automatically rewritten from C to Rust
//! Source: drivers/hid/hid-jabra.c
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
// Jabra USB HID Driver
//
// Copyright (c) 2017 Niels Skou Olsen <nolsen@jabra.com>
//

pub const HID_UP_VENDOR_DEFINED_MIN: c_uint = 0xff000000;
pub const HID_UP_VENDOR_DEFINED_MAX: c_uint = 0xffff0000;
    static int jabra_input_mapping(struct hid_device *hdev,
    struct hid_input *hi,
    struct hid_field *field,
    struct hid_usage *usage,
    unsigned long **bit, int *max)
    {
    int is_vendor_defined =
    ((usage.hid & HID_USAGE_PAGE) >= HID_UP_VENDOR_DEFINED_MIN &&
    (usage.hid & HID_USAGE_PAGE) <= HID_UP_VENDOR_DEFINED_MAX);
    dbg_hid("hid=0x%08x appl=0x%08x coll_idx=0x%02x usage_idx=0x%02x: %s\n",
    usage.hid,
    field.application,
    usage.collection_index,
    usage.usage_index,
    is_vendor_defined ? "ignored" : "defaulted");
// Ignore vendor defined usages, default map standard usages
    return is_vendor_defined ? -1 : 0;
    }
    static const struct hid_device_id jabra_devices[] = {
    { HID_USB_DEVICE(USB_VENDOR_ID_JABRA, HID_ANY_ID) },
    { }
    };
    MODULE_DEVICE_TABLE(hid, jabra_devices);
    static struct hid_driver jabra_driver = {
    .name = "jabra",
    .id_table = jabra_devices,
    .input_mapping = jabra_input_mapping,
    };
    module_hid_driver(jabra_driver);
    MODULE_AUTHOR("Niels Skou Olsen <nolsen@jabra.com>");
    MODULE_DESCRIPTION("Jabra USB HID Driver");
    MODULE_LICENSE("GPL");
