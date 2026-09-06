//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/flex_proportions.h
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
// Floating proportions with flexible aging period
//
// Copyright (C) 2011, SUSE, Jan Kara <jack@suse.cz>
//

//
// When maximum proportion of some event type is specified, this is the
// precision with which we allow limitting. Note that this creates an upper
// bound on the number of events per period like
// ULLONG_MAX >> FPROP_FRAC_SHIFT.
//
pub const FPROP_FRAC_SHIFT: c_int = 10;

//
// ---- Global proportion definitions ----
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fprop_global {
// Number of events in the current period
    pub events: percpu_counter,
// Current period
    pub period: c_uint,
// Synchronization with period transitions
    pub sequence: seqcount_t,
}

extern "C" {
    pub fn fprop_global_init(p: *mut fprop_global, gfp: gfp_t) -> c_int;
}
extern "C" {
    pub fn fprop_global_destroy(p: *mut fprop_global);
}
extern "C" {
    pub fn fprop_new_period(p: *mut fprop_global, periods: c_int) -> bool;
}
//
// ---- PERCPU ----
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fprop_local_percpu {
// the local events counter
    pub events: percpu_counter,
// Period in which we last updated events
    pub period: c_uint,
    pub /: *mut *mut raw_spinlock_t lock; / Protect period and numerator,
}

extern "C" {
    pub fn fprop_local_init_percpu(pl: *mut fprop_local_percpu, gfp: gfp_t) -> c_int;
}
extern "C" {
    pub fn fprop_local_destroy_percpu(pl: *mut fprop_local_percpu);
}
