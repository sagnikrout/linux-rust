//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/amdgpu/amdgpu_userq_fence.h
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
// Copyright 2023 Advanced Micro Devices, Inc.
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
pub struct amdgpu_userq_fence {
    pub base: dma_fence,
//
// This lock is necessary to synchronize the
// userqueue dma fence operations.
//
    pub lock: spinlock_t,
    pub link: list_head,
    pub fence_drv_array_count: c_ulong,
    pub fence_drv: *mut amdgpu_userq_fence_driver,
    pub fence_drv_array: *mut amdgpu_userq_fence_driver,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_userq_fence_driver {
    pub refcount: kref,
    pub va: u64,
    pub gpu_addr: u64,
    pub cpu_addr: *mut u64,
    pub context: u64,
//
// This lock is necesaary to synchronize the access
// to the fences list by the fence driver.
//
    pub fence_list_lock: spinlock_t,
    pub fences: list_head,
    pub adev: *mut amdgpu_device,
    pub timeline_name: [c_char; TASK_COMM_LEN],
}

extern "C" {
    pub fn amdgpu_userq_fence_driver_get(fence_drv: *mut amdgpu_userq_fence_driver);
}
extern "C" {
    pub fn amdgpu_userq_fence_driver_put(fence_drv: *mut amdgpu_userq_fence_driver);
}
extern "C" {
    pub fn amdgpu_userq_fence_driver_free(userq: *mut amdgpu_usermode_queue);
}
extern "C" {
    pub fn amdgpu_userq_fence_driver_process(fence_drv: *mut amdgpu_userq_fence_driver) -> c_int;
}
extern "C" {
    pub fn amdgpu_userq_fence_driver_force_completion(userq: *mut amdgpu_usermode_queue);
}
extern "C" {
    pub fn amdgpu_userq_fence_driver_destroy(ref: *mut kref);
}
