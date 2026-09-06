//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/amdgpu/amdgpu_acpi.h
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


// SPDX-License-Identifier: GPL-2.0 OR MIT
//
// Copyright 2026 Advanced Micro Devices, Inc.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
// OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
// ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
// OTHER DEALINGS IN THE SOFTWARE.
//

pub const MAX_UMA_OPTION_NAME: c_int = 28;
pub const MAX_UMA_OPTION_ENTRIES: c_int = 19;

// ATCS Device/Driver State
pub const AMDGPU_ATCS_PSC_DEV_STATE_D0: c_int = 0;
pub const AMDGPU_ATCS_PSC_DEV_STATE_D3_HOT: c_int = 3;
pub const AMDGPU_ATCS_PSC_DRV_STATE_OPR: c_int = 0;
pub const AMDGPU_ATCS_PSC_DRV_STATE_NOT_OPR: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amdgpu_ss {
    AMDGPU_SS_DRV_LOAD,
    AMDGPU_SS_DEV_D0,
    AMDGPU_SS_DEV_D3,
    AMDGPU_SS_DRV_UNLOAD
}

//
// struct amdgpu_uma_carveout_option - single UMA carveout option
// @name: Name of the carveout option
// @memory_carved_mb: Amount of memory carved in MB
// @flags: ATCS flags supported by this option
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_uma_carveout_option {
    pub name: [c_char; MAX_UMA_OPTION_NAME],
    pub memory_carved_mb: u32,
    pub flags: u8,
}

//
// struct amdgpu_uma_carveout_info - table of available UMA carveout options
// @num_entries: Number of available options
// @uma_option_index: The index of the option currently applied
// @update_lock: Lock to serialize changes to the option
// @entries: The array of carveout options
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_uma_carveout_info {
    pub num_entries: u8,
    pub uma_option_index: u8,
    pub update_lock: mutex,
    pub entries: [amdgpu_uma_carveout_option; MAX_UMA_OPTION_ENTRIES],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_numa_info {
    pub size: u64,
    pub pxm: c_int,
    pub nid: c_int,
}

extern "C" {
    pub fn amdgpu_acpi_init(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_acpi_fini(adev: *mut amdgpu_device);
}
extern "C" {
    pub fn amdgpu_acpi_is_pcie_performance_request_supported(adev: *mut amdgpu_device) -> bool;
}
extern "C" {
    pub fn amdgpu_acpi_is_power_shift_control_supported() -> bool;
}
extern "C" {
    pub fn amdgpu_acpi_is_set_uma_allocation_size_supported() -> bool;
}
extern "C" {
    pub fn amdgpu_acpi_set_uma_allocation_size(adev: *mut amdgpu_device, index: u8, type: u8) -> c_int;
}
extern "C" {
    pub fn amdgpu_acpi_pcie_notify_device_ready(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_acpi_get_backlight_caps(caps: *mut amdgpu_dm_backlight_caps);
}
extern "C" {
    pub fn amdgpu_acpi_should_gpu_reset(adev: *mut amdgpu_device) -> bool;
}
extern "C" {
    pub fn amdgpu_acpi_detect();
}
extern "C" {
    pub fn amdgpu_acpi_release();
}

extern "C" {
    pub fn amdgpu_acpi_is_s3_active(adev: *mut amdgpu_device) -> bool;
}
extern "C" {
    pub fn amdgpu_acpi_is_s0ix_active(adev: *mut amdgpu_device) -> bool;
}

extern "C" {
    pub fn amdgpu_acpi_get_isp4_dev(dev: *mut acpi_device) -> c_int;
}

