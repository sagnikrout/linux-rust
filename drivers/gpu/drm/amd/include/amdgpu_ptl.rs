//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/include/amdgpu_ptl.h
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
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amdgpu_ptl_fmt {
    AMDGPU_PTL_FMT_I8   = 0,
    AMDGPU_PTL_FMT_F16  = 1,
    AMDGPU_PTL_FMT_BF16 = 2,
    AMDGPU_PTL_FMT_F32  = 3,
    AMDGPU_PTL_FMT_F64  = 4,
    AMDGPU_PTL_FMT_F8   = 5,
    AMDGPU_PTL_FMT_VECTOR  = 6,
    AMDGPU_PTL_FMT_INVALID = 7,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amdgpu_ptl_disable_source {
    AMDGPU_PTL_DISABLE_SYSFS = 0,
    AMDGPU_PTL_DISABLE_PROFILER,
    AMDGPU_PTL_DISABLE_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amdgpu_ptl_hw_supported_state {
    AMDGPU_PTL_HW_UNINIT = 0,       /* Not yet initialized */
    AMDGPU_PTL_HW_SUPPORTED,        /* Initialized and supported */
    AMDGPU_PTL_HW_NOT_SUPPORTED,    /* Initialized and not supported */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_ptl {
    pub fmt1: amdgpu_ptl_fmt,
    pub fmt2: amdgpu_ptl_fmt,
    pub enabled: bool,
    pub hw_supported_state: amdgpu_ptl_hw_supported_state,
    pub permanently_disabled: bool,
// PTL disable reference counting
    pub disable_ref: core::sync::atomic::AtomicI32,
    pub mutex: mutex,
    pub AMDGPU_PTL_DISABLE_MAX): DECLARE_BITMAP(disable_bitmap,,
    pub ptl_sysfs_created: bool,
}

extern "C" {
    pub fn amdgpu_ptl_sysfs_init(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_ptl_sysfs_fini(adev: *mut amdgpu_device);
}
