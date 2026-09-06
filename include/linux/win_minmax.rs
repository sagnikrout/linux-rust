//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/win_minmax.h
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
// win_minmax.h: windowed min/max tracker by Kathleen Nichols.
//

// A single data point for our parameterized min-max tracker
#[repr(C)]
#[derive(Copy, Clone)]
pub struct minmax_sample {
    pub /: *mut *mut u32 t; / time measurement was taken,
    pub /: *mut *mut u32 v; / value measured,
}

// State for the parameterized min-max tracker
#[repr(C)]
#[derive(Copy, Clone)]
pub struct minmax {
    pub s: [minmax_sample; 3],
}

extern "C" {
    pub fn minmax_running_max(m: *mut minmax, win: u32, t: u32, meas: u32) -> u32;
}
extern "C" {
    pub fn minmax_running_min(m: *mut minmax, win: u32, t: u32, meas: u32) -> u32;
}
