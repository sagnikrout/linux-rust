//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/i915_gpu_error.h
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
// SPDX-License-Identifier: MIT
//
// Copyright © 2008-2018 Intel Corporation
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i915_vma_coredump {
    pub next: *mut i915_vma_coredump,
    pub name: [c_char; 20],
    pub gtt_offset: u64,
    pub gtt_size: u64,
    pub gtt_page_sizes: u32,
    pub unused: c_int,
    pub page_list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i915_request_coredump {
    pub flags: c_ulong,
    pub pid: pid_t,
    pub context: u32,
    pub seqno: u32,
    pub head: u32,
    pub tail: u32,
    pub sched_attr: i915_sched_attr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_engine_coredump {
    pub engine: *const intel_engine_cs,
    pub hung: bool,
    pub simulated: bool,
    pub reset_count: u32,
// position of active request inside the ring
    pub rq_tail: u32 rq_head, rq_post,,
// Register state
    pub ccid: u32,
    pub start: u32,
    pub tail: u32,
    pub head: u32,
    pub ctl: u32,
    pub mode: u32,
    pub hws: u32,
    pub ipeir: u32,
    pub ipehr: u32,
    pub esr: u32,
    pub bbstate: u32,
    pub instpm: u32,
    pub instps: u32,
    pub bbaddr: u64,
    pub acthd: u64,
    pub fault_reg: u32,
    pub faddr: u64,
    pub /: *mut *mut u32 rc_psmi; / sleep state,
    pub nopid: u32,
    pub excc: u32,
    pub cmd_cctl: u32,
    pub cscmdop: u32,
    pub ctx_sr_ctl: u32,
    pub dma_faddr_hi: u32,
    pub dma_faddr_lo: u32,
    pub instdone: intel_instdone,
// GuC matched capture-lists info
    pub guc_capture: *mut intel_guc_state_capture,
    pub guc_capture_node: *mut __guc_capture_parsed_output,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i915_gem_context_coredump {
    pub comm: [c_char; TASK_COMM_LEN],
    pub total_runtime: u64,
    pub avg_runtime: u64,
    pub pid: pid_t,
    pub active: c_int,
    pub guilty: c_int,
    pub sched_attr: i915_sched_attr,
    pub hwsp_seqno: u32,
    pub context: },
    pub vma: *mut i915_vma_coredump,
    pub execlist: [i915_request_coredump; EXECLIST_MAX_PORTS],
    pub num_ports: c_uint,
    pub gfx_mode: u32,
    pub pdp: [u64; 4],
    pub pp_dir_base: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_ctb_coredump {
    pub head: u32 raw_head,,
    pub tail: u32 raw_tail,,
    pub raw_status: u32,
    pub desc_offset: u32,
    pub cmds_offset: u32,
    pub size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_gt_coredump {
    pub _gt: *const intel_gt,
    pub awake: bool,
    pub simulated: bool,
    pub info: intel_gt_info,
// Generic register state
    pub eir: u32,
    pub pgtbl_er: u32,
    pub ngtier: u32 gtier[6],,
    pub forcewake: u32,
    pub /: *mut *mut u32 error; / gen6+,
    pub /: *mut *mut u32 fault_data0; / gen8, gen9,
    pub /: *mut *mut u32 fault_data1; / gen8, gen9,
    pub done_reg: u32,
    pub gac_eco: u32,
    pub gam_ecochk: u32,
    pub gab_ctl: u32,
    pub gfx_mode: u32,
    pub gtt_cache: u32,
    pub /: *mut *mut u32 aux_err; / gen12,
    pub /: *mut *mut u32 gam_done; / gen12,
    pub clock_frequency: u32,
    pub clock_period_ns: u32,
    pub /: *mut *mut u32 sfc_done[I915_MAX_SFC]; / gen12,
    pub nfence: u32,
    pub fence: [u64; I915_MAX_NUM_FENCES],
    pub engine: *mut intel_engine_coredump,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_uc_coredump {
    pub guc_fw: intel_uc_fw,
    pub huc_fw: intel_uc_fw,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct guc_info {
    pub ctb: [intel_ctb_coredump; 2],
    pub vma_ctb: *mut i915_vma_coredump,
    pub vma_log: *mut i915_vma_coredump,
    pub hw_state: *mut u32,
    pub timestamp: u32,
    pub last_fence: u16,
    pub is_guc_capture: bool,
    pub guc: },
    pub uc: *mut },
    pub next: *mut intel_gt_coredump,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i915_gpu_coredump {
    pub ref: kref,
    pub time: ktime_t,
    pub boottime: ktime_t,
    pub uptime: ktime_t,
    pub capture: c_ulong,
    pub i915: *mut drm_i915_private,
    pub gt: *mut intel_gt_coredump,
    pub error_msg: [c_char; 128],
    pub simulated: bool,
    pub wakelock: bool,
    pub suspended: bool,
    pub iommu: c_int,
    pub reset_count: u32,
    pub suspend_count: u32,
    pub device_info: intel_device_info,
    pub runtime_info: intel_runtime_info,
    pub driver_caps: intel_driver_caps,
    pub params: i915_params,
    pub fit: *mut *mut scatterlist sgl,,
    pub display_snapshot: *mut intel_display_snapshot,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i915_gpu_error {
// For reset and error_state handling.
    pub lock: spinlock_t,
// Protected by the above dev->gpu_error.lock.
    pub first_error: *mut i915_gpu_coredump,
// Number of times the device has been reset (global)
    pub reset_count: core::sync::atomic::AtomicI32,
// Number of times an engine has been reset
    pub reset_engine_count: [core::sync::atomic::AtomicI32; MAX_ENGINE_CLASS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_i915_error_state_buf {
    pub i915: *mut drm_i915_private,
    pub end: *mut *mut *mut scatterlist sgl, cur,,
    pub buf: *mut c_char,
    pub bytes: usize,
    pub size: usize,
    pub iter: loff_t,
    pub err: c_int,
}

extern "C" {
    pub fn atomic_read(_arg: &error->reset_count) -> return;
}
extern "C" {
    pub fn atomic_read(_arg: &error->reset_engine_count[engine->class]) -> return;
}
pub const CORE_DUMP_FLAG_NONE: c_uint = 0x0;

extern "C" {
    pub fn i915_error_printf(e: *mut drm_i915_error_state_buf, f: *const c_char, ...);
}
extern "C" {
    pub fn i915_error_state_store(error: *mut i915_gpu_coredump);
}
extern "C" {
    pub fn __i915_gpu_coredump_free(kref: *mut kref);
}
extern "C" {
    pub fn i915_reset_error_state(i915: *mut drm_i915_private);
}
extern "C" {
    pub fn i915_disable_error_state(i915: *mut drm_i915_private, err: c_int);
}
extern "C" {
    pub fn i915_gpu_error_debugfs_register(i915: *mut drm_i915_private);
}
extern "C" {
    pub fn i915_gpu_error_sysfs_setup(i915: *mut drm_i915_private);
}
extern "C" {
    pub fn i915_gpu_error_sysfs_teardown(i915: *mut drm_i915_private);
}

