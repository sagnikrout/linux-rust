//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/ethosu/ethosu_drv.h
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


// SPDX-License-Identifier: GPL-2.0-only OR MIT
// Copyright 2025-2026 Arm, Ltd.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ethosu_file_priv {
    pub edev: *mut ethosu_device,
    pub sched_entity: drm_sched_entity,
    pub perfmons: xarray,
}

// Performance monitor object. The perfmon lifetime is controlled by userspace
// using perfmon related ioctls. A perfmon can be attached to a DRM_ETHOSU_SUBMIT
// request, and when this is the case, HW perf counters will be activated just
// before the job is submitted to the NPU and disabled when the job is
// done. This way, only events related to a specific job will be counted.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ethosu_perfmon {
// Tracks the number of users of the perfmon, when this counter reaches
// zero the perfmon is destroyed.
//
    pub refcnt: refcount_t,
// Number of counters activated in this perfmon instance
// (should be less than or equal to DRM_ETHOSU_MAX_PERF_COUNTERS).
//
    pub ncounters: u8,
// Events counted by the HW perf counters.
    pub counters: [u16; DRM_ETHOSU_MAX_PERF_EVENT_COUNTERS],
//
// Storage for counter values. Counters are incremented by the HW
// perf counter values every time the perfmon is attached to an
// NPU job. This way, perfmon users don't have to retrieve the
// results after each job if they want to track events covering
// several submissions. Note that counter values can't be reset,
// but you can fake a reset by destroying the perfmon and
// creating a new one.
//
    pub __counted_by(ncounters): u64 values[],
}

// ethosu_perfmon.c
extern "C" {
    pub fn ethosu_perfmon_get(perfmon: *mut ethosu_perfmon);
}
extern "C" {
    pub fn ethosu_perfmon_put(perfmon: *mut ethosu_perfmon);
}
extern "C" {
    pub fn ethosu_perfmon_open_file(ethosu_priv: *mut ethosu_file_priv);
}
extern "C" {
    pub fn ethosu_perfmon_close_file(ethosu_priv: *mut ethosu_file_priv);
}
