//! Automatically rewritten from C to Rust
//! Source: drivers/input/misc/amd_sfh_tabletmode.c
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
// AMD SFH tablet-mode switch driver
//
// Copyright (c) 2026, Advanced Micro Devices, Inc.
// All Rights Reserved.
//
// Author: Basavaraj Natikar <Basavaraj.Natikar@amd.com>
//

pub const POLL_INTERVAL_MS: c_int = 200;
#[no_mangle]
unsafe extern "C" fn sfh_tm_poll(input: *mut input_dev) {
    static void sfh_tm_poll(struct input_dev *input)
    {
    let mut info: amd_sfh_info = {};
    if (amd_get_sfh_info(&info, MT_OP_MODE))
    return;
    input_report_switch(input, SW_TABLET_MODE,
    info.op_mode == SFH_MODE_TABLET);
    input_sync(input);
    }
    static int sfh_tm_probe(struct auxiliary_device *auxdev,
    const struct auxiliary_device_id *id)
    {
    struct device *dev = &auxdev.dev;
    let mut info: amd_sfh_info = {};
    struct input_dev *input;
    int error;
    error = amd_get_sfh_info(&info, MT_OP_MODE);
    if (error)
    let mut error: return = = -EINVAL ? -ENODEV : error;
    input = devm_input_allocate_device(dev);
    if (!input)
    return -ENOMEM;
    input.name		= "AMD SFH tablet mode switch";
    input.phys		= "amd-sfh/tabletmode";
    input.id.bustype	= BUS_HOST;
    input.id.vendor	= PCI_VENDOR_ID_AMD;
    input_set_capability(input, EV_SW, SW_TABLET_MODE);
    error = input_setup_polling(input, sfh_tm_poll);
    if (error)
    return error;
    input_set_poll_interval(input, POLL_INTERVAL_MS);
    sfh_tm_poll(input);
    error = input_register_device(input);
    if (error)
    return error;
    return 0;
    }
    static const struct auxiliary_device_id sfh_tm_id_table[] = {
    { .name = "amd_sfh.tabletmode" },
    {}
    };
    MODULE_DEVICE_TABLE(auxiliary, sfh_tm_id_table);
    static struct auxiliary_driver sfh_tm_driver = {
    .name		= "tabletmode",
    .id_table	= sfh_tm_id_table,
    .probe		= sfh_tm_probe,
    };
    module_auxiliary_driver(sfh_tm_driver);
    MODULE_DESCRIPTION("AMD SFH tablet mode switch");
    MODULE_LICENSE("GPL");
