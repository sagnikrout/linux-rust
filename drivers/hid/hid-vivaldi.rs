//! Automatically rewritten from C to Rust
//! Source: drivers/hid/hid-vivaldi.c
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


// SPDX-License-Identifier: GPL-2.0
//
// HID support for Vivaldi Keyboard
//
// Copyright 2020 Google LLC.
// Author: Sean O'Brien <seobrien@chromium.org>
//

    static int vivaldi_probe(struct hid_device *hdev,
    const struct hid_device_id *id)
    {
    struct vivaldi_data *drvdata;
    int ret;
    drvdata = devm_kzalloc(&hdev.dev, sizeof(*drvdata), GFP_KERNEL);
    if (!drvdata)
    return -ENOMEM;
    hid_set_drvdata(hdev, drvdata);
    ret = hid_parse(hdev);
    if (ret)
    return ret;
    return hid_hw_start(hdev, HID_CONNECT_DEFAULT);
    }
    static const struct hid_device_id vivaldi_table[] = {
    { HID_DEVICE(HID_BUS_ANY, HID_GROUP_VIVALDI, HID_ANY_ID, HID_ANY_ID) },
    { }
    };
    MODULE_DEVICE_TABLE(hid, vivaldi_table);
    static struct hid_driver hid_vivaldi = {
    .name = "hid-vivaldi",
    .id_table = vivaldi_table,
    .probe = vivaldi_probe,
    .feature_mapping = vivaldi_feature_mapping,
    .driver = {
    .dev_groups = vivaldi_attribute_groups,
    },
    };
    module_hid_driver(hid_vivaldi);
    MODULE_AUTHOR("Sean O'Brien");
    MODULE_DESCRIPTION("HID vivaldi driver");
    MODULE_LICENSE("GPL");
