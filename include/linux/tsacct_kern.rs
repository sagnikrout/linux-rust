//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/tsacct_kern.h
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
// tsacct_kern.h - kernel header for system accounting over taskstats interface
//
// Copyright (C) Jay Lan	SGI
//

extern "C" {
    pub fn xacct_add_tsk(stats: *mut taskstats, p: *mut task_struct);
}
extern "C" {
    pub fn acct_update_integrals(tsk: *mut task_struct);
}
extern "C" {
    pub fn acct_account_cputime(tsk: *mut task_struct);
}
extern "C" {
    pub fn acct_clear_integrals(tsk: *mut task_struct);
}

