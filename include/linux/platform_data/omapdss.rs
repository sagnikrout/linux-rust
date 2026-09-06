//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/platform_data/omapdss.h
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
// Copyright (C) 2016 Texas Instruments, Inc.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum omapdss_version {
    OMAPDSS_VER_UNKNOWN = 0,
    OMAPDSS_VER_OMAP24xx,
    OMAPDSS_VER_OMAP34xx_ES1,	/* OMAP3430 ES1.0, 2.0 */
    OMAPDSS_VER_OMAP34xx_ES3,	/* OMAP3430 ES3.0+ */
    OMAPDSS_VER_OMAP3630,
    OMAPDSS_VER_AM35xx,
    OMAPDSS_VER_OMAP4430_ES1,	/* OMAP4430 ES1.0 */
    OMAPDSS_VER_OMAP4430_ES2,	/* OMAP4430 ES2.0, 2.1, 2.2 */
    OMAPDSS_VER_OMAP4,		/* All other OMAP4s */
    OMAPDSS_VER_OMAP5,
    OMAPDSS_VER_AM43xx,
    OMAPDSS_VER_DRA7xx,
}

// Board specific data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap_dss_board_info {
    pub lane_mask): *mut *mut int (dsi_enable_pads)(int dsi_id, unsigned int,
    pub lane_mask): *mut *mut void (dsi_disable_pads)(int dsi_id, unsigned int,
    pub r): *mut *mut *mut int (set_min_bus_tput)(struct device dev, unsigned long,
    pub version: omapdss_version,
}
