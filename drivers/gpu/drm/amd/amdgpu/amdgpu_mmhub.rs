//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/amdgpu/amdgpu_mmhub.h
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
// Copyright (C) 2019  Advanced Micro Devices, Inc.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included
// in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
// OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// THE COPYRIGHT HOLDER(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN
// AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_mmhub_ras {
    pub ras_block: amdgpu_ras_block_object,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_mmhub_funcs {
    pub adev): *mut *mut u64 (get_fb_location)(struct amdgpu_device,
    pub adev): *mut *mut u64 (get_mc_fb_offset)(struct amdgpu_device,
    pub adev): *mut *mut void (init)(struct amdgpu_device,
    pub adev): *mut *mut int (gart_enable)(struct amdgpu_device,
    pub value): bool,
    pub adev): *mut *mut void (gart_disable)(struct amdgpu_device,
    pub state): amd_clockgating_state,
    pub flags): *mut *mut *mut void (get_clockgating)(struct amdgpu_device adev, u64,
    pub page_table_base): u64,
    pub enable): bool,
    pub adev): *mut *mut int (get_xgmi_info)(struct amdgpu_device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_mmhub_client_ids {
    pub (*names)[2]: *const *const c_char,
    pub size: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_mmhub {
    pub ras_if: *mut ras_common_if,
    pub funcs: *const amdgpu_mmhub_funcs,
    pub ras: *mut amdgpu_mmhub_ras,
    pub client_ids: amdgpu_mmhub_client_ids,
}

extern "C" {
    pub fn amdgpu_mmhub_ras_sw_init(adev: *mut amdgpu_device) -> c_int;
}
