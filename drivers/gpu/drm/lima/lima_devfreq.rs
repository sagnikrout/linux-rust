//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/lima/lima_devfreq.h
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
// Copyright 2020 Martin Blumenstingl <martin.blumenstingl@googlemail.com>

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lima_devfreq {
    pub devfreq: *mut devfreq,
    pub cooling: *mut thermal_cooling_device,
    pub gov_data: devfreq_simple_ondemand_data,
    pub busy_time: ktime_t,
    pub idle_time: ktime_t,
    pub time_last_update: ktime_t,
    pub busy_count: c_int,
//
// Protect busy_time, idle_time, time_last_update and busy_count
// because these can be updated concurrently, for example by the GP
// and PP interrupts.
//
    pub lock: spinlock_t,
}

extern "C" {
    pub fn lima_devfreq_init(ldev: *mut lima_device) -> c_int;
}
extern "C" {
    pub fn lima_devfreq_fini(ldev: *mut lima_device);
}
extern "C" {
    pub fn lima_devfreq_record_busy(devfreq: *mut lima_devfreq);
}
extern "C" {
    pub fn lima_devfreq_record_idle(devfreq: *mut lima_devfreq);
}
extern "C" {
    pub fn lima_devfreq_resume(devfreq: *mut lima_devfreq) -> c_int;
}
extern "C" {
    pub fn lima_devfreq_suspend(devfreq: *mut lima_devfreq) -> c_int;
}
