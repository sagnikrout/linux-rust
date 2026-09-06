//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/ras/ras_mgr/amdgpu_virt_ras_cmd.h
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


// SPDX-License-Identifier: MIT
//
// Copyright 2025 Advanced Micro Devices, Inc.
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
pub struct remote_batch_trace_mgr {
    pub batch_overview: ras_log_batch_overview,
    pub batch_trace: ras_cmd_batch_trace_record_rsp,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_virt_shared_mem {
    pub gpa: u64,
    pub cpu_addr: *mut c_void,
    pub size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vram_blocks_ecc {
    pub shared_mem: amdgpu_virt_shared_mem,
    pub auto_update_actived: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_virt_ras_cmd {
    pub remote_uniras_supported: bool,
    pub batch_mgr: remote_batch_trace_mgr,
    pub blocks_ecc: vram_blocks_ecc,
    pub remote_access_lock: mutex,
}

extern "C" {
    pub fn amdgpu_virt_ras_sw_init(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_virt_ras_sw_fini(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_virt_ras_hw_init(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_virt_ras_hw_fini(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_virt_ras_pre_reset(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_virt_ras_post_reset(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_virt_ras_set_remote_uniras(adev: *mut amdgpu_device, en: bool);
}
extern "C" {
    pub fn amdgpu_virt_ras_remote_uniras_enabled(adev: *mut amdgpu_device) -> bool;
}
