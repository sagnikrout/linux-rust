//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/amdgpu/amdgpu_userq.h
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

pub const AMDGPU_MAX_USERQ_COUNT: c_int = 512;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amdgpu_userq_state {
    AMDGPU_USERQ_STATE_UNMAPPED = 0,
    AMDGPU_USERQ_STATE_MAPPED,
    AMDGPU_USERQ_STATE_PREEMPTED,
    AMDGPU_USERQ_STATE_HUNG,
    AMDGPU_USERQ_STATE_INVALID_VA,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_userq_obj {
    pub cpu_ptr: *mut c_void,
    pub gpu_addr: u64,
    pub obj: *mut amdgpu_bo,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_usermode_queue {
    pub queue_type: c_int,
    pub state: amdgpu_userq_state,
    pub doorbell_handle: u64,
    pub doorbell_index: u64,
    pub doorbell_offset: u32,
    pub flags: u64,
    pub userq_prop: *mut amdgpu_mqd_prop,
    pub userq_mgr: *mut amdgpu_userq_mgr,
    pub vm: *mut amdgpu_vm,
    pub mqd: amdgpu_userq_obj,
    pub db_obj: amdgpu_userq_obj,
    pub fw_obj: amdgpu_userq_obj,
    pub wptr_obj: amdgpu_userq_obj,
//
// @fence_drv_lock: Protecting @fence_drv_xa.
//
    pub fence_drv_lock: mutex,
//
// @fence_drv_xa:
//
// References to the external fence drivers returned by wait_ioctl.
// Dropped on the next signaled dma_fence or queue destruction.
//
    pub fence_drv_xa: xarray,
    pub fence_drv: *mut amdgpu_userq_fence_driver,
    pub last_fence: *mut dma_fence,
    pub xcp_id: u32,
    pub priority: c_int,
    pub debugfs_queue: *mut dentry,
//
// @hang_detect_work:
//
// Delayed work which runs when userq_fences time out.
//
    pub hang_detect_work: delayed_work,
    pub refcount: kref,
    pub queue_rb: u64,
    pub wptr: u64,
    pub rptr: u64,
    pub eop: u64,
    pub shadow: u64,
    pub csa: u64,
    pub va: },
    pub va_array: [u64; 6],
    pub userq_vas: },
    pub gang_ctx_array_index: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_userq_funcs {
    pub args): *mut drm_amdgpu_userq_in,
    pub args): *mut drm_amdgpu_userq_in,
    pub uq): *mut *mut void (mqd_destroy)(struct amdgpu_usermode_queue,
    pub queue): *mut *mut int (unmap)(struct amdgpu_usermode_queue,
    pub queue): *mut *mut int (map)(struct amdgpu_usermode_queue,
    pub queue): *mut *mut int (preempt)(struct amdgpu_usermode_queue,
    pub queue): *mut *mut int (restore)(struct amdgpu_usermode_queue,
    pub queue): *mut *mut int (reset)(struct amdgpu_usermode_queue,
}

// Usermode queues for gfx
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_userq_mgr {
//
// @userq_xa: Per-process user queue map (queue ID → queue)
// Key: queue_id (unique ID within the process's userq manager)
// Value: struct amdgpu_usermode_queue
//
    pub userq_xa: xarray,
    pub userq_mutex: mutex,
    pub adev: *mut amdgpu_device,
    pub resume_work: delayed_work,
    pub file: *mut drm_file,
    pub proc_ctx_lock: mutex,
    pub proc_ctx_obj: amdgpu_userq_obj,
    pub proc_ctx_allocated: bool,
    pub proc_ctx_array_index: u32,
//
// @reset_work:
//
// Reset work which is used when eviction fails.
//
    pub reset_work: work_struct,
    pub userq_count: [core::sync::atomic::AtomicI32; AMDGPU_RING_TYPE_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_db_info {
    pub doorbell_handle: u64,
    pub queue_type: u32,
    pub doorbell_offset: u32,
    pub db_obj: *mut amdgpu_userq_obj,
}

extern "C" {
    pub fn amdgpu_userq_put(queue: *mut amdgpu_usermode_queue);
}
extern "C" {
    pub fn amdgpu_userq_ioctl(dev: *mut drm_device, data: *mut c_void, filp: *mut drm_file) -> c_int;
}
extern "C" {
    pub fn amdgpu_userq_mgr_cancel_reset_work(adev: *mut amdgpu_device);
}
extern "C" {
    pub fn amdgpu_userq_mgr_cancel_resume(userq_mgr: *mut amdgpu_userq_mgr);
}
extern "C" {
    pub fn amdgpu_userq_mgr_fini(userq_mgr: *mut amdgpu_userq_mgr);
}
extern "C" {
    pub fn amdgpu_userq_evict(uq_mgr: *mut amdgpu_userq_mgr);
}
extern "C" {
    pub fn amdgpu_userq_get_supported_ip_mask(adev: *mut amdgpu_device) -> u32;
}
extern "C" {
    pub fn amdgpu_userq_enabled(dev: *mut drm_device) -> bool;
}
extern "C" {
    pub fn amdgpu_userq_suspend(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_userq_resume(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_userq_reset_work(work: *mut work_struct);
}
extern "C" {
    pub fn amdgpu_userq_pre_reset(adev: *mut amdgpu_device);
}
extern "C" {
    pub fn amdgpu_userq_post_reset(adev: *mut amdgpu_device, vram_lost: bool) -> c_int;
}
extern "C" {
    pub fn amdgpu_userq_start_hang_detect_work(queue: *mut amdgpu_usermode_queue);
}
extern "C" {
    pub fn amdgpu_userq_process_fence_irq(adev: *mut amdgpu_device, doorbell: u32);
}
//
// CP packs the per-process doorbell_id of the queue in
// CTXID0[9:0] on priv-fault (same encoding KFD uses via
// KFD_CTXID0_DOORBELL_ID_MASK)
//
pub const AMDGPU_CTXID0_DOORBELL_ID_MASK: c_uint = 0x3ff;
