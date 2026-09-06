//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/util/c2c.h
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
pub struct c2c_hists {
    pub hists: hists,
    pub list: perf_hpp_list,
    pub stats: c2c_stats,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compute_stats {
    pub lcl_hitm: stats,
    pub rmt_hitm: stats,
    pub lcl_peer: stats,
    pub rmt_peer: stats,
    pub load: stats,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct c2c_hist_entry {
    pub hists: *mut c2c_hists,
    pub evsel: *mut evsel,
    pub stats: c2c_stats,
    pub cpuset: *mut c_ulong,
    pub nodeset: *mut c_ulong,
    pub node_stats: *mut c2c_stats,
    pub cacheline_idx: c_uint,
    pub cstats: compute_stats,
    pub paddr: c_ulong,
    pub paddr_cnt: c_ulong,
    pub paddr_zero: bool,
    pub nodestr: *mut c_char,
//
// must be at the end,
// because of its callchain dynamic entry
//
    pub he: hist_entry,
}

pub const C2C_HEADER_MAX: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct c2c_header {
    pub text: *const c_char,
    pub span: c_int,
    pub line: [}; C2C_HEADER_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct c2c_dimension {
    pub header: c2c_header,
    pub name: *const c_char,
    pub width: c_int,
    pub se: *mut sort_entry,
    pub right): *mut *mut hist_entry left, hist_entry,
    pub he): *mut hist_entry,
    pub he): *mut hist_entry,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct c2c_fmt {
    pub fmt: perf_hpp_fmt,
    pub dim: *mut c2c_dimension,
}

pub const SYMBOL_WIDTH: c_int = 30;

extern "C" {
    pub fn c2c_fmt_free(fmt: *mut perf_hpp_fmt);
}
extern "C" {
    pub fn c2c_fmt_equal(a: *mut perf_hpp_fmt, b: *mut perf_hpp_fmt) -> bool;
}
//
// Build the function-view hierarchy. Returns -EOPNOTSUPP when @cl_sort lacks
// iaddr. On success, *@hists remains valid until the next
// c2c_function__build() or c2c_function__reset(). On failure, *@hists is
// NULL.
//
extern "C" {
    pub fn c2c_function__reset();
}
// Valid only between a successful build and c2c_function__reset().
// Inputs and TUI callback supplied by the c2c command.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct c2c_function_view_args {
// Source cacheline histograms used by the common model.
    pub cl_hists: *mut c2c_hists,
// --coalesce field list, used to require iaddr.
    pub cl_sort: *const c_char,
// Do not cap long symbol names.
    pub symbol_full: bool,
// Open the cacheline detail view for @he.
    pub he): *mut *mut int (browse_cacheline)(struct hist_entry,
}

extern "C" {
    pub fn perf_c2c__browse_function_view(args: *mut c2c_function_view_args) -> c_int;
}

