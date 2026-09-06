//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/amdgpu/amdgpu_ctx.h
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
// Copyright 2018 Advanced Micro Devices, Inc.
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

pub const AMDGPU_MAX_ENTITY_NUM: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_ctx_entity {
    pub hw_ip: u32,
    pub sequence: u64,
    pub entity: drm_sched_entity,
    pub fences: [*mut dma_fence; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_ctx {
    pub refcount: kref,
    pub ring_lock: spinlock_t,
    pub reset_counter: unsigned,
    pub reset_counter_query: unsigned,
    pub init_priority: i32,
    pub override_priority: i32,
    pub stable_pstate: u32,
    pub preamble_presented: bool,
    pub generation: u64,
    pub ras_counter_ce: c_ulong,
    pub ras_counter_ue: c_ulong,
    pub mgr: *mut amdgpu_ctx_mgr,
    pub entities: [*mut amdgpu_ctx_entity; AMDGPU_HW_IP_NUM][AMDGPU_MAX_ENTITY_NUM],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_ctx_mgr {
    pub adev: *mut amdgpu_device,
    pub ctx_handles: xarray,
    pub time_spend: [core::sync::atomic::AtomicI64; AMDGPU_HW_IP_NUM],
}

extern "C" {
    pub fn amdgpu_ctx_fini(kref: *mut kref);
}
extern "C" {
    pub fn amdgpu_ctx_priority_is_valid(ctx_prio: i32) -> bool;
}
extern "C" {
    pub fn amdgpu_ctx_priority_override(ctx: *mut amdgpu_ctx, ctx_prio: i32);
}
extern "C" {
    pub fn amdgpu_ctx_mgr_entity_flush(mgr: *mut amdgpu_ctx_mgr, timeout: c_long) -> c_long;
}
extern "C" {
    pub fn amdgpu_ctx_mgr_fini(mgr: *mut amdgpu_ctx_mgr);
}
