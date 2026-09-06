//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/i2c-algo-pcf.h
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
// -------------------------------------------------------------------------
// adap-pcf.h i2c driver algorithms for PCF8584 adapters
// -------------------------------------------------------------------------
// Copyright (C) 1995-97 Simon G. Vogl
//
// -------------------------------------------------------------------------
// With some changes from Kyösti Mälkki <kmalkki@cc.hut.fi> and even
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i2c_algo_pcf_data {
    pub /: *mut *mut *mut void data; / private data for lolevel routines,
    pub val): *mut *mut *mut void (setpcf) (void data, int ctl, int,
    pub ctl): *mut *mut *mut int (getpcf) (void data, int,
    pub data): *mut *mut int (getown) (void,
    pub data): *mut *mut int (getclock) (void,
    pub data): *mut *mut void (waitforpin) (void,
    pub data): *mut *mut void (xfer_begin) (void,
    pub data): *mut *mut void (xfer_end) (void,
// Multi-master lost arbitration back-off delay (msecs)
// This should be set by the bus adapter or knowledgable client
// if bus is multi-mastered, else zero
//
    pub lab_mdelay: c_ulong,
}

extern "C" {
    pub fn i2c_pcf_add_bus(: *mut i2c_adapter) -> c_int;
}
