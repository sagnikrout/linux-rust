//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/util/pmus.h
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

extern "C" {
    pub fn pmu_name_len_no_suffix(str: *const c_char) -> usize;
}
// Exposed for testing only.
extern "C" {
    pub fn pmu_name_cmp(lhs_pmu_name: *const c_char, rhs_pmu_name: *const c_char) -> c_int;
}
extern "C" {
    pub fn perf_pmus__destroy();
}
extern "C" {
    pub fn perf_pmus__print_pmu_events(print_cb: *const print_callbacks, print_state: *mut c_void);
}
extern "C" {
    pub fn perf_pmus__print_raw_pmu_events(print_cb: *const print_callbacks, print_state: *mut c_void);
}
extern "C" {
    pub fn perf_pmus__have_event(pname: *const c_char, name: *const c_char) -> bool;
}
extern "C" {
    pub fn perf_pmus__num_core_pmus() -> c_int;
}
extern "C" {
    pub fn perf_pmus__supports_extended_type() -> bool;
}
