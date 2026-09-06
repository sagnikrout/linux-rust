//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/util/time-utils.h
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
pub struct perf_time_interval {
    pub end: u64 start,,
}

extern "C" {
    pub fn parse_nsec_time(str: *const c_char, ptime: *mut u64) -> c_int;
}
extern "C" {
    pub fn perf_time__parse_str(ptime: *mut perf_time_interval, ostr: *const c_char) -> c_int;
}
extern "C" {
    pub fn perf_time__skip_sample(ptime: *mut perf_time_interval, timestamp: u64) -> bool;
}
extern "C" {
    pub fn timestamp__scnprintf_usec(timestamp: u64, buf: *mut c_char, sz: usize) -> c_int;
}
extern "C" {
    pub fn timestamp__scnprintf_nsec(timestamp: u64, buf: *mut c_char, sz: usize) -> c_int;
}
extern "C" {
    pub fn fetch_current_timestamp(buf: *mut c_char, sz: usize) -> c_int;
}
