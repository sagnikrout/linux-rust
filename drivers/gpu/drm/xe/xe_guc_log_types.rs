//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_guc_log_types.h
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
// struct xe_guc_log_snapshot:
// Capture of the GuC log plus various state useful for decoding the log
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_guc_log_snapshot {
// @size: Size in bytes of the @copy allocation
    pub size: usize,
// @copy: Host memory copy of the log buffer for later dumping, split into chunks
    pub copy: *mut c_void,
// @num_chunks: Number of chunks within @copy
    pub num_chunks: c_int,
// @ktime: Kernel time the snapshot was taken
    pub ktime: u64,
// @stamp: GuC timestamp at which the snapshot was taken
    pub stamp: u64,
// @level: GuC log verbosity level
    pub level: u32,
// @ver_found: GuC firmware version
    pub ver_found: xe_uc_fw_version,
// @ver_want: GuC firmware version that driver expected
    pub ver_want: xe_uc_fw_version,
// @path: Path of GuC firmware blob
    pub path: *const c_char,
}

//
// struct xe_guc_log - GuC log
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_guc_log {
// @level: GuC log level
    pub level: u32,
// @bo: Xe BO for GuC log
    pub bo: *mut xe_bo,
// @stats: logging related stats
    pub sampled_overflow: u32,
    pub overflow: u32,
    pub flush: u32,
    pub stats: [}; GUC_LOG_BUFFER_TYPE_MAX],
}
