//! Automatically rewritten from C Header to Rust Module
//! Source: include/video/videomode.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright 2012 Steffen Trumtrar <s.trumtrar@pengutronix.de>
//
// generic videomode description
//

//
// Subsystem independent description of a videomode.
// Can be generated from struct display_timing.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct videomode {
    pub /: *mut *mut unsigned long pixelclock; / pixelclock in Hz,
    pub hactive: u32,
    pub hfront_porch: u32,
    pub hback_porch: u32,
    pub hsync_len: u32,
    pub vactive: u32,
    pub vfront_porch: u32,
    pub vback_porch: u32,
    pub vsync_len: u32,
    pub /: *mut *mut display_flags flags; / display flags,
}

//
// videomode_from_timing - convert display timing to videomode
// @dt: display_timing structure
// @vm: return value
//
// DESCRIPTION:
// This function converts a struct display_timing to a struct videomode.
//
// videomode_from_timings - convert one display timings entry to videomode
// @disp: structure with all possible timing entries
// @vm: return value
// @index: index into the list of display timings in devicetree
//
// DESCRIPTION:
// This function converts one struct display_timing entry to a struct videomode.
//
