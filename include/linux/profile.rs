//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/profile.h
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

pub const CPU_PROFILING: c_int = 1;
pub const SCHED_PROFILING: c_int = 2;
pub const KVM_PROFILING: c_int = 4;

extern "C" {
    pub fn create_proc_profile() -> c_int;
}

// init basic kernel profiler
extern "C" {
    pub fn profile_init() -> c_int;
}
extern "C" {
    pub fn profile_setup(str: *mut c_char) -> c_int;
}
extern "C" {
    pub fn profile_tick(type: c_int);
}
extern "C" {
    pub fn setup_profiling_timer(multiplier: c_uint) -> c_int;
}
//
// Add multiple profiler hits to a given address:
//
extern "C" {
    pub fn profile_hits(type: c_int, ip: *mut c_void, nr_hits: c_uint);
}
//
// Single profiler hit:
//
// Speedup for the common (no profiling enabled) case:
//

pub const prof_on: c_int = 0;

