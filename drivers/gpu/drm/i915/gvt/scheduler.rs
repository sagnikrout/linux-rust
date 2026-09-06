//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/gvt/scheduler.h
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
// Copyright(c) 2011-2016 Intel Corporation. All rights reserved.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice (including the next
// paragraph) shall be included in all copies or substantial portions of the
// Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//
// Authors:
// Zhi Wang <zhi.a.wang@intel.com>
//
// Contributors:
// Ping Gao <ping.a.gao@intel.com>
// Tina Zhang <tina.zhang@intel.com>
// Chanbin Du <changbin.du@intel.com>
// Min He <min.he@intel.com>
// Bing Niu <bing.niu@intel.com>
// Zhenyu Wang <zhenyuw@linux.intel.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_gvt_workload_scheduler {
    pub current_vgpu: *mut intel_vgpu,
    pub next_vgpu: *mut intel_vgpu,
    pub current_workload: [*mut intel_vgpu_workload; I915_NUM_ENGINES],
    pub need_reschedule: bool,
    pub mmio_context_lock: spinlock_t,
// can be null when owner is host
    pub engine_owner: [*mut intel_vgpu; I915_NUM_ENGINES],
    pub workload_complete_wq: wait_queue_head_t,
    pub thread: [*mut task_struct; I915_NUM_ENGINES],
    pub waitq: [wait_queue_head_t; I915_NUM_ENGINES],
    pub sched_data: *mut c_void,
    pub sched_ops: *const intel_gvt_sched_policy_ops,
}

pub const INDIRECT_CTX_ADDR_MASK: c_uint = 0xffffffc0;
pub const INDIRECT_CTX_SIZE_MASK: c_uint = 0x3f;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct shadow_indirect_ctx {
    pub obj: *mut drm_i915_gem_object,
    pub guest_gma: c_ulong,
    pub shadow_gma: c_ulong,
    pub shadow_va: *mut c_void,
    pub size: u32,
}

pub const PER_CTX_ADDR_MASK: c_uint = 0xfffff000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct shadow_per_ctx {
    pub guest_gma: c_ulong,
    pub shadow_gma: c_ulong,
    pub valid: unsigned,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_shadow_wa_ctx {
    pub indirect_ctx: shadow_indirect_ctx,
    pub per_ctx: shadow_per_ctx,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_vgpu_workload {
    pub vgpu: *mut intel_vgpu,
    pub engine: *const intel_engine_cs,
    pub req: *mut i915_request,
// if this workload has been dispatched to i915?
    pub dispatched: bool,
    pub /: *mut *mut bool shadow; / if workload has done shadow of guest request,
    pub status: c_int,
    pub shadow_mm: *mut intel_vgpu_mm,
    pub /: *mut *mut list_head lri_shadow_mm; / For PPGTT load cmd,
// different submission model may need different handler
    pub ): *mut *mut int (prepare)(struct intel_vgpu_workload,
    pub ): *mut *mut int (complete)(struct intel_vgpu_workload,
    pub list: list_head,
    pub INTEL_GVT_EVENT_MAX): DECLARE_BITMAP(pending_events,,
    pub shadow_ring_buffer_va: *mut c_void,
// execlist context information
    pub ctx_desc: execlist_ctx_descriptor_format,
    pub rb_len: unsigned long rb_head, rb_tail, rb_ctl, rb_start,,
    pub guest_rb_head: c_ulong,
    pub elsp_dwords: intel_vgpu_elsp_dwords,
    pub emulate_schedule_in: bool,
    pub shadow_ctx_active: core::sync::atomic::AtomicI32,
    pub shadow_ctx_status_wq: wait_queue_head_t,
    pub ring_context_gpa: u64,
// shadow batch buffer
    pub shadow_bb: list_head,
    pub wa_ctx: intel_shadow_wa_ctx,
// oa registers
    pub oactxctrl: u32,
    pub flex_mmio: [u32; 7],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_vgpu_shadow_bb {
    pub list: list_head,
    pub obj: *mut drm_i915_gem_object,
    pub vma: *mut i915_vma,
    pub va: *mut c_void,
    pub bb_start_cmd_va: *mut u32,
    pub bb_offset: c_ulong,
    pub ppgtt: bool,
}

extern "C" {
    pub fn intel_vgpu_queue_workload(workload: *mut intel_vgpu_workload);
}
extern "C" {
    pub fn intel_gvt_init_workload_scheduler(gvt: *mut intel_gvt) -> c_int;
}
extern "C" {
    pub fn intel_gvt_clean_workload_scheduler(gvt: *mut intel_gvt);
}
extern "C" {
    pub fn intel_gvt_wait_vgpu_idle(vgpu: *mut intel_vgpu);
}
extern "C" {
    pub fn intel_vgpu_setup_submission(vgpu: *mut intel_vgpu) -> c_int;
}
extern "C" {
    pub fn intel_vgpu_clean_submission(vgpu: *mut intel_vgpu);
}
extern "C" {
    pub fn intel_vgpu_destroy_workload(workload: *mut intel_vgpu_workload);
}
