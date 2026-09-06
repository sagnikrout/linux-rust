//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/amdgpu/amdgpu_df.h
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
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_df_hash_status {
    pub hash_64k: bool,
    pub hash_2m: bool,
    pub hash_1g: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_df_funcs {
    pub adev): *mut *mut void (sw_init)(struct amdgpu_device,
    pub adev): *mut *mut void (sw_fini)(struct amdgpu_device,
    pub adev): *mut *mut void (hw_init)(struct amdgpu_device,
    pub enable): bool,
    pub adev): *mut *mut u32 (get_fb_channel_number)(struct amdgpu_device,
    pub adev): *mut *mut u32 (get_hbm_channel_number)(struct amdgpu_device,
    pub enable): bool,
    pub flags): *mut u64,
    pub enable): bool,
    pub is_add): int counter_idx, int,
    pub is_remove): int counter_idx, int,
    pub count): *mut int counter_idx, uint64_t,
    pub ficaa_val): *mut *mut *mut uint64_t (get_fica)(struct amdgpu_device adev, uint32_t,
    pub ficadh_val): uint32_t ficadl_val, uint32_t,
    pub adev): *mut *mut bool (query_ras_poison_mode)(struct amdgpu_device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_df {
    pub hash_status: amdgpu_df_hash_status,
    pub funcs: *const amdgpu_df_funcs,
}
