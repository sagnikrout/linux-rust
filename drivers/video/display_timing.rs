//! Automatically rewritten from C to Rust
//! Source: drivers/video/display_timing.c
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
// generic display timing functions
//
// Copyright (c) 2012 Steffen Trumtrar <s.trumtrar@pengutronix.de>, Pengutronix
//

#[no_mangle]
pub unsafe extern "C" fn display_timings_release(disp: *mut display_timings) {
    void display_timings_release(struct display_timings *disp)
    {
    if (disp.timings) {
    unsigned int i;
    for (i = 0; i < disp.num_timings; i++)
    kfree(disp.timings[i]);
    kfree(disp.timings);
    }
    kfree(disp);
    }
    EXPORT_SYMBOL_GPL(display_timings_release);
