//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/devfreq_cooling.h
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
// devfreq_cooling: Thermal cooling device implementation for devices using
// devfreq
//
// Copyright (C) 2014-2015 ARM Limited
//

//
// struct devfreq_cooling_power - Devfreq cooling power ops
// @get_real_power:	When this is set, the framework uses it to ask the
// device driver for the actual power.
// Some devices have more sophisticated methods
// (like power counters) to approximate the actual power
// that they use.
// This function provides more accurate data to the
// thermal governor. When the driver does not provide
// such function, framework just uses pre-calculated
// table and scale the power by 'utilization'
// (based on 'busy_time' and 'total_time' taken from
// devfreq 'last_status').
// The value returned by this function must be lower
// or equal than the maximum power value
// for the current	state
// (which can be found in power_table[state]).
// When this interface is used, the power_table holds
// max total (static + dynamic) power value for each OPP.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct devfreq_cooling_power {
    pub voltage): unsigned long freq, unsigned long,
}

extern "C" {
    pub fn devfreq_cooling_unregister(dfc: *mut thermal_cooling_device);
}

extern "C" {
    pub fn ERR_PTR(_arg: -EINVAL) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -EINVAL) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -EINVAL) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -EINVAL) -> return;
}

