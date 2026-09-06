//! Automatically rewritten from C Header to Rust Module
//! Source: include/rv/kunit.h
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
// Copyright (C) 2026-2029 Red Hat, Inc. Gabriele Monaco <gmonaco@redhat.com>
//
// Declaration of wrappers to allow mocking core functionality, like current,
// and other testing utilities.
// Necessary only when mocking may be needed. If the RV KUnit test is
// enabled, the wrappers incur an additional function call overhead.
//

extern "C" {
    pub fn rv_set_testing(suite: *mut kunit_suite) -> c_int;
}
extern "C" {
    pub fn rv_clear_testing(suite: *mut kunit_suite);
}
pub const RV_KUNIT_MAX_MOCK_TASKS: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rv_kunit_ctx {
    pub expected: int reactions,,
    pub mock_task_count: c_int,
    pub mock_tasks: [*mut task_struct; RV_KUNIT_MAX_MOCK_TASKS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rv_kunit_mon {
    pub rv_this: *mut rv_monitor,
    pub (*monitor_init)(void): *mut c_int,
    pub (*monitor_destroy)(void): *mut c_void,
    pub is_per_task: bool,
    pub task_slot: *mut c_int,
    pub task): *mut *mut void (task_reset)(struct task_struct,
}

extern "C" {
    pub fn prepare_test(test: *mut kunit, mon: *const rv_kunit_mon);
}
extern "C" {
    pub fn teardown_test(arg: *mut c_void);
}
extern "C" {
    pub fn rv_mock_current(tsk: *mut task_struct);
}

