//! Automatically rewritten from C to Rust
//! Source: drivers/video/videomode.c
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

    void videomode_from_timing(const struct display_timing *dt,
    struct videomode *vm)
    {
    vm.pixelclock = dt.pixelclock.typ;
    vm.hactive = dt.hactive.typ;
    vm.hfront_porch = dt.hfront_porch.typ;
    vm.hback_porch = dt.hback_porch.typ;
    vm.hsync_len = dt.hsync_len.typ;
    vm.vactive = dt.vactive.typ;
    vm.vfront_porch = dt.vfront_porch.typ;
    vm.vback_porch = dt.vback_porch.typ;
    vm.vsync_len = dt.vsync_len.typ;
    vm.flags = dt.flags;
    }
    EXPORT_SYMBOL_GPL(videomode_from_timing);
    int videomode_from_timings(const struct display_timings *disp,
    struct videomode *vm, unsigned int index)
    {
    struct display_timing *dt;
    dt = display_timings_get(disp, index);
    if (!dt)
    return -EINVAL;
    videomode_from_timing(dt, vm);
    return 0;
    }
    EXPORT_SYMBOL_GPL(videomode_from_timings);
