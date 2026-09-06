//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/video/fbdev/via/via_clock.h
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
// Copyright 1998-2008 VIA Technologies, Inc. All Rights Reserved.
// Copyright 2001-2008 S3 Graphics, Inc. All Rights Reserved.
// Copyright 2011 Florian Tobias Schandinat <FlorianSchandinat@gmx.de>
//
// clock and PLL management functions
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum via_clksrc {
    VIA_CLKSRC_X1 = 0,
    VIA_CLKSRC_TVX1,
    VIA_CLKSRC_TVPLL,
    VIA_CLKSRC_DVP1TVCLKR,
    VIA_CLKSRC_CAP0,
    VIA_CLKSRC_CAP1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct via_pll_config {
    pub multiplier: u16,
    pub divisor: u8,
    pub rshift: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct via_clock {
    pub state): *mut *mut void (set_primary_clock_state)(u8,
    pub use_pll): *mut *mut void (set_primary_clock_source)(enum via_clksrc src, bool,
    pub state): *mut *mut void (set_primary_pll_state)(u8,
    pub config): *mut *mut void (set_primary_pll)(struct via_pll_config,
    pub state): *mut *mut void (set_secondary_clock_state)(u8,
    pub use_pll): *mut *mut void (set_secondary_clock_source)(enum via_clksrc src, bool,
    pub state): *mut *mut void (set_secondary_pll_state)(u8,
    pub config): *mut *mut void (set_secondary_pll)(struct via_pll_config,
    pub state): *mut *mut void (set_engine_pll_state)(u8,
    pub config): *mut *mut void (set_engine_pll)(struct via_pll_config,
}

extern "C" {
    pub fn via_clock_init(clock: *mut via_clock, gfx_chip: c_int);
}
