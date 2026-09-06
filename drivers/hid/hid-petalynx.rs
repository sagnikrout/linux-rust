//! Automatically rewritten from C to Rust
//! Source: drivers/hid/hid-petalynx.c
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
// HID driver for some petalynx "special" devices
//
// Copyright (c) 1999 Andreas Gal
// Copyright (c) 2000-2005 Vojtech Pavlik <vojtech@suse.cz>
// Copyright (c) 2005 Michael Haboustak <mike-@cinci.rr.com> for Concept2, Inc
// Copyright (c) 2006-2007 Jiri Kosina
// Copyright (c) 2008 Jiri Slaby
//

// Petalynx Maxter Remote has maximum for consumer page set too low
    static const __u8 *pl_report_fixup(struct hid_device *hdev, __u8 *rdesc,
    unsigned int *rsize)
    {
    if (*rsize >= 62 && rdesc[39] == 0x2a && rdesc[40] == 0xf5 &&
    rdesc[41] == 0x00 && rdesc[59] == 0x26 &&
    rdesc[60] == 0xf9 && rdesc[61] == 0x00) {
    hid_info(hdev, "fixing up Petalynx Maxter Remote report descriptor\n");
    rdesc[60] = 0xfa;
    rdesc[40] = 0xfa;
    }
    return rdesc;
    }

    EV_KEY, (c))
    static int pl_input_mapping(struct hid_device *hdev, struct hid_input *hi,
    struct hid_field *field, struct hid_usage *usage,
    unsigned long **bit, int *max)
    {
    if ((usage.hid & HID_USAGE_PAGE) == HID_UP_LOGIVENDOR) {
    switch (usage.hid & HID_USAGE) {
    case 0x05a: pl_map_key_clear(KEY_TEXT);		break;
    case 0x05b: pl_map_key_clear(KEY_RED);		break;
    case 0x05c: pl_map_key_clear(KEY_GREEN);	break;
    case 0x05d: pl_map_key_clear(KEY_YELLOW);	break;
    case 0x05e: pl_map_key_clear(KEY_BLUE);		break;
    default:
    return 0;
    }
    return 1;
    }
    if ((usage.hid & HID_USAGE_PAGE) == HID_UP_CONSUMER) {
    switch (usage.hid & HID_USAGE) {
    case 0x0f6: pl_map_key_clear(KEY_NEXT);		break;
    case 0x0fa: pl_map_key_clear(KEY_BACK);		break;
    default:
    return 0;
    }
    return 1;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pl_probe(hdev: *mut hid_device, id: *const hid_device_id) -> c_int {
    static int pl_probe(struct hid_device *hdev, const struct hid_device_id *id)
    {
    int ret;
    hdev.quirks |= HID_QUIRK_NOGET;
    ret = hid_parse(hdev);
    if (ret) {
    hid_err(hdev, "parse failed\n");
    goto err_free;
    }
    ret = hid_hw_start(hdev, HID_CONNECT_DEFAULT);
    if (ret) {
    hid_err(hdev, "hw start failed\n");
    goto err_free;
    }
    return 0;
    err_free:
    return ret;
    }
    static const struct hid_device_id pl_devices[] = {
    { HID_USB_DEVICE(USB_VENDOR_ID_PETALYNX, USB_DEVICE_ID_PETALYNX_MAXTER_REMOTE) },
    { }
    };
    MODULE_DEVICE_TABLE(hid, pl_devices);
    static struct hid_driver pl_driver = {
    .name = "petalynx",
    .id_table = pl_devices,
    .report_fixup = pl_report_fixup,
    .input_mapping = pl_input_mapping,
    .probe = pl_probe,
    };
    module_hid_driver(pl_driver);
    MODULE_DESCRIPTION("HID driver for some petalynx \"special\" devices");
    MODULE_LICENSE("GPL");
