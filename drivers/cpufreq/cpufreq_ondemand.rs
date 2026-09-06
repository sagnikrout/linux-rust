//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/cpufreq/cpufreq_ondemand.h
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
// Header file for CPUFreq ondemand governor and related code.
//
// Copyright (C) 2016, Intel Corporation
// Author: Rafael J. Wysocki <rafael.j.wysocki@intel.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct od_policy_dbs_info {
    pub policy_dbs: policy_dbs_info,
    pub freq_lo: c_uint,
    pub freq_lo_delay_us: c_uint,
    pub freq_hi_delay_us: c_uint,
    pub sample_type:1: c_uint,
}

extern "C" {
    pub fn container_of(_arg: policy_dbs, od_policy_dbs_info: struct, _arg: policy_dbs) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct od_dbs_tuners {
    pub powersave_bias: c_uint,
}

//
// Not all CPUs want IO time to be accounted as busy; this depends on
// how efficient idling at a higher frequency/voltage is.
//
// Pavel Machek says this is not so for various generations of AMD and
// old Intel systems. Mike Chan (android.com) claims this is also not
// true for ARM.
//
// Because of this, select a known series of Intel CPUs (Family 6 and
// later) by default, and leave all others up to the user.
//

