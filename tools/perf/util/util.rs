//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/util/util.h
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
pub const _BSD_SOURCE: c_int = 1;
// glibc 2.20 deprecates _BSD_SOURCE in favour of _DEFAULT_SOURCE
pub const _DEFAULT_SOURCE: c_int = 1;

// This will control if perf_{host,guest} will set attr.exclude_{host,guest}.
// General helper functions
extern "C" {
    pub fn mkdir_p(path: *mut c_char, mode: mode_t) -> c_int;
}
extern "C" {
    pub fn rm_rf(path: *const c_char) -> c_int;
}
extern "C" {
    pub fn rm_rf_perf_data(path: *const c_char) -> c_int;
}
extern "C" {
    pub fn lsdir_no_dot_filter(name: *const c_char, d: *mut dirent) -> bool;
}
extern "C" {
    pub fn hex_width(v: u64) -> usize;
}
extern "C" {
    pub fn sysctl__max_stack() -> c_int;
}
extern "C" {
    pub fn sysctl__nmi_watchdog_enabled() -> bool;
}
extern "C" {
    pub fn perf_tip(strp: *mut c_char, dirpath: *const c_char) -> c_int;
}
extern "C" {
    pub fn cpumask_to_cpulist(cpumask: *mut c_char, cpulist: *mut c_char);
}
extern "C" {
    pub fn print_separator2(pre_dash_cnt: c_int, s: *const c_char, post_dash_cnt: c_int);
}

extern "C" {
    pub fn sched_getcpu() -> c_int;
}

extern "C" {
    pub fn perf_set_singlethreaded();
}
extern "C" {
    pub fn perf_set_multithreaded();
}

pub const O_CLOEXEC: c_uint = 0x400000;

pub const O_CLOEXEC: c_int = 010000000;

pub const O_CLOEXEC: c_int = 02000000;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_debuginfod {
    pub urls: *const c_char,
    pub set: bool,
}

extern "C" {
    pub fn perf_debuginfod_setup(di: *mut perf_debuginfod);
}

