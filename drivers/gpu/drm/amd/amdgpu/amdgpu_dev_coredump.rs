//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/amdgpu/amdgpu_dev_coredump.h
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
// Copyright 2024 Advanced Micro Devices, Inc.
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
pub struct amdgpu_coredump_ring {
    pub rptr: u64,
    pub wptr: u64,
    pub ring_dw: *mut u32,
    pub ring_index: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_coredump_ib_info {
    pub gpu_addr: u64,
    pub ib_size_dw: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_coredump_info {
    pub adev: *mut amdgpu_device,
    pub reset_task_info: amdgpu_task_info,
    pub reset_time: timespec64,
    pub skip_vram_check: bool,
    pub reset_vram_lost: bool,
    pub ring: *mut amdgpu_ring,
    pub rings: *mut amdgpu_coredump_ring,
    pub num_rings: u32,
// Readable form of coredevdump, generate once to speed up
// reading it (see drm_coredump_printer's documentation).
//
    pub formatted_size: isize,
    pub formatted: *mut c_char,
    pub pasid: c_uint,
    pub vmid: c_uint,
    pub num_ibs: c_int,
    pub __counted_by(num_ibs): amdgpu_coredump_ib_info ibs[],
}

extern "C" {
    pub fn amdgpu_coredump_init(adev: *mut amdgpu_device);
}
extern "C" {
    pub fn amdgpu_coredump_fini(adev: *mut amdgpu_device);
}
