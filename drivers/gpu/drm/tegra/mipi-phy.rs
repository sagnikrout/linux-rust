//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/tegra/mipi-phy.h
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
// Copyright (C) 2013 NVIDIA Corporation
//
// D-PHY timing parameters
//
// A detailed description of these parameters can be found in the  MIPI
// Alliance Specification for D-PHY, Section 5.9 "Global Operation Timing
// Parameters".
//
// All parameters are specified in nanoseconds.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mipi_dphy_timing {
    pub clkmiss: c_uint,
    pub clkpost: c_uint,
    pub clkpre: c_uint,
    pub clkprepare: c_uint,
    pub clksettle: c_uint,
    pub clktermen: c_uint,
    pub clktrail: c_uint,
    pub clkzero: c_uint,
    pub dtermen: c_uint,
    pub eot: c_uint,
    pub hsexit: c_uint,
    pub hsprepare: c_uint,
    pub hszero: c_uint,
    pub hssettle: c_uint,
    pub hsskip: c_uint,
    pub hstrail: c_uint,
    pub init: c_uint,
    pub lpx: c_uint,
    pub taget: c_uint,
    pub tago: c_uint,
    pub tasure: c_uint,
    pub wakeup: c_uint,
}
