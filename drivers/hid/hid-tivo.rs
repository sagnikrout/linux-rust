//! Automatically rewritten from C to Rust
//! Source: drivers/hid/hid-tivo.c
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
// HID driver for TiVo Slide Bluetooth remote
//
// Copyright (c) 2011 Jarod Wilson <jarod@redhat.com>
// based on the hid-topseed driver, which is in turn, based on hid-cherry...
//

pub const HID_UP_TIVOVENDOR: c_uint = 0xffff0000;

    EV_KEY, (c))
    static int tivo_input_mapping(struct hid_device *hdev, struct hid_input *hi,
    struct hid_field *field, struct hid_usage *usage,
    unsigned long **bit, int *max)
    {
    switch (usage.hid & HID_USAGE_PAGE) {
    case HID_UP_TIVOVENDOR:
    switch (usage.hid & HID_USAGE) {
// TiVo button
    case 0x3d: tivo_map_key_clear(KEY_MEDIA);	break;
// Live TV
    case 0x3e: tivo_map_key_clear(KEY_TV);		break;
// Red thumbs down
    case 0x41: tivo_map_key_clear(KEY_KPMINUS);	break;
// Green thumbs up
    case 0x42: tivo_map_key_clear(KEY_KPPLUS);	break;
    default:
    return 0;
    }
    break;
    case HID_UP_CONSUMER:
    switch (usage.hid & HID_USAGE) {
// Enter/Last (default mapping: KEY_LAST)
    case 0x083: tivo_map_key_clear(KEY_ENTER);	break;
// Info (default mapping: KEY_PROPS)
    case 0x209: tivo_map_key_clear(KEY_INFO);	break;
    default:
    return 0;
    }
    break;
    default:
    return 0;
    }
// This means we found a matching mapping here, else, look in the
// standard hid mappings in hid-input.c
    return 1;
    }
    static const struct hid_device_id tivo_devices[] = {
// TiVo Slide Bluetooth remote, pairs with a Broadcom dongle
    { HID_BLUETOOTH_DEVICE(USB_VENDOR_ID_TIVO, USB_DEVICE_ID_TIVO_SLIDE_BT) },
    { HID_USB_DEVICE(USB_VENDOR_ID_TIVO, USB_DEVICE_ID_TIVO_SLIDE) },
    { HID_USB_DEVICE(USB_VENDOR_ID_TIVO, USB_DEVICE_ID_TIVO_SLIDE_PRO) },
    { }
    };
    MODULE_DEVICE_TABLE(hid, tivo_devices);
    static struct hid_driver tivo_driver = {
    .name = "tivo_slide",
    .id_table = tivo_devices,
    .input_mapping = tivo_input_mapping,
    };
    module_hid_driver(tivo_driver);
    MODULE_DESCRIPTION("HID driver for TiVo Slide Bluetooth remote");
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Jarod Wilson <jarod@redhat.com>");
