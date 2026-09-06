//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_guc_log.h
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

//
// While we're using plain log level in i915, GuC controls are much more...
// "elaborate"? We have a couple of bits for verbosity, separate bit for actual
// log enabling, and separate bit for default logging - which "conveniently"
// ignores the enable bit.
//
pub const GUC_LOG_LEVEL_DISABLED: c_int = 0;
pub const GUC_LOG_LEVEL_NON_VERBOSE: c_int = 1;

extern "C" {
    pub fn xe_guc_log_init(log: *mut xe_guc_log) -> c_int;
}
extern "C" {
    pub fn xe_guc_log_print(log: *mut xe_guc_log, p: *mut drm_printer);
}
extern "C" {
    pub fn xe_guc_log_print_lfd(log: *mut xe_guc_log, p: *mut drm_printer);
}
extern "C" {
    pub fn xe_guc_log_print_dmesg(log: *mut xe_guc_log);
}
extern "C" {
    pub fn xe_guc_log_snapshot_print(snapshot: *mut xe_guc_log_snapshot, p: *mut drm_printer);
}
extern "C" {
    pub fn xe_guc_log_snapshot_free(snapshot: *mut xe_guc_log_snapshot);
}
