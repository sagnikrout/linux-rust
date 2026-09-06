//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/amdgpu/amdgpu_wb.h
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

//
// Writeback
//

//
// struct amdgpu_wb - This struct is used for small GPU memory allocation.
//
// This struct is used to allocate a small amount of GPU memory that can be
// used to shadow certain states into the memory. This is especially useful for
// providing easy CPU access to some states without requiring register access
// (e.g., if some block is power gated, reading register may be problematic).
//
// Note: the term writeback was initially used because many of the amdgpu
// components had some level of writeback memory, and this struct initially
// described those components.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_wb {
//
// @wb_obj:
//
// Buffer Object used for the writeback memory.
//
    pub wb_obj: *mut amdgpu_bo,
//
// @wb:
//
// Pointer to the first writeback slot. In terms of CPU address
// this value can be accessed directly by using the offset as an index.
// For the GPU address, it is necessary to use gpu_addr and the offset.
//
    pub wb: *mut u32,
//
// @gpu_addr:
//
// Writeback base address in the GPU.
//
    pub gpu_addr: u64,
//
// @num_wb:
//
// Number of writeback slots reserved for amdgpu.
//
    pub num_wb: u32,
//
// @used:
//
// Track the writeback slot already used.
//
    pub BITS_PER_LONG)]: unsigned long used[DIV_ROUND_UP(AMDGPU_MAX_WB,,
//
// @lock:
//
// Protects read and write of the used field array.
//
    pub lock: spinlock_t,
}

extern "C" {
    pub fn amdgpu_wb_fini(adev: *mut amdgpu_device);
}
extern "C" {
    pub fn amdgpu_wb_init(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_wb_get(adev: *mut amdgpu_device, wb: *mut u32) -> c_int;
}
extern "C" {
    pub fn amdgpu_wb_free(adev: *mut amdgpu_device, wb: u32);
}
