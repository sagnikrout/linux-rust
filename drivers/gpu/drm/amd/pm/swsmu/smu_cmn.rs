//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/pm/swsmu/smu_cmn.h
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
// Copyright 2020 Advanced Micro Devices, Inc.
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

extern "C" {
    pub fn smu_msg_wait_response(ctl: *mut smu_msg_ctl, timeout_us: u32) -> c_int;
}

pub const FDO_PWM_MODE_STATIC: c_int = 1;
pub const FDO_PWM_MODE_STATIC_RPM: c_int = 5;
pub const SMU_IH_INTERRUPT_ID_TO_DRIVER: c_uint = 0xFE;
pub const SMU_IH_INTERRUPT_CONTEXT_ID_BACO: c_uint = 0x2;
pub const SMU_IH_INTERRUPT_CONTEXT_ID_AC: c_uint = 0x3;
pub const SMU_IH_INTERRUPT_CONTEXT_ID_DC: c_uint = 0x4;
pub const SMU_IH_INTERRUPT_CONTEXT_ID_AUDIO_D0: c_uint = 0x5;
pub const SMU_IH_INTERRUPT_CONTEXT_ID_AUDIO_D3: c_uint = 0x6;
pub const SMU_IH_INTERRUPT_CONTEXT_ID_THERMAL_THROTTLING: c_uint = 0x7;
pub const SMU_IH_INTERRUPT_CONTEXT_ID_FAN_ABNORMAL: c_uint = 0x8;
pub const SMU_IH_INTERRUPT_CONTEXT_ID_FAN_RECOVERY: c_uint = 0x9;
pub const SMU_IGNORE_IF_VERSION: c_uint = 0xFFFFFFFF;

// Helper to Convert from PCIE Gen 1/2/3/4/5/6 to 0.1 GT/s speed units
extern "C" {
    pub fn smu_cmn_wait_for_response(smu: *mut smu_context) -> c_int;
}
extern "C" {
    pub fn smu_cmn_write_watermarks_table(smu: *mut smu_context) -> c_int;
}
extern "C" {
    pub fn smu_cmn_write_pptable(smu: *mut smu_context) -> c_int;
}
extern "C" {
    pub fn smu_cmn_get_combo_pptable(smu: *mut smu_context) -> c_int;
}
extern "C" {
    pub fn smu_cmn_is_audio_func_enabled(adev: *mut amdgpu_device) -> bool;
}
extern "C" {
    pub fn smu_cmn_generic_soc_policy_desc(policy: *mut smu_dpm_policy);
}
extern "C" {
    pub fn smu_cmn_generic_plpd_policy_desc(policy: *mut smu_dpm_policy);
}
extern "C" {
    pub fn smu_cmn_reset_custom_level(smu: *mut smu_context);
}
extern "C" {
    pub fn smu_cmn_dpm_pcie_gen_idx(gen: c_int) -> c_int;
}
extern "C" {
    pub fn smu_cmn_dpm_pcie_width_idx(width: c_int) -> c_int;
}
extern "C" {
    pub fn smu_cmn_check_fw_version(smu: *mut smu_context) -> c_int;
}
// SMU gpu metrics
// Attribute ID mapping

// Type ID mapping

// Unit ID mapping

// Map TYPEID to C type

// struct members

// Init functions for scalar/array fields - init to 0xFFs

// Declare Metrics Class and Template object

