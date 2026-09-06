//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/platform_data/tsc2007.h
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
// linux/platform_data/tsc2007.h
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tsc2007_platform_data {
    pub /: *mut *mut u16 model; / 2007.,
    pub /: *mut *mut u16 x_plate_ohms; / must be non-zero value,
    pub /: *mut *mut u16 max_rt; / max. resistance above which samples are ignored,
    pub /: *mut *mut unsigned long poll_period; / time (in ms) between samples,
    pub /: *mut *mut int fuzzx; / fuzz factor for X, Y and pressure axes,
    pub fuzzy: c_int,
    pub fuzzz: c_int,
    pub ): *mut *mut int (get_pendown_state)(struct device,
// If needed, clear 2nd level interrupt source
    pub (*clear_penirq)(void): *mut c_void,
    pub (*init_platform_hw)(void): *mut c_int,
    pub (*exit_platform_hw)(void): *mut c_void,
}
