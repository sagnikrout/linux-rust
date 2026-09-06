//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_pxp_submit.h
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
// Copyright(c) 2024, Intel Corporation. All rights reserved.
//

extern "C" {
    pub fn xe_pxp_allocate_execution_resources(pxp: *mut xe_pxp) -> c_int;
}
extern "C" {
    pub fn xe_pxp_destroy_execution_resources(pxp: *mut xe_pxp);
}
extern "C" {
    pub fn xe_pxp_submit_session_init(gsc_res: *mut xe_pxp_gsc_client_resources, id: u32) -> c_int;
}
extern "C" {
    pub fn xe_pxp_submit_session_termination(pxp: *mut xe_pxp, id: u32) -> c_int;
}
