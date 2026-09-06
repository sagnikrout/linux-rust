//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/amdgpu/amdgpu_ih.h
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
// Maximum number of IVs processed at once
pub const AMDGPU_IH_MAX_NUM_IVS: c_int = 32;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_ih_regs {
    pub ih_rb_base: u32,
    pub ih_rb_base_hi: u32,
    pub ih_rb_cntl: u32,
    pub ih_rb_wptr: u32,
    pub ih_rb_rptr: u32,
    pub ih_doorbell_rptr: u32,
    pub ih_rb_wptr_addr_lo: u32,
    pub ih_rb_wptr_addr_hi: u32,
    pub psp_reg_id: u32,
}

//
// R6xx+ IH ring
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_ih_ring {
    pub ring_size: unsigned,
    pub ptr_mask: u32,
    pub doorbell_index: u32,
    pub use_doorbell: bool,
    pub use_bus_addr: bool,
    pub ring_obj: *mut amdgpu_bo,
    pub ring: *mut u32,
    pub gpu_addr: u64,
    pub wptr_addr: u64,
    pub wptr_cpu: *mut u32,
    pub rptr_addr: u64,
    pub rptr_cpu: *mut u32,
    pub enabled: bool,
    pub rptr: unsigned,
    pub ih_regs: amdgpu_ih_regs,
// For waiting on IH processing at checkpoint.
    pub wait_process: wait_queue_head_t,
    pub processed_timestamp: u64,
    pub overflow: bool,
}

// return true if time stamp t2 is after t1 with 48bit wrap around

// provided by the ih block
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_ih_funcs {
// ring read/write ptr handling, called from interrupt context
    pub ih): *mut *mut *mut u32 (get_wptr)(struct amdgpu_device adev, struct amdgpu_ih_ring,
    pub entry): *mut amdgpu_iv_entry,
    pub offset): signed int,
    pub ih): *mut *mut *mut void (set_rptr)(struct amdgpu_device adev, struct amdgpu_ih_ring,
// Decode IH cookie node_id into a human-readable die name string.
// Returns buf, or NULL if this IH version does not support node_id decoding.
//
    pub size): *mut *mut char buf, size_t,
}

extern "C" {
    pub fn amdgpu_ih_ring_fini(adev: *mut amdgpu_device, ih: *mut amdgpu_ih_ring);
}
extern "C" {
    pub fn amdgpu_ih_process(adev: *mut amdgpu_device, ih: *mut amdgpu_ih_ring) -> c_int;
}
