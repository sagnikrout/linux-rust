//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/pps_gen_kernel.h
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
// PPS generator API kernel header
//
// Copyright (C) 2024 Rodolfo Giometti <giometti@enneenne.com>
//

//
// Global defines
//

//
// struct pps_gen_source_info - the specific PPS generator info
// @use_system_clock: true, if the system clock is used to generate pulses
// @get_time: query the time stored into the generator clock
// @enable: enable/disable the PPS pulses generation
//
// This is the main generator struct where all needed information must be
// placed before calling the pps_gen_register_source().
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pps_gen_source_info {
    pub use_system_clock: bool,
    pub time): *mut timespec64,
    pub enable): *mut *mut *mut int (enable)(struct pps_gen_device pps_gen, bool,
// private: internal use only
    pub owner: *mut module,
    pub /: *mut *mut *mut device parent; / for device_create,
}

// The main struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pps_gen_device {
    pub /: *const *const *const pps_gen_source_info info; / PSS generator info,
    pub /: *mut *mut bool enabled; / PSS generator status,
    pub event: c_uint,
    pub sequence: c_uint,
    pub /: *mut *mut unsigned int last_ev; / last PPS event id,
    pub /: *mut *mut wait_queue_head_t queue; / PPS event queue,
    pub /: *mut *mut unsigned int id; / PPS generator unique ID,
    pub cdev: cdev,
    pub dev: *mut device,
    pub /: *mut *mut *mut fasync_async_queue; / fasync method,
    pub lock: spinlock_t,
}

//
// Global variables
//
// Exported functions
//
extern "C" {
    pub fn pps_gen_unregister_source(pps_gen: *mut pps_gen_device);
}
