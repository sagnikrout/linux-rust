//! Automatically rewritten from C to Rust
//! Source: drivers/hid/hid-gyration.c
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
// HID driver for some gyration "special" devices
//
// Copyright (c) 1999 Andreas Gal
// Copyright (c) 2000-2005 Vojtech Pavlik <vojtech@suse.cz>
// Copyright (c) 2005 Michael Haboustak <mike-@cinci.rr.com> for Concept2, Inc
// Copyright (c) 2008 Jiri Slaby
// Copyright (c) 2006-2008 Jiri Kosina
//

    EV_KEY, (c))
    static int gyration_input_mapping(struct hid_device *hdev, struct hid_input *hi,
    struct hid_field *field, struct hid_usage *usage,
    unsigned long **bit, int *max)
    {
    if ((usage.hid & HID_USAGE_PAGE) != HID_UP_LOGIVENDOR)
    return 0;
    set_bit(EV_REP, hi.input.evbit);
    switch (usage.hid & HID_USAGE) {
// Reported on Gyration MCE Remote
    case 0x00d: gy_map_key_clear(KEY_HOME);		break;
    case 0x024: gy_map_key_clear(KEY_DVD);		break;
    case 0x025: gy_map_key_clear(KEY_PVR);		break;
    case 0x046: gy_map_key_clear(KEY_MEDIA);	break;
    case 0x047: gy_map_key_clear(KEY_MP3);		break;
    case 0x048: gy_map_key_clear(KEY_MEDIA);	break;
    case 0x049: gy_map_key_clear(KEY_CAMERA);	break;
    case 0x04a: gy_map_key_clear(KEY_VIDEO);	break;
    case 0x05a: gy_map_key_clear(KEY_TEXT);		break;
    case 0x05b: gy_map_key_clear(KEY_RED);		break;
    case 0x05c: gy_map_key_clear(KEY_GREEN);	break;
    case 0x05d: gy_map_key_clear(KEY_YELLOW);	break;
    case 0x05e: gy_map_key_clear(KEY_BLUE);		break;
    default:
    return 0;
    }
    return 1;
    }
    static int gyration_event(struct hid_device *hdev, struct hid_field *field,
    struct hid_usage *usage, __s32 value)
    {
    if (!(hdev.claimed & HID_CLAIMED_INPUT) || !field.hidinput)
    return 0;
    if ((usage.hid & HID_USAGE_PAGE) == HID_UP_GENDESK &&
    (usage.hid & 0xff) == 0x82) {
    struct input_dev *input = field.hidinput.input;
    input_event(input, usage.type, usage.code, 1);
    input_sync(input);
    input_event(input, usage.type, usage.code, 0);
    input_sync(input);
    return 1;
    }
    return 0;
    }
    static const struct hid_device_id gyration_devices[] = {
    { HID_USB_DEVICE(USB_VENDOR_ID_GYRATION, USB_DEVICE_ID_GYRATION_REMOTE) },
    { HID_USB_DEVICE(USB_VENDOR_ID_GYRATION, USB_DEVICE_ID_GYRATION_REMOTE_2) },
    { HID_USB_DEVICE(USB_VENDOR_ID_GYRATION, USB_DEVICE_ID_GYRATION_REMOTE_3) },
    { }
    };
    MODULE_DEVICE_TABLE(hid, gyration_devices);
    static struct hid_driver gyration_driver = {
    .name = "gyration",
    .id_table = gyration_devices,
    .input_mapping = gyration_input_mapping,
    .event = gyration_event,
    };
    module_hid_driver(gyration_driver);
    MODULE_DESCRIPTION("HID driver for some gyration \"special\" devices");
    MODULE_LICENSE("GPL");
