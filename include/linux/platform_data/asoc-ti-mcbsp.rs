//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/platform_data/asoc-ti-mcbsp.h
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
// Defines for Multi-Channel Buffered Serial Port
//
// Copyright (C) 2002 RidgeRun, Inc.
// Author: Steve Johnson
//

// Platform specific configuration
#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap_mcbsp_ops {
    pub int): *mut *mut void (request)(unsigned,
    pub int): *mut *mut void (free)(unsigned,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap_mcbsp_platform_data {
    pub ops: *mut omap_mcbsp_ops,
    pub buffer_size: u16,
    pub reg_size: u8,
    pub reg_step: u8,
// McBSP platform and instance specific features
    pub /: *mut *mut bool has_wakeup; / Wakeup capability,
    pub /: *mut *mut bool has_ccr; / Transceiver has configuration control registers,
    pub force_on): *mut *mut *mut int (force_ick_on)(struct clk clk, bool,
}

extern "C" {
    pub fn omap3_mcbsp_init_pdata_callback(pdata: *mut omap_mcbsp_platform_data);
}
