//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/util/intel-tpebs.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// intel_tpebs.h: Intel TEPBS support
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tpebs_mode {
    TPEBS_MODE__MEAN,
    TPEBS_MODE__MIN,
    TPEBS_MODE__MAX,
    TPEBS_MODE__LAST,
}

extern "C" {
    pub fn evsel__tpebs_open(evsel: *mut evsel) -> c_int;
}
extern "C" {
    pub fn evsel__tpebs_close(evsel: *mut evsel);
}
extern "C" {
    pub fn evsel__tpebs_read(evsel: *mut evsel, cpu_map_idx: c_int, thread: c_int) -> c_int;
}
