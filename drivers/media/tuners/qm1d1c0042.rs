//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/tuners/qm1d1c0042.h
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
// Sharp QM1D1C0042 8PSK tuner driver
//
// Copyright (C) 2014 Akihiro Tsukada <tskd08@gmail.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qm1d1c0042_config {
    pub fe: *mut dvb_frontend,
    pub /: *mut *mut *mut *mut u32 xtal_freq; / [kHz] / / currently ignored,
    pub /: *mut *mut bool lpf; / enable LPF,
    pub /: *mut *mut bool fast_srch; / enable fast search mode, no LPF,
    pub /: *mut *mut u32 lpf_wait; / wait in tuning with LPF enabled. [ms],
    pub /: *mut *mut u32 fast_srch_wait; / with fast-search mode, no LPF. [ms],
    pub /: *mut *mut u32 normal_srch_wait; / with no LPF/fast-search mode. [ms],
}

// special values indicating to use the default in qm1d1c0042_config
pub const QM1D1C0042_CFG_XTAL_DFLT: c_int = 0;
pub const QM1D1C0042_CFG_WAIT_DFLT: c_int = 0;
