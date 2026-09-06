//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/display/intel_global_state.h
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
// Copyright © 2020 Intel Corporation
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_global_state_funcs {
    pub obj): *mut *mut *mut intel_global_state (atomic_duplicate_state)(intel_global_obj,
    pub state): *mut intel_global_state,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_global_obj {
    pub head: list_head,
    pub state: *mut intel_global_state,
    pub funcs: *const intel_global_state_funcs,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_global_state {
    pub obj: *mut intel_global_obj,
    pub state: *mut intel_atomic_state,
    pub commit: *mut intel_global_commit,
    pub ref: kref,
    pub serialized: bool changed,,
}

extern "C" {
    pub fn intel_atomic_global_obj_cleanup(display: *mut intel_display);
}
extern "C" {
    pub fn intel_atomic_swap_global_state(state: *mut intel_atomic_state);
}
extern "C" {
    pub fn intel_atomic_clear_global_state(state: *mut intel_atomic_state);
}
extern "C" {
    pub fn intel_atomic_lock_global_state(obj_state: *mut intel_global_state) -> c_int;
}
extern "C" {
    pub fn intel_atomic_serialize_global_state(obj_state: *mut intel_global_state) -> c_int;
}
extern "C" {
    pub fn intel_atomic_global_state_setup_commit(state: *mut intel_atomic_state) -> c_int;
}
extern "C" {
    pub fn intel_atomic_global_state_commit_done(state: *mut intel_atomic_state);
}
extern "C" {
    pub fn intel_atomic_global_state_wait_for_dependencies(state: *mut intel_atomic_state) -> c_int;
}
extern "C" {
    pub fn intel_atomic_global_state_is_serialized(state: *mut intel_atomic_state) -> bool;
}
