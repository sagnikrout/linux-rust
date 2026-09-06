//! Automatically rewritten from C Header to Rust Module
//! Source: kernel/sched/ext/arena.h
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
// BPF extensible scheduler class: Documentation/scheduler/sched-ext.rst
//
// Copyright (c) 2025 Meta Platforms, Inc. and affiliates.
// Copyright (c) 2025 Tejun Heo <tj@kernel.org>
//

extern "C" {
    pub fn scx_arena_pool_init(sch: *mut scx_sched) -> i32;
}
extern "C" {
    pub fn scx_arena_pool_destroy(sch: *mut scx_sched);
}
extern "C" {
    pub fn scx_arena_free(sch: *mut scx_sched, kern_va: *mut c_void, size: usize);
}
