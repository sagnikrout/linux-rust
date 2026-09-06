//! Automatically rewritten from C to Rust
//! Source: drivers/hid/hid-generic.c
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
// HID support for Linux
//
// Copyright (c) 1999 Andreas Gal
// Copyright (c) 2000-2005 Vojtech Pavlik <vojtech@suse.cz>
// Copyright (c) 2005 Michael Haboustak <mike-@cinci.rr.com> for Concept2, Inc
// Copyright (c) 2007-2008 Oliver Neukum
// Copyright (c) 2006-2012 Jiri Kosina
// Copyright (c) 2012 Henrik Rydberg
//

    static struct hid_driver hid_generic;
#[no_mangle]
unsafe extern "C" fn __check_hid_generic(drv: *mut device_driver, data: *mut c_void) -> c_int {
    static int __check_hid_generic(struct device_driver *drv, void *data)
    {
    struct hid_driver *hdrv = to_hid_driver(drv);
    struct hid_device *hdev = data;
    if (hdrv == &hid_generic)
    return 0;
    return hid_match_device(hdev, hdrv) != core::ptr::null_mut();
    }
    static bool hid_generic_match(struct hid_device *hdev,
    bool ignore_special_driver)
    {
    if (ignore_special_driver)
    return true;
    if (hdev.quirks & HID_QUIRK_IGNORE_SPECIAL_DRIVER)
    return true;
    if (hdev.quirks & HID_QUIRK_HAVE_SPECIAL_DRIVER)
    return false;
//
// If any other driver wants the device, leave the device to this other
// driver.
//
    if (bus_for_each_drv(&hid_bus_type, core::ptr::null_mut(), hdev, __check_hid_generic))
    return false;
    return true;
    }
    static int hid_generic_probe(struct hid_device *hdev,
    const struct hid_device_id *id)
    {
    int ret;
    hdev.quirks |= HID_QUIRK_INPUT_PER_APP;
    ret = hid_parse(hdev);
    if (ret)
    return ret;
    return hid_hw_start(hdev, HID_CONNECT_DEFAULT);
    }
#[no_mangle]
unsafe extern "C" fn hid_generic_reset_resume(hdev: *mut hid_device) -> c_int {
    static int hid_generic_reset_resume(struct hid_device *hdev)
    {
    if (hdev.claimed & HID_CLAIMED_INPUT)
    hidinput_reset_resume(hdev);
    return 0;
    }
    static const struct hid_device_id hid_table[] = {
    { HID_DEVICE(HID_BUS_ANY, HID_GROUP_ANY, HID_ANY_ID, HID_ANY_ID) },
    { }
    };
    MODULE_DEVICE_TABLE(hid, hid_table);
    static struct hid_driver hid_generic = {
    .name = "hid-generic",
    .id_table = hid_table,
    .match = hid_generic_match,
    .probe = hid_generic_probe,
    .reset_resume = hid_generic_reset_resume,
    };
    module_hid_driver(hid_generic);
    MODULE_AUTHOR("Henrik Rydberg");
    MODULE_DESCRIPTION("HID generic driver");
    MODULE_LICENSE("GPL");
