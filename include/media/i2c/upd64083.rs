//! Automatically rewritten from C Header to Rust Module
//! Source: include/media/i2c/upd64083.h
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
// upd6408x - NEC Electronics 3-Dimensional Y/C separation input defines
//
// 2006 by Hans Verkuil (hverkuil@kernel.org)
//
// There are two bits of information that the driver needs in order
// Operating modes:
// YCS mode: Y/C separation (burst locked clocking)
pub const UPD64083_YCS_MODE: c_int = 0;
// YCS+ mode: 2D Y/C separation and YCNR (burst locked clocking)
pub const UPD64083_YCS_PLUS_MODE: c_int = 1;
// Note: the following two modes cannot be used in combination with the
// MNNR mode: frame comb type YNR+C delay (line locked clocking)
pub const UPD64083_MNNR_MODE: c_int = 2;
// YCNR mode: frame recursive YCNR (burst locked clocking)
pub const UPD64083_YCNR_MODE: c_int = 3;
// Select external Y-ADC: this should be set if this device is used in

