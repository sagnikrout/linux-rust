//! Automatically rewritten from C to Rust
//! Source: drivers/base/firmware.c
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
// firmware.c - firmware subsystem hoohaw.
//
// Copyright (c) 2002-3 Patrick Mochel
// Copyright (c) 2002-3 Open Source Development Labs
// Copyright (c) 2007 Greg Kroah-Hartman <gregkh@suse.de>
// Copyright (c) 2007 Novell Inc.
//

    struct kobject *firmware_kobj;
    EXPORT_SYMBOL_GPL(firmware_kobj);
#[no_mangle]
pub unsafe extern "C" fn firmware_init() -> int __init {
    int __init firmware_init(void)
    {
    firmware_kobj = kobject_create_and_add("firmware", core::ptr::null_mut());
    if (!firmware_kobj)
    return -ENOMEM;
    return 0;
    }
