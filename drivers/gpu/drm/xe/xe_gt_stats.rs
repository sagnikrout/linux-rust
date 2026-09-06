//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xe/xe_gt_stats.h
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


// SPDX-License-Identifier: MIT
//
// Copyright © 2024 Intel Corporation
//

extern "C" {
    pub fn xe_gt_stats_init(gt: *mut xe_gt) -> c_int;
}
extern "C" {
    pub fn xe_gt_stats_print_info(gt: *mut xe_gt, p: *mut drm_printer) -> c_int;
}
extern "C" {
    pub fn xe_gt_stats_clear(gt: *mut xe_gt);
}
extern "C" {
    pub fn xe_gt_stats_incr(gt: *mut xe_gt, id: xe_gt_stats_id, incr: c_int);
}

//
// xe_gt_stats_ktime_us_delta() - Get delta in microseconds between now and a
// start time
// @start: Start time
//
// Helper for GT stats to get delta in microseconds between now and a start
// time, compiles out if GT stats are disabled.
//
// Return: Delta in microseconds between now and a start time
//
// xe_gt_stats_ktime_get() - Get current ktime
//
// Helper for GT stats to get current ktime, compiles out if GT stats are
// disabled.
//
// Return: Get current ktime
//
