//! Automatically rewritten from C to Rust
//! Source: sound/soc/amd/acp/amd-sdw-acpi.c
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// This file is provided under a dual BSD/GPLv2 license. When using or
// redistributing this file, you may do so under either license.
//
// Copyright(c) 2023 Advanced Micro Devices, Inc. All rights reserved.
//
// Authors: Vijendar Mukunda <Vijendar.Mukunda@amd.com>
//
// SDW AMD ACPI scan helper function
//

#[no_mangle]
pub unsafe extern "C" fn amd_sdw_scan_controller(info: *mut sdw_amd_acpi_info) -> c_int {
    int amd_sdw_scan_controller(struct sdw_amd_acpi_info *info)
    {
    struct acpi_device *adev = acpi_fetch_acpi_dev(info.handle);
    let mut sdw_bitmap: u32 = 0;
    let mut count: u8 = 0;
    int ret;
    if (!adev)
    return -EINVAL;
// Found controller, find links supported
    ret = fwnode_property_read_u32_array(acpi_fwnode_handle(adev),
    "mipi-sdw-manager-list", &sdw_bitmap, 1);
    if (ret) {
    dev_err(&adev.dev,
    "Failed to read mipi-sdw-manager-list: %d\n", ret);
    return -EINVAL;
    }
    count = hweight32(sdw_bitmap);
// Check count is within bounds
    if (count > info.count) {
    dev_err(&adev.dev, "Manager count %d exceeds max %d\n",
    count, info.count);
    return -EINVAL;
    }
    if (!count) {
    dev_dbg(&adev.dev, "No SoundWire Managers detected\n");
    return -EINVAL;
    }
    dev_dbg(&adev.dev, "ACPI reports %d SoundWire Manager devices\n", count);
    info.link_mask = sdw_bitmap;
    return 0;
    }
    EXPORT_SYMBOL_NS(amd_sdw_scan_controller, "SND_AMD_SOUNDWIRE_ACPI");
    MODULE_LICENSE("Dual BSD/GPL");
    MODULE_DESCRIPTION("AMD SoundWire ACPI helpers");
