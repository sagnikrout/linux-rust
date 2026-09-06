//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/util/iostat.h
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
//
// perf iostat
//
// Copyright (C) 2020, Intel Corporation
//
// Authors: Alexander Antonov <alexander.antonov@linux.intel.com>
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iostat_mode_t {
    IOSTAT_NONE		= -1,
    IOSTAT_RUN		= 0,
    IOSTAT_LIST		= 1
}

extern "C" {
    pub fn void(: *mut *mut iostat_print_counter_t)(struct perf_stat_config, : *mut evsel, : *mut c_void) -> typedef;
}
extern "C" {
    pub fn iostat_prepare(evlist: *mut evlist, config: *mut perf_stat_config) -> c_int;
}
extern "C" {
    pub fn iostat_list(evlist: *mut evlist, config: *mut perf_stat_config);
}
extern "C" {
    pub fn iostat_release(evlist: *mut evlist);
}
extern "C" {
    pub fn iostat_print_header_prefix(config: *mut perf_stat_config);
}
