//! Automatically rewritten from C to Rust
//! Source: drivers/hid/hid-speedlink.c
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
// HID driver for Speedlink Vicious and Divine Cezanne (USB mouse).
// Fixes "jumpy" cursor and removes nonexistent keyboard LEDS from
// the HID descriptor.
//
// Copyright (c) 2011, 2013 Stefan Kriwanek <dev@stefankriwanek.de>
//

    static const struct hid_device_id speedlink_devices[] = {
    { HID_USB_DEVICE(USB_VENDOR_ID_X_TENSIONS, USB_DEVICE_ID_SPEEDLINK_VAD_CEZANNE)},
    { }
    };
    static int speedlink_input_mapping(struct hid_device *hdev,
    struct hid_input *hi,
    struct hid_field *field, struct hid_usage *usage,
    unsigned long **bit, int *max)
    {
//
// The Cezanne mouse has a second "keyboard" USB endpoint for it is
// able to map keyboard events to the button presses.
// It sends a standard keyboard report descriptor, though, whose
// LEDs we ignore.
//
    switch (usage.hid & HID_USAGE_PAGE) {
    case HID_UP_LED:
    return -1;
    }
    return 0;
    }
    static int speedlink_event(struct hid_device *hdev, struct hid_field *field,
    struct hid_usage *usage, __s32 value)
    {
// No other conditions due to usage_table.
// This fixes the "jumpy" cursor occuring due to invalid events sent
// by the device. Some devices only send them with value==+256, others
// don't. However, catching abs(value)>=256 is restrictive enough not
// to interfere with devices that were bug-free (has been tested).
//
    if (abs(value) >= 256)
    return 1;
// Drop useless distance 0 events (on button clicks etc.) as well
    if (value == 0)
    return 1;
    return 0;
    }
    MODULE_DEVICE_TABLE(hid, speedlink_devices);
    static const struct hid_usage_id speedlink_grabbed_usages[] = {
    { HID_GD_X, EV_REL, 0 },
    { HID_GD_Y, EV_REL, 1 },
    { HID_ANY_ID - 1, HID_ANY_ID - 1, HID_ANY_ID - 1}
    };
    static struct hid_driver speedlink_driver = {
    .name = "speedlink",
    .id_table = speedlink_devices,
    .usage_table = speedlink_grabbed_usages,
    .input_mapping = speedlink_input_mapping,
    .event = speedlink_event,
    };
    module_hid_driver(speedlink_driver);
    MODULE_DESCRIPTION("HID driver for Speedlink Vicious and Divine Cezanne (USB mouse)");
    MODULE_LICENSE("GPL");
