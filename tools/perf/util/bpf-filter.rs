//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/util/bpf-filter.h
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
pub struct perf_bpf_filter_expr {
    pub list: list_head,
    pub groups: list_head,
    pub op: perf_bpf_filter_op,
    pub part: c_int,
    pub term: perf_bpf_filter_term,
    pub val: c_ulong,
}

// path in BPF-fs for the pinned program and maps

extern "C" {
    pub fn perf_bpf_filter__parse(expr_head: *mut list_head, str: *const c_char) -> c_int;
}
extern "C" {
    pub fn perf_bpf_filter__prepare(evsel: *mut evsel, target: *mut target) -> c_int;
}
extern "C" {
    pub fn perf_bpf_filter__destroy(evsel: *mut evsel) -> c_int;
}
extern "C" {
    pub fn perf_bpf_filter__lost_count(evsel: *mut evsel) -> u64;
}
extern "C" {
    pub fn perf_bpf_filter__pin() -> c_int;
}
extern "C" {
    pub fn perf_bpf_filter__unpin() -> c_int;
}

