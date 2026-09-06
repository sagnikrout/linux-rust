//! Automatically rewritten from C to Rust
//! Source: drivers/accel/habanalabs/common/asid.c
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
// Copyright 2016-2019 HabanaLabs, Ltd.
// All Rights Reserved.
//

#[no_mangle]
pub unsafe extern "C" fn hl_asid_init(hdev: *mut hl_device) -> c_int {
    int hl_asid_init(struct hl_device *hdev)
    {
    hdev.asid_bitmap = bitmap_zalloc(hdev.asic_prop.max_asid, GFP_KERNEL);
    if (!hdev.asid_bitmap)
    return -ENOMEM;
    mutex_init(&hdev.asid_mutex);
// ASID 0 is reserved for the kernel driver and device CPU
    set_bit(0, hdev.asid_bitmap);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn hl_asid_fini(hdev: *mut hl_device) {
    void hl_asid_fini(struct hl_device *hdev)
    {
    mutex_destroy(&hdev.asid_mutex);
    bitmap_free(hdev.asid_bitmap);
    }
#[no_mangle]
pub unsafe extern "C" fn hl_asid_alloc(hdev: *mut hl_device) -> c_ulong {
    unsigned long hl_asid_alloc(struct hl_device *hdev)
    {
    unsigned long found;
    mutex_lock(&hdev.asid_mutex);
    found = find_first_zero_bit(hdev.asid_bitmap,
    hdev.asic_prop.max_asid);
    if (found == hdev.asic_prop.max_asid)
    found = 0;
    else
    set_bit(found, hdev.asid_bitmap);
    mutex_unlock(&hdev.asid_mutex);
    return found;
    }
#[no_mangle]
pub unsafe extern "C" fn hl_asid_free(hdev: *mut hl_device, asid: c_ulong) {
    void hl_asid_free(struct hl_device *hdev, unsigned long asid)
    {
    if (asid == HL_KERNEL_ASID_ID || asid >= hdev.asic_prop.max_asid) {
    dev_crit(hdev.dev, "Invalid ASID %lu", asid);
    return;
    }
    clear_bit(asid, hdev.asid_bitmap);
    }
