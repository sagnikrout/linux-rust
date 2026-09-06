//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/amdgpu/amdgpu_cs.h
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
// Copyright 2022 Advanced Micro Devices, Inc.
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

pub const AMDGPU_CS_GANG_SIZE: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_cs_chunk {
    pub chunk_id: u32,
    pub length_dw: u32,
    pub kdata: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_cs_post_dep {
    pub syncobj: *mut drm_syncobj,
    pub chain: *mut dma_fence_chain,
    pub point: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_cs_parser {
    pub adev: *mut amdgpu_device,
    pub filp: *mut drm_file,
    pub ctx: *mut amdgpu_ctx,
// chunks
    pub nchunks: unsigned,
    pub chunks: *mut amdgpu_cs_chunk,
// scheduler job objects
    pub gang_size: c_uint,
    pub gang_leader_idx: c_uint,
    pub entities: [*mut drm_sched_entity; AMDGPU_CS_GANG_SIZE],
    pub jobs: [*mut amdgpu_job; AMDGPU_CS_GANG_SIZE],
    pub gang_leader: *mut amdgpu_job,
// buffer objects
    pub exec: drm_exec,
    pub bo_list: *mut amdgpu_bo_list,
    pub mn: *mut amdgpu_mn,
    pub fence: *mut dma_fence,
    pub bytes_moved_threshold: u64,
    pub bytes_moved_vis_threshold: u64,
    pub bytes_moved: u64,
    pub bytes_moved_vis: u64,
// user fence
    pub uf_bo: *mut amdgpu_bo,
    pub num_post_deps: unsigned,
    pub post_deps: *mut amdgpu_cs_post_dep,
    pub sync: amdgpu_sync,
}
