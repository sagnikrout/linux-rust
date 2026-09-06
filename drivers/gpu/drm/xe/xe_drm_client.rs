//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_drm_client.h
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
// Copyright © 2023 Intel Corporation
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe_drm_client {
    pub kref: kref,
    pub id: c_uint,

//
// @bos_lock: lock protecting @bos_list
//
    pub bos_lock: spinlock_t,
//
// @bos_list: list of bos created by this client
//
// Protected by @bos_lock.
//
    pub bos_list: list_head,

}

extern "C" {
    pub fn __xe_drm_client_free(kref: *mut kref);
}
extern "C" {
    pub fn xe_drm_client_put(client: *mut xe_drm_client);
}

extern "C" {
    pub fn xe_drm_client_fdinfo(p: *mut drm_printer, file: *mut drm_file);
}
extern "C" {
    pub fn xe_drm_client_remove_bo(bo: *mut xe_bo);
}

