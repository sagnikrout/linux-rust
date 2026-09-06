//! Automatically rewritten from C Header to Rust Module
//! Source: include/media/drv-intf/si476x.h
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
// include/media/drv-intf/si476x.h -- Common definitions for si476x driver
//
// Copyright (C) 2012 Innovative Converged Devices(ICD)
// Copyright (C) 2013 Andrey Smirnov
//
// Author: Andrey Smirnov <andrew.smirnov@gmail.com>
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum si476x_ctrl_id {
    V4L2_CID_SI476X_RSSI_THRESHOLD	= (V4L2_CID_USER_SI476X_BASE + 1),
    V4L2_CID_SI476X_SNR_THRESHOLD	= (V4L2_CID_USER_SI476X_BASE + 2),
    V4L2_CID_SI476X_MAX_TUNE_ERROR	= (V4L2_CID_USER_SI476X_BASE + 3),
    V4L2_CID_SI476X_HARMONICS_COUNT	= (V4L2_CID_USER_SI476X_BASE + 4),
    V4L2_CID_SI476X_DIVERSITY_MODE	= (V4L2_CID_USER_SI476X_BASE + 5),
    V4L2_CID_SI476X_INTERCHIP_LINK	= (V4L2_CID_USER_SI476X_BASE + 6),
}
