//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/input/ti-drv260x.h
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
// DRV260X haptics driver family
//
// Author: Dan Murphy <dmurphy@ti.com>
//
// Copyright:   (C) 2014 Texas Instruments, Inc.
//
// Calibration Types
pub const DRV260X_LRA_MODE: c_uint = 0x00;
pub const DRV260X_LRA_NO_CAL_MODE: c_uint = 0x01;
pub const DRV260X_ERM_MODE: c_uint = 0x02;
// Library Selection
pub const DRV260X_LIB_EMPTY: c_uint = 0x00;
pub const DRV260X_ERM_LIB_A: c_uint = 0x01;
pub const DRV260X_ERM_LIB_B: c_uint = 0x02;
pub const DRV260X_ERM_LIB_C: c_uint = 0x03;
pub const DRV260X_ERM_LIB_D: c_uint = 0x04;
pub const DRV260X_ERM_LIB_E: c_uint = 0x05;
pub const DRV260X_LIB_LRA: c_uint = 0x06;
pub const DRV260X_ERM_LIB_F: c_uint = 0x07;
