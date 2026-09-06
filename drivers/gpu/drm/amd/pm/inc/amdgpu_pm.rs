//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/pm/inc/amdgpu_pm.h
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


//
// Copyright 2014 Advanced Micro Devices, Inc.
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
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cg_flag_name {
    pub flag: u64,
    pub name: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amdgpu_device_attr_flags {
    ATTR_FLAG_BASIC = (1 << 0),
    ATTR_FLAG_ONEVF = (1 << 16),
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amdgpu_device_attr_states {
    ATTR_STATE_UNSUPPORTED = 0,
    ATTR_STATE_SUPPORTED,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amdgpu_device_attr_id {
    device_attr_id__unknown = -1,
    device_attr_id__power_dpm_state = 0,
    device_attr_id__power_dpm_force_performance_level,
    device_attr_id__pp_num_states,
    device_attr_id__pp_cur_state,
    device_attr_id__pp_force_state,
    device_attr_id__pp_table,
    device_attr_id__pp_dpm_sclk,
    device_attr_id__pp_dpm_mclk,
    device_attr_id__pp_dpm_socclk,
    device_attr_id__pp_dpm_fclk,
    device_attr_id__pp_dpm_vclk,
    device_attr_id__pp_dpm_vclk1,
    device_attr_id__pp_dpm_dclk,
    device_attr_id__pp_dpm_dclk1,
    device_attr_id__pp_dpm_dcefclk,
    device_attr_id__pp_dpm_pcie,
    device_attr_id__pp_sclk_od,
    device_attr_id__pp_mclk_od,
    device_attr_id__pp_power_profile_mode,
    device_attr_id__pp_od_clk_voltage,
    device_attr_id__gpu_busy_percent,
    device_attr_id__mem_busy_percent,
    device_attr_id__vcn_busy_percent,
    device_attr_id__pcie_bw,
    device_attr_id__pp_features,
    device_attr_id__unique_id,
    device_attr_id__thermal_throttling_logging,
    device_attr_id__apu_thermal_cap,
    device_attr_id__gpu_metrics,
    device_attr_id__smartshift_apu_power,
    device_attr_id__smartshift_dgpu_power,
    device_attr_id__smartshift_bias,
    device_attr_id__pm_metrics,
    device_attr_id__count,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_device_attr {
    pub dev_attr: device_attribute,
    pub attr_id: amdgpu_device_attr_id,
    pub flags: amdgpu_device_attr_flags,
    pub states): *mut uint32_t mask, enum amdgpu_device_attr_states,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_device_attr_entry {
    pub entry: list_head,
    pub attr: *mut amdgpu_device_attr,
}

extern "C" {
    pub fn amdgpu_pm_sysfs_init(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_pm_virt_sysfs_init(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_pm_sysfs_fini(adev: *mut amdgpu_device);
}
extern "C" {
    pub fn amdgpu_pm_virt_sysfs_fini(adev: *mut amdgpu_device);
}
extern "C" {
    pub fn amdgpu_debugfs_pm_init(adev: *mut amdgpu_device);
}
