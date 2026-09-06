//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/kcsan.h
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
// The Kernel Concurrency Sanitizer (KCSAN) infrastructure. Public interface and
// data structures to set up runtime. See kcsan-checks.h for explicit checks and
// modifiers. For more info please see Documentation/dev-tools/kcsan.rst.
//
// Copyright (C) 2019, Google LLC.
//

//
// Context for each thread of execution: for tasks, this is stored in
// task_struct, and interrupts access internal per-CPU storage.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kcsan_ctx {
    pub /: *mut *mut int disable_count; / disable counter,
    pub /: *mut *mut int disable_scoped; / disable scoped access counter,
    pub /: *mut *mut int atomic_next; / number of following atomic ops,
//
// We distinguish between: (a) nestable atomic regions that may contain
// other nestable regions; and (b) flat atomic regions that do not keep
// track of nesting. Both (a) and (b) are entirely independent of each
// other, and a flat region may be started in a nestable region or
// vice-versa.
//
// This is required because, for example, in the annotations for
// seqlocks, we declare seqlock writer critical sections as (a) nestable
// atomic regions, but reader critical sections as (b) flat atomic
// regions, but have encountered cases where seqlock reader critical
// sections are contained within writer critical sections (the opposite
// may be possible, too).
//
// To support these cases, we independently track the depth of nesting
// for (a), and whether the leaf level is flat for (b).
//
    pub atomic_nest_count: c_int,
    pub in_flat_atomic: bool,
//
// Access mask for all accesses if non-zero.
//
    pub access_mask: c_ulong,
// List of scoped accesses; likely to be empty.
    pub scoped_accesses: list_head,

//
// Scoped access for modeling access reordering to detect missing memory
// barriers; only keep 1 to keep fast-path complexity manageable.
//
    pub reorder_access: kcsan_scoped_access,

}

//
// kcsan_init - initialize KCSAN runtime
//
extern "C" {
    pub fn kcsan_init();
}

