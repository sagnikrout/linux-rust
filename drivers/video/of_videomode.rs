//! Automatically rewritten from C to Rust
//! Source: drivers/video/of_videomode.c
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
// generic videomode helper
//
// Copyright (c) 2012 Steffen Trumtrar <s.trumtrar@pengutronix.de>, Pengutronix
//

//
// of_get_videomode - get the videomode #<index> from devicetree
// @np: devicenode with the display_timings
// @vm: set to return value
// @index: index into list of display_timings
// (Set this to OF_USE_NATIVE_MODE to use whatever mode is
// specified as native mode in the DT.)
//
// DESCRIPTION:
// Get a list of all display timings and put the one
// specified by index into *vm. This function should only be used, if
// only one videomode is to be retrieved. A driver that needs to work
// with multiple/all videomodes should work with
// of_get_display_timings instead.
//
    int of_get_videomode(struct device_node *np, struct videomode *vm,
    int index)
    {
    struct display_timings *disp;
    int ret;
    disp = of_get_display_timings(np);
    if (!disp) {
    pr_err("%pOF: no timings specified\n", np);
    return -EINVAL;
    }
    if (index == OF_USE_NATIVE_MODE)
    index = disp.native_mode;
    ret = videomode_from_timings(disp, vm, index);
    display_timings_release(disp);
    return ret;
    }
    EXPORT_SYMBOL_GPL(of_get_videomode);
