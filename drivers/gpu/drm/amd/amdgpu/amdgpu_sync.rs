//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/amdgpu/amdgpu_sync.h
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
// Copyright 2016 Advanced Micro Devices, Inc.
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
// Authors: Christian König
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amdgpu_sync_mode {
    AMDGPU_SYNC_ALWAYS,
    AMDGPU_SYNC_NE_OWNER,
    AMDGPU_SYNC_EQ_OWNER,
    AMDGPU_SYNC_EXPLICIT
}

//
// Container for fences used to sync command submissions.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_sync {
    pub 4): DECLARE_HASHTABLE(fences,,
}

extern "C" {
    pub fn amdgpu_sync_create(sync: *mut amdgpu_sync);
}
extern "C" {
    pub fn amdgpu_sync_kfd(sync: *mut amdgpu_sync, resv: *mut dma_resv) -> c_int;
}
extern "C" {
    pub fn amdgpu_sync_clone(source: *mut amdgpu_sync, clone: *mut amdgpu_sync) -> c_int;
}
extern "C" {
    pub fn amdgpu_sync_move(src: *mut amdgpu_sync, dst: *mut amdgpu_sync);
}
extern "C" {
    pub fn amdgpu_sync_push_to_job(sync: *mut amdgpu_sync, job: *mut amdgpu_job) -> c_int;
}
extern "C" {
    pub fn amdgpu_sync_wait(sync: *mut amdgpu_sync, intr: bool) -> c_int;
}
extern "C" {
    pub fn amdgpu_sync_free(sync: *mut amdgpu_sync);
}
extern "C" {
    pub fn amdgpu_sync_init() -> c_int;
}
extern "C" {
    pub fn amdgpu_sync_fini();
}
