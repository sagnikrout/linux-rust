//! Automatically rewritten from C to Rust
//! Source: drivers/hid/hid-penmount.c
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
// HID driver for PenMount touchscreens
//
// Copyright (c) 2014 Christian Gmeiner <christian.gmeiner <at> gmail.com>
//
// based on hid-penmount copyrighted by
// PenMount Touch Solutions <penmount <at> seed.net.tw>
//

    static int penmount_input_mapping(struct hid_device *hdev,
    struct hid_input *hi, struct hid_field *field,
    struct hid_usage *usage, unsigned long **bit, int *max)
    {
    if ((usage.hid & HID_USAGE_PAGE) == HID_UP_BUTTON) {
    if (((usage.hid - 1) & HID_USAGE) == 0) {
    hid_map_usage(hi, usage, bit, max, EV_KEY, BTN_TOUCH);
    return 1;
    } else {
    return -1;
    }
    }
    return 0;
    }
    static const struct hid_device_id penmount_devices[] = {
    { HID_USB_DEVICE(USB_VENDOR_ID_PENMOUNT, USB_DEVICE_ID_PENMOUNT_6000) },
    { }
    };
    MODULE_DEVICE_TABLE(hid, penmount_devices);
    static struct hid_driver penmount_driver = {
    .name = "hid-penmount",
    .id_table = penmount_devices,
    .input_mapping = penmount_input_mapping,
    };
    module_hid_driver(penmount_driver);
    MODULE_AUTHOR("Christian Gmeiner <christian.gmeiner@gmail.com>");
    MODULE_DESCRIPTION("PenMount HID TouchScreen driver");
    MODULE_LICENSE("GPL");
