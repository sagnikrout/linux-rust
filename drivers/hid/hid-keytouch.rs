//! Automatically rewritten from C to Rust
//! Source: drivers/hid/hid-keytouch.c
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
// HID driver for Keytouch devices not fully compliant with HID standard
//
// Copyright (c) 2011 Jiri Kosina
//

// Replace the broken report descriptor of this device with rather
// a default one
    static const __u8 keytouch_fixed_rdesc[] = {
    0x05, 0x01, 0x09, 0x06, 0xa1, 0x01, 0x05, 0x07, 0x19, 0xe0, 0x29, 0xe7, 0x15,
    0x00, 0x25, 0x01, 0x75, 0x01, 0x95, 0x08, 0x81, 0x02, 0x95, 0x01, 0x75, 0x08,
    0x81, 0x01, 0x95, 0x03, 0x75, 0x01, 0x05, 0x08, 0x19, 0x01, 0x29, 0x03, 0x91,
    0x02, 0x95, 0x05, 0x75, 0x01, 0x91, 0x01, 0x95, 0x06, 0x75, 0x08, 0x15, 0x00,
    0x26, 0xff, 0x00, 0x05, 0x07, 0x19, 0x00, 0x2a, 0xff, 0x00, 0x81, 0x00, 0xc0
    };
    static const __u8 *keytouch_report_fixup(struct hid_device *hdev, __u8 *rdesc,
    unsigned int *rsize)
    {
    hid_info(hdev, "fixing up Keytouch IEC report descriptor\n");
// rsize = sizeof(keytouch_fixed_rdesc);
    return keytouch_fixed_rdesc;
    }
    static const struct hid_device_id keytouch_devices[] = {
    { HID_USB_DEVICE(USB_VENDOR_ID_KEYTOUCH, USB_DEVICE_ID_KEYTOUCH_IEC) },
    { }
    };
    MODULE_DEVICE_TABLE(hid, keytouch_devices);
    static struct hid_driver keytouch_driver = {
    .name = "keytouch",
    .id_table = keytouch_devices,
    .report_fixup = keytouch_report_fixup,
    };
    module_hid_driver(keytouch_driver);
    MODULE_DESCRIPTION("HID driver for Keytouch devices not fully compliant with HID standard");
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Jiri Kosina");
