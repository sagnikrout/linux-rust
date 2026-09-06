//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/amdgpu/amdgpu_uvd.h
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
// Copyright 2014 Advanced Micro Devices, Inc.
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
pub const AMDGPU_DEFAULT_UVD_HANDLES: c_int = 10;
pub const AMDGPU_MAX_UVD_HANDLES: c_int = 40;

pub const AMDGPU_UVD_FIRMWARE_OFFSET: c_int = 256;
pub const AMDGPU_MAX_UVD_INSTANCES: c_int = 2;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_uvd_inst {
    pub vcpu_bo: *mut amdgpu_bo,
    pub cpu_addr: *mut c_void,
    pub gpu_addr: u64,
    pub saved_bo: *mut c_void,
    pub ring: amdgpu_ring,
    pub ring_enc: [amdgpu_ring; AMDGPU_MAX_UVD_ENC_RINGS],
    pub irq: amdgpu_irq_src,
    pub srbm_soft_reset: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_uvd {
    pub /: *const *const *const firmware fw; / UVD firmware,
    pub fw_version: unsigned,
    pub max_handles: unsigned,
    pub num_enc_rings: unsigned,
    pub num_uvd_inst: u8,
    pub address_64_bit: bool,
    pub use_ctx_buf: bool,
    pub inst: [amdgpu_uvd_inst; AMDGPU_MAX_UVD_INSTANCES],
    pub filp: [*mut drm_file; AMDGPU_MAX_UVD_HANDLES],
    pub handles: [core::sync::atomic::AtomicI32; AMDGPU_MAX_UVD_HANDLES],
    pub entity: drm_sched_entity,
    pub idle_work: delayed_work,
    pub harvest_config: unsigned,
// store image width to adjust nb memory state
    pub decode_image_width: unsigned,
    pub keyselect: u32,
    pub ib_bo: *mut amdgpu_bo,
}

extern "C" {
    pub fn amdgpu_uvd_sw_init(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_uvd_sw_fini(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_uvd_entity_init(adev: *mut amdgpu_device, ring: *mut amdgpu_ring) -> c_int;
}
extern "C" {
    pub fn amdgpu_uvd_prepare_suspend(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_uvd_suspend(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_uvd_resume(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_uvd_ring_begin_use(ring: *mut amdgpu_ring);
}
extern "C" {
    pub fn amdgpu_uvd_ring_end_use(ring: *mut amdgpu_ring);
}
extern "C" {
    pub fn amdgpu_uvd_ring_test_ib(ring: *mut amdgpu_ring, timeout: c_long) -> c_int;
}
extern "C" {
    pub fn amdgpu_uvd_used_handles(adev: *mut amdgpu_device) -> u32;
}
