//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/platform_data/omap-wd-timer.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// OMAP2+ WDTIMER-specific function prototypes
//
// Copyright (C) 2012 Texas Instruments, Inc.
// Paul Walmsley
//

//
// Standardized OMAP reset source bits
//
// This is a subset of the ones listed in arch/arm/mach-omap2/prm.h
// and are the only ones needed in the watchdog driver.
//
pub const OMAP_MPU_WD_RST_SRC_ID_SHIFT: c_int = 3;
//
// struct omap_wd_timer_platform_data - WDTIMER integration to the host SoC
// @read_reset_sources - fn ptr for the SoC to indicate the last reset cause
//
// The function pointed to by @read_reset_sources must return its data
// in a standard format - search for RST_SRC_ID_SHIFT in
// arch/arm/mach-omap2
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap_wd_timer_platform_data {
    pub (*read_reset_sources)(void): *mut u32,
}
