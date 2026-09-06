//! Automatically rewritten from C Header to Rust Module
//! Source: tools/lib/perf/include/perf/evsel.h
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
pub struct perf_counts_values {
    pub val: u64,
    pub ena: u64,
    pub run: u64,
    pub id: u64,
    pub lost: u64,
}

extern "C" {
    pub fn perf_evsel__delete(evsel: *mut perf_evsel) -> LIBPERF_API void;
}
extern "C" {
    pub fn perf_evsel__close(evsel: *mut perf_evsel) -> LIBPERF_API void;
}
extern "C" {
    pub fn perf_evsel__close_cpu(evsel: *mut perf_evsel, cpu_map_idx: c_int) -> LIBPERF_API void;
}
extern "C" {
    pub fn perf_evsel__mmap(evsel: *mut perf_evsel, pages: c_int) -> LIBPERF_API int;
}
extern "C" {
    pub fn perf_evsel__munmap(evsel: *mut perf_evsel) -> LIBPERF_API void;
}
extern "C" {
    pub fn perf_evsel__enable(evsel: *mut perf_evsel) -> LIBPERF_API int;
}
extern "C" {
    pub fn perf_evsel__enable_cpu(evsel: *mut perf_evsel, cpu_map_idx: c_int) -> LIBPERF_API int;
}
extern "C" {
    pub fn perf_evsel__enable_thread(evsel: *mut perf_evsel, thread: c_int) -> LIBPERF_API int;
}
extern "C" {
    pub fn perf_evsel__disable(evsel: *mut perf_evsel) -> LIBPERF_API int;
}
extern "C" {
    pub fn perf_evsel__disable_cpu(evsel: *mut perf_evsel, cpu_map_idx: c_int) -> LIBPERF_API int;
}
