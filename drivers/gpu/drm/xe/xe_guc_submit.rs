//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_guc_submit.h
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
// Copyright © 2022 Intel Corporation
//

extern "C" {
    pub fn xe_guc_submit_init(guc: *mut xe_guc, num_ids: c_uint) -> c_int;
}
extern "C" {
    pub fn xe_guc_submit_enable(guc: *mut xe_guc) -> c_int;
}
extern "C" {
    pub fn xe_guc_submit_disable(guc: *mut xe_guc);
}
extern "C" {
    pub fn xe_guc_submit_reset_prepare(guc: *mut xe_guc) -> c_int;
}
extern "C" {
    pub fn xe_guc_submit_reset_wait(guc: *mut xe_guc);
}
extern "C" {
    pub fn xe_guc_submit_stop(guc: *mut xe_guc);
}
extern "C" {
    pub fn xe_guc_submit_start(guc: *mut xe_guc) -> c_int;
}
extern "C" {
    pub fn xe_guc_submit_pause(guc: *mut xe_guc);
}
extern "C" {
    pub fn xe_guc_submit_pause_abort(guc: *mut xe_guc);
}
extern "C" {
    pub fn xe_guc_submit_pause_vf(guc: *mut xe_guc);
}
extern "C" {
    pub fn xe_guc_submit_unpause(guc: *mut xe_guc);
}
extern "C" {
    pub fn xe_guc_submit_unpause_vf(guc: *mut xe_guc);
}
extern "C" {
    pub fn xe_guc_submit_unpause_prepare_vf(guc: *mut xe_guc);
}
extern "C" {
    pub fn xe_guc_submit_wedge(guc: *mut xe_guc);
}
extern "C" {
    pub fn xe_guc_read_stopped(guc: *mut xe_guc) -> c_int;
}
extern "C" {
    pub fn xe_guc_sched_done_handler(guc: *mut xe_guc, msg: *mut u32, len: u32) -> c_int;
}
extern "C" {
    pub fn xe_guc_deregister_done_handler(guc: *mut xe_guc, msg: *mut u32, len: u32) -> c_int;
}
extern "C" {
    pub fn xe_guc_exec_queue_reset_handler(guc: *mut xe_guc, msg: *mut u32, len: u32) -> c_int;
}
extern "C" {
    pub fn xe_guc_uncorrectable_error_handler(guc: *mut xe_guc, msg: *mut u32, len: u32) -> c_int;
}
extern "C" {
    pub fn xe_guc_exec_queue_reset_failure_handler(guc: *mut xe_guc, msg: *mut u32, len: u32) -> c_int;
}
extern "C" {
    pub fn xe_guc_error_capture_handler(guc: *mut xe_guc, msg: *mut u32, len: u32) -> c_int;
}
extern "C" {
    pub fn xe_guc_exec_queue_cgp_sync_done_handler(guc: *mut xe_guc, msg: *mut u32, len: u32) -> c_int;
}
extern "C" {
    pub fn xe_guc_submit_print(guc: *mut xe_guc, p: *mut drm_printer);
}
extern "C" {
    pub fn xe_guc_register_vf_exec_queue(q: *mut xe_exec_queue, ctx_type: c_int);
}
extern "C" {
    pub fn xe_guc_has_registered_mlrc_queues(guc: *mut xe_guc) -> bool;
}
extern "C" {
    pub fn xe_guc_contexts_hwsp_rebase(guc: *mut xe_guc, scratch: *mut c_void) -> c_int;
}
