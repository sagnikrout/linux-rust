//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/crush/mapper.h
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


// SPDX-License-Identifier: GPL-2.0
//
// CRUSH functions for find rules and then mapping an input to an
// output set.
//
// LGPL2
//

extern "C" {
    pub fn crush_find_rule(map: *const crush_map, ruleset: c_int, type: c_int, size: c_int) -> c_int;
}
//
// Returns the exact amount of workspace that will need to be used
// for a given combination of crush_map and result_max. The caller can
// then allocate this much on its own, either on the stack, in a
// per-thread long-lived buffer, or however it likes.
//
extern "C" {
    pub fn crush_init_workspace(map: *const crush_map, v: *mut c_void);
}
