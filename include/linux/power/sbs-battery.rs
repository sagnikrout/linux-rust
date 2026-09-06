//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/power/sbs-battery.h
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
// Gas Gauge driver for SBS Compliant Gas Gauges
//
// Copyright (c) 2010, NVIDIA Corporation.
//

//
// struct sbs_platform_data - platform data for sbs devices
// @i2c_retry_count:		# of times to retry on i2c IO failure
// @poll_retry_count:		# of times to retry looking for new status after
// external change notification
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sbs_platform_data {
    pub i2c_retry_count: u32,
    pub poll_retry_count: u32,
}
