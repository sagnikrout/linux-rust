//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/acct.h
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


// SPDX-License-Identifier: GPL-2.0
//
// BSD Process Accounting for Linux - Definitions
//
// Author: Marco van Wieringen (mvw@planets.elm.net)
//
// This header file contains the definitions needed to implement
// BSD-style process accounting. The kernel accounting code and all
// user-level programs that try to do something useful with the
// process accounting log must include this file.
//
// Copyright (C) 1995 - 1997 Marco van Wieringen - ELM Consultancy B.V.
//

extern "C" {
    pub fn acct_collect(exitcode: c_long, group_dead: c_int);
}
extern "C" {
    pub fn acct_process();
}
extern "C" {
    pub fn acct_exit_ns(: *mut pid_namespace);
}

//
// ACCT_VERSION numbers as yet defined:
// 0: old format (until 2.6.7) with 16 bit uid/gid
// 1: extended variant (binary compatible on M68K)
// 2: extended variant (binary compatible on everything except M68K)
// 3: new binary incompatible format (64 bytes)
// 4: new binary incompatible format (128 bytes)
// 5: new binary incompatible format (128 bytes, second half)
//

pub const ACCT_VERSION: c_int = 3;
pub const AHZ: c_int = 100;
pub type acct_t = acct_v3;

pub const ACCT_VERSION: c_int = 1;

pub const ACCT_VERSION: c_int = 2;

pub type acct_t = acct;

//
// Yet another set of HZ to *HZ helper functions.
// See <linux/jiffies.h> for the original.
//

//
// max relative error 5.7e-8 (1.8s per year) for AHZ <= 1024,
// overflow after 64.99 years.
// exact for AHZ=60, 72, 90, 120, 144, 180, 300, 600, 900, ...
//

