//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/i2c/ccs/ccs-quirk.h
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
// drivers/media/i2c/ccs/ccs-quirk.h
//
// Generic driver for MIPI CCS/SMIA/SMIA++ compliant camera sensors
//
// Copyright (C) 2020 Intel Corporation
// Copyright (C) 2011--2012 Nokia Corporation
// Contact: Sakari Ailus <sakari.ailus@linux.intel.com>
//
// struct ccs_quirk - quirks for sensors that deviate from SMIA++ standard
//
// @limits: Replace sensor->limits with values which can't be read from
// sensor registers. Called the first time the sensor is powered up.
// @post_poweron: Called always after the sensor has been fully powered on.
// @pre_streamon: Called just before streaming is enabled.
// @post_streamoff: Called right after stopping streaming.
// @pll_flags: Return flags for the PLL calculator.
// @init: Quirk initialisation, called the last in probe(). This is
// also appropriate for adding sensor specific controls, for instance.
// @reg_access: Register access quirk. The quirk may divert the access
// to another register, or no register at all.
//
// -write: Is this read (false) or write (true) access?
// -reg:   Pointer to the register to access
// -val:   Register value, set by the caller on write, or
// by the quirk on read
// -return: 0 on success, -ENOIOCTLCMD if no register
// access may be done by the caller (default read
// value is zero), else negative error code on error
// @flags: Quirk flags
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccs_quirk {
    pub sensor): *mut *mut int (limits)(struct ccs_sensor,
    pub sensor): *mut *mut int (post_poweron)(struct ccs_sensor,
    pub sensor): *mut *mut int (pre_streamon)(struct ccs_sensor,
    pub sensor): *mut *mut int (post_streamoff)(struct ccs_sensor,
    pub sensor): *mut *mut unsigned long (pll_flags)(struct ccs_sensor,
    pub sensor): *mut *mut int (init)(struct ccs_sensor,
    pub val): *mut u32,
    pub flags: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccs_reg_8 {
    pub reg: u16,
    pub val: u8,
}

