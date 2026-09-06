//! Automatically rewritten from C Header to Rust Module
//! Source: tools/lib/perf/include/perf/evlist.h
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
    pub fn perf_evlist__delete(evlist: *mut perf_evlist) -> LIBPERF_API void;
}
extern "C" {
    pub fn perf_evlist__open(evlist: *mut perf_evlist) -> LIBPERF_API int;
}
extern "C" {
    pub fn perf_evlist__close(evlist: *mut perf_evlist) -> LIBPERF_API void;
}
extern "C" {
    pub fn perf_evlist__enable(evlist: *mut perf_evlist) -> LIBPERF_API void;
}
extern "C" {
    pub fn perf_evlist__disable(evlist: *mut perf_evlist) -> LIBPERF_API void;
}

extern "C" {
    pub fn perf_evlist__poll(evlist: *mut perf_evlist, timeout: c_int) -> LIBPERF_API int;
}
extern "C" {
    pub fn perf_evlist__mmap(evlist: *mut perf_evlist, pages: c_int) -> LIBPERF_API int;
}
extern "C" {
    pub fn perf_evlist__munmap(evlist: *mut perf_evlist) -> LIBPERF_API void;
}

extern "C" {
    pub fn perf_evlist__set_leader(evlist: *mut perf_evlist) -> LIBPERF_API void;
}
extern "C" {
    pub fn perf_evlist__nr_groups(evlist: *mut perf_evlist) -> LIBPERF_API int;
}
