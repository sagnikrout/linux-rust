//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/sched/exec_state.h
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
// Copyright (c) 2026 Christian Brauner <brauner@kernel.org>

#[repr(C)]
#[derive(Copy, Clone)]
pub struct task_exec_state {
    pub count: refcount_t,
    pub dumpable: task_dumpable,
    pub user_ns: *mut user_namespace,
    pub rcu: rcu_head,
}

extern "C" {
    pub fn put_task_exec_state(exec_state: *mut task_exec_state);
}
extern "C" {
    pub fn task_exec_state_copy(tsk: *mut task_struct) -> c_int;
}
extern "C" {
    pub fn exec_state_init() -> void __init;
}
