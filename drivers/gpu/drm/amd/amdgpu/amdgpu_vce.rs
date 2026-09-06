//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/amdgpu/amdgpu_vce.h
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
pub const AMDGPU_MAX_VCE_HANDLES: c_int = 16;
pub const AMDGPU_VCE_FIRMWARE_OFFSET: c_int = 256;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_vce {
    pub vcpu_bo: *mut amdgpu_bo,
    pub gpu_addr: u64,
    pub cpu_addr: *mut c_void,
    pub saved_bo: *mut c_void,
    pub fw_version: unsigned,
    pub fb_version: unsigned,
    pub handles: [core::sync::atomic::AtomicI32; AMDGPU_MAX_VCE_HANDLES],
    pub filp: [*mut drm_file; AMDGPU_MAX_VCE_HANDLES],
    pub img_size: [u32; AMDGPU_MAX_VCE_HANDLES],
    pub idle_work: delayed_work,
    pub idle_mutex: mutex,
    pub /: *const *const *const firmware fw; / VCE firmware,
    pub ring: [amdgpu_ring; AMDGPU_MAX_VCE_RINGS],
    pub irq: amdgpu_irq_src,
    pub harvest_config: unsigned,
    pub entity: drm_sched_entity,
    pub srbm_soft_reset: u32,
    pub num_rings: unsigned,
    pub keyselect: u32,
    pub gart_node: drm_mm_node,
}

extern "C" {
    pub fn amdgpu_vce_early_init(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_vce_sw_init(adev: *mut amdgpu_device, size: c_ulong) -> c_int;
}
extern "C" {
    pub fn amdgpu_vce_sw_fini(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_vce_entity_init(adev: *mut amdgpu_device, ring: *mut amdgpu_ring) -> c_int;
}
extern "C" {
    pub fn amdgpu_vce_suspend(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_vce_resume(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_vce_free_handles(adev: *mut amdgpu_device, filp: *mut drm_file);
}
extern "C" {
    pub fn amdgpu_vce_ring_test_ring(ring: *mut amdgpu_ring) -> c_int;
}
extern "C" {
    pub fn amdgpu_vce_ring_test_ib(ring: *mut amdgpu_ring, timeout: c_long) -> c_int;
}
extern "C" {
    pub fn amdgpu_vce_ring_begin_use(ring: *mut amdgpu_ring);
}
extern "C" {
    pub fn amdgpu_vce_ring_end_use(ring: *mut amdgpu_ring);
}
extern "C" {
    pub fn amdgpu_vce_ring_get_emit_ib_size(ring: *mut amdgpu_ring) -> unsigned;
}
extern "C" {
    pub fn amdgpu_vce_ring_get_dma_frame_size(ring: *mut amdgpu_ring) -> unsigned;
}
extern "C" {
    pub fn amdgpu_vce_get_ring_prio(ring: c_int) -> amdgpu_ring_priority_level;
}
