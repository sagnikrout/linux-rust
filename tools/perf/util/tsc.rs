//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/util/tsc.h
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
pub struct perf_tsc_conversion {
    pub time_shift: u16,
    pub time_mult: u32,
    pub time_zero: u64,
    pub time_cycles: u64,
    pub time_mask: u64,
    pub cap_user_time_zero: bool,
    pub cap_user_time_short: bool,
}

extern "C" {
    pub fn perf_time_to_tsc(ns: u64, tc: *mut perf_tsc_conversion) -> u64;
}
extern "C" {
    pub fn tsc_to_perf_time(cyc: u64, tc: *mut perf_tsc_conversion) -> u64;
}
extern "C" {
    pub fn rdtsc() -> u64;
}
extern "C" {
    pub fn arch_get_tsc_freq() -> u64;
}
extern "C" {
    pub fn perf_event__fprintf_time_conv(event: *mut perf_event, fp: *mut FILE) -> usize;
}
