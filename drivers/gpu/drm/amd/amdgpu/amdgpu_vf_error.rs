//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/amdgpu/amdgpu_vf_error.h
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
// Copyright 2017 Advanced Micro Devices, Inc.
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

// Please keep enum same as AMD GIM driver
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum AMDGIM_ERROR_VF {
    AMDGIM_ERROR_VF_ATOMBIOS_INIT_FAIL = 0,
    AMDGIM_ERROR_VF_NO_VBIOS,
    AMDGIM_ERROR_VF_GPU_POST_ERROR,
    AMDGIM_ERROR_VF_ATOMBIOS_GET_CLOCK_FAIL,
    AMDGIM_ERROR_VF_FENCE_INIT_FAIL,

    AMDGIM_ERROR_VF_AMDGPU_INIT_FAIL,
    AMDGIM_ERROR_VF_IB_INIT_FAIL,
    AMDGIM_ERROR_VF_AMDGPU_LATE_INIT_FAIL,
    AMDGIM_ERROR_VF_ASIC_RESUME_FAIL,
    AMDGIM_ERROR_VF_GPU_RESET_FAIL,

    AMDGIM_ERROR_VF_TEST,
    AMDGIM_ERROR_VF_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum AMDGIM_ERROR_CATEGORY {
    AMDGIM_ERROR_CATEGORY_NON_USED = 0,
    AMDGIM_ERROR_CATEGORY_GIM,
    AMDGIM_ERROR_CATEGORY_PF,
    AMDGIM_ERROR_CATEGORY_VF,
    AMDGIM_ERROR_CATEGORY_VBIOS,
    AMDGIM_ERROR_CATEGORY_MONITOR,

    AMDGIM_ERROR_CATEGORY_MAX
}

extern "C" {
    pub fn amdgpu_vf_error_trans_all(adev: *mut amdgpu_device);
}
