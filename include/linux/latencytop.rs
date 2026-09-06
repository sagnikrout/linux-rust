//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/latencytop.h
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
// latencytop.h: Infrastructure for displaying latency
//
// (C) Copyright 2008 Intel Corporation
// Author: Arjan van de Ven <arjan@linux.intel.com>
//

pub const LT_SAVECOUNT: c_int = 32;
pub const LT_BACKTRACEDEPTH: c_int = 12;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct latency_record {
    pub backtrace: [c_ulong; LT_BACKTRACEDEPTH],
    pub count: c_uint,
    pub time: c_ulong,
    pub max: c_ulong,
}

extern "C" {
    pub fn __account_scheduler_latency(task: *mut task_struct, usecs: c_int, inter: c_int);
}
extern "C" {
    pub fn clear_tsk_latency_tracing(p: *mut task_struct);
}

