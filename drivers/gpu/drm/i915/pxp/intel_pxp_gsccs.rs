//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/pxp/intel_pxp_gsccs.h
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
// Copyright(c) 2022, Intel Corporation. All rights reserved.
//

pub const GSC_PENDING_RETRY_MAXCOUNT: c_int = 40;
pub const GSC_PENDING_RETRY_PAUSE_MS: c_int = 50;

extern "C" {
    pub fn intel_pxp_gsccs_fini(pxp: *mut intel_pxp);
}
extern "C" {
    pub fn intel_pxp_gsccs_init(pxp: *mut intel_pxp) -> c_int;
}
extern "C" {
    pub fn intel_pxp_gsccs_create_session(pxp: *mut intel_pxp, arb_session_id: c_int) -> c_int;
}
extern "C" {
    pub fn intel_pxp_gsccs_end_arb_fw_session(pxp: *mut intel_pxp, arb_session_id: u32);
}
extern "C" {
    pub fn intel_pxp_gsccs_is_ready_for_sessions(pxp: *mut intel_pxp) -> bool;
}

