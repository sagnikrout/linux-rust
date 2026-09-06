//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/util/unwind.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct unwind_entry {
    pub ms: map_symbol,
    pub ip: u64,
}

extern "C" {
    pub fn int(entry: *mut *mut unwind_entry_cb_t)(struct unwind_entry, arg: *mut c_void) -> typedef;
}
extern "C" {
    pub fn unwind__configure(var: *const c_char, value: *const c_char, cb: *mut c_void) -> c_int;
}
extern "C" {
    pub fn unwind__option(opt: *const option, arg: *const c_char, unset: c_int) -> c_int;
}
//
// When best_effort is set, don't report errors and fail silently. This could
// be expanded in the future to be more permissive about things other than
// error messages.
//

// libunwind specific
extern "C" {
    pub fn unwind__prepare_access(maps: *mut maps, e_machine: u16) -> c_int;
}
extern "C" {
    pub fn unwind__flush_access(maps: *mut maps);
}
extern "C" {
    pub fn unwind__finish_access(maps: *mut maps);
}

