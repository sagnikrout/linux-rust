//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/clk/versatile/icst.h
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
// Copyright (C) 2003 Deep Blue Solutions, Ltd, All Rights Reserved.
//
// Support functions for calculating clocks/divisors for the ICST
// clock generators.  See https://www.idt.com/ for more information
// on these devices.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct icst_params {
    pub ref: c_ulong,
    pub /: *mut *mut unsigned long vco_max; / inclusive,
    pub /: *mut *mut unsigned long vco_min; / exclusive,
    pub /: *mut *mut unsigned short vd_min; / inclusive,
    pub /: *mut *mut unsigned short vd_max; / inclusive,
    pub /: *mut *mut unsigned char rd_min; / inclusive,
    pub /: *mut *mut unsigned char rd_max; / inclusive,
    pub /: *const *const *const unsigned char s2div; / chip specific s2div array,
    pub /: *const *const *const unsigned char idx2s; / chip specific idx2s array,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icst_vco {
    pub v: c_ushort,
    pub r: c_uchar,
    pub s: c_uchar,
}

extern "C" {
    pub fn icst_hz(p: *const icst_params, vco: icst_vco) -> c_ulong;
}
extern "C" {
    pub fn icst_hz_to_vco(p: *const icst_params, freq: c_ulong) -> icst_vco;
}
//
// ICST307 VCO frequency must be between 6MHz and 200MHz (3.3 or 5V).
// This frequency is pre-output divider.
//
pub const ICST307_VCO_MIN: c_int = 6000000;
pub const ICST307_VCO_MAX: c_int = 200000000;
//
// ICST525 VCO frequency must be between 10MHz and 200MHz (3V) or 320MHz (5V).
// This frequency is pre-output divider.
//
pub const ICST525_VCO_MIN: c_int = 10000000;
pub const ICST525_VCO_MAX_3V: c_int = 200000000;
pub const ICST525_VCO_MAX_5V: c_int = 320000000;
