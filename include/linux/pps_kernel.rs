//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/pps_kernel.h
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
// PPS API kernel header
//
// Copyright (C) 2009   Rodolfo Giometti <giometti@linux.it>
//

//
// Global defines
//
// The specific PPS source info
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pps_source_info {
    pub /: *mut *mut char name[PPS_MAX_NAME_LEN]; / symbolic name,
    pub /: *mut *mut char path[PPS_MAX_NAME_LEN]; / path of connected device,
    pub /: *mut *mut int mode; / PPS allowed mode,
    pub /: *mut *mut *mut int event, void data); / PPS echo function,
    pub owner: *mut module,
    pub /: *mut *mut *mut device dev; / Parent device for device_create,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pps_event_time {

    pub ts_raw: timespec64,

    pub ts_real: timespec64,
}

// The main struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pps_device {
    pub /: *mut *mut pps_source_info info; / PSS source info,
    pub /: *mut *mut pps_kparams params; / PPS current params,
    pub /: *mut *mut __u32 assert_sequence; / PPS assert event seq #,
    pub /: *mut *mut __u32 clear_sequence; / PPS clear event seq #,
    pub assert_tu: pps_ktime,
    pub clear_tu: pps_ktime,
    pub /: *mut *mut int current_mode; / PPS mode at event time,
    pub /: *mut *mut unsigned int last_ev; / last PPS event id,
    pub /: *mut *mut unsigned int last_fetched_ev; / last fetched PPS event id,
    pub /: *mut *mut wait_queue_head_t queue; / PPS event queue,
    pub /: *mut *mut unsigned int id; / PPS source unique ID,
    pub /: *const *const *const void lookup_cookie; / For pps_lookup_dev() only,
    pub dev: device,
    pub /: *mut *mut *mut fasync_async_queue; / fasync method,
    pub lock: spinlock_t,
    pub kc_removed: bool,
}

//
// Global variables
//
// Internal functions.
//
// These are not actually part of the exported API, but this is a
// convenient header file to put them in.
//
extern "C" {
    pub fn pps_register_cdev(pps: *mut pps_device) -> c_int;
}
extern "C" {
    pub fn pps_unregister_cdev(pps: *mut pps_device);
}
//
// Exported functions
//
extern "C" {
    pub fn pps_unregister_source(pps: *mut pps_device);
}
// Look up a pps_device by magic cookie

// Subtract known time delay from PPS event time(s)

