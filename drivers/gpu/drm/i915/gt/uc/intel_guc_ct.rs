//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/gt/uc/intel_guc_ct.h
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
// Copyright © 2016-2019 Intel Corporation
//

//
// DOC: Command Transport (CT).
//
// Buffer based command transport is a replacement for MMIO based mechanism.
// It can be used to perform both host-2-guc and guc-to-host communication.
//
// Represents single command transport buffer.
//
// A single command transport buffer consists of two parts, the header
// record (command transport buffer descriptor) and the actual buffer which
// holds the commands.
//
// @lock: protects access to the commands buffer and buffer descriptor
// @desc: pointer to the buffer descriptor
// @cmds: pointer to the commands buffer
// @size: size of the commands buffer in dwords
// @resv_space: reserved space in buffer in dwords
// @head: local shadow copy of head in dwords
// @tail: local shadow copy of tail in dwords
// @space: local shadow copy of space in dwords
// @broken: flag to indicate if descriptor data is broken
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_guc_ct_buffer {
    pub lock: spinlock_t,
    pub desc: *mut guc_ct_buffer_desc,
    pub cmds: *mut u32,
    pub size: u32,
    pub resv_space: u32,
    pub tail: u32,
    pub head: u32,
    pub space: core::sync::atomic::AtomicI32,
    pub broken: bool,
}

// Top-level structure for Command Transport related data
//
// Includes a pair of CT buffers for bi-directional communication and tracking
// for the H2G and G2H requests sent and received through the buffers.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_guc_ct {
    pub vma: *mut i915_vma,
    pub enabled: bool,
// buffers for sending and receiving commands
    pub send: intel_guc_ct_buffer,
    pub recv: intel_guc_ct_buffer,
    pub ctbs: },
    pub receive_tasklet: tasklet_struct,
// @wq: wait queue for g2h chanenl
    pub wq: wait_queue_head_t,
    pub /: *mut *mut u16 last_fence; / last fence used to send request,
    pub /: *mut *mut spinlock_t lock; / protects pending requests list,
    pub /: *mut *mut list_head pending; / requests waiting for response,
    pub /: *mut *mut list_head incoming; / incoming requests,
    pub /: *mut *mut work_worker; / handler for incoming requests,

    pub fence: u16,
    pub action: u16,

    pub stack: depot_stack_handle_t,
    pub lost_and_found: [}; SZ_16],
    pub requests: },
// @stall_time: time of first time a CTB submission is stalled
    pub stall_time: ktime_t,

    pub dead_ct_reason: c_int,
    pub dead_ct_reported: bool,
    pub dead_ct_worker: work_struct,

}

extern "C" {
    pub fn intel_guc_ct_max_queue_time_jiffies() -> c_long;
}
extern "C" {
    pub fn intel_guc_ct_init_early(ct: *mut intel_guc_ct);
}
extern "C" {
    pub fn intel_guc_ct_init(ct: *mut intel_guc_ct) -> c_int;
}
extern "C" {
    pub fn intel_guc_ct_fini(ct: *mut intel_guc_ct);
}
extern "C" {
    pub fn intel_guc_ct_enable(ct: *mut intel_guc_ct) -> c_int;
}
extern "C" {
    pub fn intel_guc_ct_disable(ct: *mut intel_guc_ct);
}

pub const INTEL_GUC_CT_SEND_G2H_DW_SHIFT: c_int = 0;

extern "C" {
    pub fn intel_guc_ct_event_handler(ct: *mut intel_guc_ct);
}
extern "C" {
    pub fn intel_guc_ct_print_info(ct: *mut intel_guc_ct, p: *mut drm_printer);
}
