//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_huc.h
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

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xe_huc_auth_types {
    XE_HUC_AUTH_VIA_GUC = 0,
    XE_HUC_AUTH_VIA_GSC,
    XE_HUC_AUTH_TYPES_COUNT
}

extern "C" {
    pub fn xe_huc_init(huc: *mut xe_huc) -> c_int;
}
extern "C" {
    pub fn xe_huc_init_post_hwconfig(huc: *mut xe_huc) -> c_int;
}
extern "C" {
    pub fn xe_huc_upload(huc: *mut xe_huc) -> c_int;
}
extern "C" {
    pub fn xe_huc_auth(huc: *mut xe_huc, type: xe_huc_auth_types) -> c_int;
}
extern "C" {
    pub fn xe_huc_is_authenticated(huc: *mut xe_huc, type: xe_huc_auth_types) -> bool;
}
extern "C" {
    pub fn xe_huc_sanitize(huc: *mut xe_huc);
}
extern "C" {
    pub fn xe_huc_print_info(huc: *mut xe_huc, p: *mut drm_printer);
}
