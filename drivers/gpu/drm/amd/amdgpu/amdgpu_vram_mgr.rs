//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/amdgpu/amdgpu_vram_mgr.h
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
// Copyright 2021 Advanced Micro Devices, Inc.
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
pub struct amdgpu_vram_mgr {
    pub manager: ttm_resource_manager,
    pub mm: gpu_buddy,
// protects access to buffer objects
    pub lock: mutex,
    pub reservations_pending: list_head,
    pub reserved_pages: list_head,
    pub vis_usage: core::sync::atomic::AtomicI64,
    pub default_page_size: u64,
    pub allocated_vres_list: list_head,
    pub cg_region: *mut dmem_cgroup_region,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_vres_task {
    pub pid: pid_t,
    pub comm: [c_char; TASK_COMM_LEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_vram_block_info {
    pub start: u64,
    pub size: u64,
    pub task: amdgpu_vres_task,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_vram_mgr_resource {
    pub base: ttm_resource,
    pub blocks: list_head,
    pub flags: c_ulong,
    pub vres_node: list_head,
    pub task: amdgpu_vres_task,
}

extern "C" {
    pub fn gpu_buddy_block_offset(_arg: block) -> return;
}
extern "C" {
    pub fn gpu_buddy_block_is_clear(_arg: block) -> return;
}
extern "C" {
    pub fn container_of(_arg: res, amdgpu_vram_mgr_resource: struct, _arg: base) -> return;
}
