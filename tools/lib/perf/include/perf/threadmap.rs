//! Automatically rewritten from C Header to Rust Module
//! Source: tools/lib/perf/include/perf/threadmap.h
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
    pub fn perf_thread_map__set_pid(map: *mut perf_thread_map, idx: c_int, pid: pid_t) -> LIBPERF_API void;
}
extern "C" {
    pub fn perf_thread_map__nr(threads: *mut perf_thread_map) -> LIBPERF_API int;
}
extern "C" {
    pub fn perf_thread_map__pid(map: *mut perf_thread_map, idx: c_int) -> LIBPERF_API pid_t;
}
extern "C" {
    pub fn perf_thread_map__idx(map: *mut perf_thread_map, pid: pid_t) -> LIBPERF_API int;
}
extern "C" {
    pub fn perf_thread_map__put(map: *mut perf_thread_map) -> LIBPERF_API void;
}
