//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/util/block-info.h
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
pub struct block_info {
    pub sym: *mut symbol,
    pub start: u64,
    pub end: u64,
    pub cycles: u64,
    pub cycles_aggr: u64,
    pub cycles_spark: [i64; NUM_SPARKS],
    pub total_cycles: u64,
    pub num: c_int,
    pub num_aggr: c_int,
    pub br_cntr_nr: c_int,
    pub br_cntr: *mut u64,
    pub evsel: *mut evsel,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct block_fmt {
    pub fmt: perf_hpp_fmt,
    pub idx: c_int,
    pub width: c_int,
    pub header: *const c_char,
    pub total_cycles: u64,
    pub block_cycles: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct block_report {
    pub hist: block_hist,
    pub cycles: u64,
    pub fmts: [block_fmt; PERF_HPP_REPORT__BLOCK_MAX_INDEX],
    pub nr_fmts: c_int,
}

extern "C" {
    pub fn block_info__delete(bi: *mut block_info);
}
extern "C" {
    pub fn __block_info__cmp(left: *mut hist_entry, right: *mut hist_entry) -> i64;
}
extern "C" {
    pub fn block_info__free_report(reps: *mut block_report, nr_reps: c_int);
}
extern "C" {
    pub fn block_info__total_cycles_percent(he: *mut hist_entry) -> float;
}
