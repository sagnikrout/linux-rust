//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_guc_ct.h
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
    pub fn xe_guc_ct_init_noalloc(ct: *mut xe_guc_ct) -> c_int;
}
extern "C" {
    pub fn xe_guc_ct_init(ct: *mut xe_guc_ct) -> c_int;
}
extern "C" {
    pub fn xe_guc_ct_init_post_hwconfig(ct: *mut xe_guc_ct) -> c_int;
}
extern "C" {
    pub fn xe_guc_ct_enable(ct: *mut xe_guc_ct) -> c_int;
}
extern "C" {
    pub fn xe_guc_ct_restart(ct: *mut xe_guc_ct) -> c_int;
}
extern "C" {
    pub fn xe_guc_ct_disable(ct: *mut xe_guc_ct);
}
extern "C" {
    pub fn xe_guc_ct_runtime_resume(ct: *mut xe_guc_ct);
}
extern "C" {
    pub fn xe_guc_ct_runtime_suspend(ct: *mut xe_guc_ct);
}
extern "C" {
    pub fn xe_guc_ct_stop(ct: *mut xe_guc_ct);
}
extern "C" {
    pub fn xe_guc_ct_flush_and_stop(ct: *mut xe_guc_ct);
}
extern "C" {
    pub fn xe_guc_ct_fast_path(ct: *mut xe_guc_ct);
}
extern "C" {
    pub fn xe_guc_ct_snapshot_print(snapshot: *mut xe_guc_ct_snapshot, p: *mut drm_printer);
}
extern "C" {
    pub fn xe_guc_ct_snapshot_free(snapshot: *mut xe_guc_ct_snapshot);
}
extern "C" {
    pub fn xe_guc_ct_print(ct: *mut xe_guc_ct, p: *mut drm_printer, want_ctb: bool);
}
// READ_ONCE pairs with WRITE_ONCE in guc_ct_change_state.
// Basic CT send / receives
extern "C" {
    pub fn xe_guc_ct_send_recv(_arg: ct, _arg: action, _arg: len, _arg: NULL) -> return;
}
// This is only version of the send CT you can call from a G2H handler
// Can't fail because a GT reset is in progress
extern "C" {
    pub fn xe_guc_ct_send_recv_no_fail(_arg: ct, _arg: action, _arg: len, _arg: NULL) -> return;
}
extern "C" {
    pub fn xe_guc_ct_queue_proc_time_jiffies(ct: *mut xe_guc_ct) -> c_long;
}
//
// xe_guc_ct_wake_waiters() - GuC CT wake up waiters
// @ct: GuC CT object
//
