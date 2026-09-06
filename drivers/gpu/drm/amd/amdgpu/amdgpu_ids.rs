//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/amdgpu/amdgpu_ids.h
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

// maximum number of VMIDs
pub const AMDGPU_NUM_VMID: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_vmid {
    pub list: list_head,
    pub active: amdgpu_sync,
    pub last_flush: *mut dma_fence,
    pub owner: u64,
    pub pd_gpu_addr: u64,
// last flushed PD/PT update
    pub flushed_updates: u64,
    pub current_gpu_reset_count: u32,
    pub gds_base: u32,
    pub gds_size: u32,
    pub gws_base: u32,
    pub gws_size: u32,
    pub oa_base: u32,
    pub oa_size: u32,
    pub pasid: unsigned,
    pub pasid_mapping: *mut dma_fence,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_vmid_mgr {
    pub lock: mutex,
    pub num_ids: unsigned,
    pub ids_lru: list_head,
    pub ids: [amdgpu_vmid; AMDGPU_NUM_VMID],
    pub reserved_vmid: bool,
}

extern "C" {
    pub fn amdgpu_pasid_alloc(bits: c_uint, fpriv: *mut amdgpu_fpriv) -> c_int;
}
extern "C" {
    pub fn amdgpu_pasid_lock(flags: *mut c_ulong);
}
extern "C" {
    pub fn amdgpu_pasid_unlock(flags: c_ulong);
}
extern "C" {
    pub fn amdgpu_pasid_free(pasid: u32);
}
extern "C" {
    pub fn amdgpu_pasid_mgr_cleanup();
}
extern "C" {
    pub fn amdgpu_vmid_uses_reserved(vm: *mut amdgpu_vm, vmhub: c_uint) -> bool;
}
extern "C" {
    pub fn amdgpu_vmid_reset_all(adev: *mut amdgpu_device);
}
extern "C" {
    pub fn amdgpu_vmid_mgr_init(adev: *mut amdgpu_device);
}
extern "C" {
    pub fn amdgpu_vmid_mgr_fini(adev: *mut amdgpu_device);
}
