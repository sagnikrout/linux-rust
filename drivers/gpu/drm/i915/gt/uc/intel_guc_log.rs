//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/gt/uc/intel_guc_log.h
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
// Copyright © 2014-2019 Intel Corporation
//

//
// While we're using plain log level in i915, GuC controls are much more...
// "elaborate"? We have a couple of bits for verbosity, separate bit for actual
// log enabling, and separate bit for default logging - which "conveniently"
// ignores the enable bit.
//
pub const GUC_LOG_LEVEL_DISABLED: c_int = 0;
pub const GUC_LOG_LEVEL_NON_VERBOSE: c_int = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_guc_log {
    pub level: u32,
//
// Protects concurrent access and modification of intel_guc_log->level.
//
// This lock replaces the legacy struct_mutex usage in
// intel_guc_log system.
//
    pub guc_lock: mutex,
// Allocation settings
    pub /: *mut *mut s32 bytes; / Size in bytes,
    pub /: *mut *mut s32 units; / GuC API units - 1MB or 4KB,
    pub /: *mut *mut s32 count; / Number of API units,
    pub /: *mut *mut u32 flag; / GuC API units flag,
    pub sizes: [}; GUC_LOG_SECTIONS_LIMIT],
    pub sizes_initialised: bool,
// Combined buffer allocation
    pub vma: *mut i915_vma,
    pub buf_addr: *mut c_void,
// RelayFS support
    pub buf_in_use: bool,
    pub started: bool,
    pub flush_work: work_struct,
    pub channel: *mut rchan,
    pub lock: mutex,
    pub full_count: u32,
    pub relay: },
// logging related stats
    pub sampled_overflow: u32,
    pub overflow: u32,
    pub flush: u32,
    pub stats: [}; GUC_MAX_LOG_BUFFER],
}

extern "C" {
    pub fn intel_guc_log_init_early(log: *mut intel_guc_log);
}
extern "C" {
    pub fn intel_guc_get_log_buffer_offset(log: *mut intel_guc_log, type: guc_log_buffer_type) -> usize;
}
extern "C" {
    pub fn intel_guc_log_create(log: *mut intel_guc_log) -> c_int;
}
extern "C" {
    pub fn intel_guc_log_destroy(log: *mut intel_guc_log);
}
extern "C" {
    pub fn intel_guc_log_set_level(log: *mut intel_guc_log, level: u32) -> c_int;
}
extern "C" {
    pub fn intel_guc_log_relay_created(log: *const intel_guc_log) -> bool;
}
extern "C" {
    pub fn intel_guc_log_relay_open(log: *mut intel_guc_log) -> c_int;
}
extern "C" {
    pub fn intel_guc_log_relay_start(log: *mut intel_guc_log) -> c_int;
}
extern "C" {
    pub fn intel_guc_log_relay_flush(log: *mut intel_guc_log);
}
extern "C" {
    pub fn intel_guc_log_relay_close(log: *mut intel_guc_log);
}
extern "C" {
    pub fn intel_guc_log_handle_flush_event(log: *mut intel_guc_log);
}
extern "C" {
    pub fn intel_guc_log_info(log: *mut intel_guc_log, p: *mut drm_printer);
}
extern "C" {
    pub fn intel_guc_log_section_size_capture(log: *mut intel_guc_log) -> u32;
}
