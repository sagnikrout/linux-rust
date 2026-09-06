//! Automatically rewritten from C to Rust
//! Source: drivers/i2c/busses/i2c-designware-baytrail.c
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
// Intel BayTrail PMIC I2C bus semaphore implementation
// Copyright (c) 2014, Intel Corporation.
//

#[no_mangle]
pub unsafe extern "C" fn i2c_dw_baytrail_probe_lock_support(dev: *mut dw_i2c_dev) -> c_int {
    int i2c_dw_baytrail_probe_lock_support(struct dw_i2c_dev *dev)
    {
    acpi_status status;
    let mut shared_host: c_ulonglong = 0;
    acpi_handle handle;
    if (!dev)
    return -ENODEV;
    handle = ACPI_HANDLE(dev.dev);
    if (!handle)
    return -ENODEV;
    status = acpi_evaluate_integer(handle, "_SEM", core::ptr::null_mut(), &shared_host);
    if (ACPI_FAILURE(status))
    return -ENODEV;
    if (!shared_host)
    return -ENODEV;
    if (!iosf_mbi_available())
    return -EPROBE_DEFER;
    dev_info(dev.dev, "I2C bus managed by PUNIT\n");
    dev.acquire_lock = iosf_mbi_block_punit_i2c_access;
    dev.release_lock = iosf_mbi_unblock_punit_i2c_access;
    dev.shared_with_punit = true;
    return 0;
    }
