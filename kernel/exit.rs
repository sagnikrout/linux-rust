//! Automatically rewritten from C Header to Rust Module
//! Source: kernel/exit.h
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


// SPDX-License-Identifier: GPL-2.0-only
#[repr(C)]
#[derive(Copy, Clone)]
pub struct waitid_info {
    pub pid: pid_t,
    pub uid: uid_t,
    pub status: c_int,
    pub cause: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wait_opts {
    pub wo_type: pid_type,
    pub wo_flags: c_int,
    pub wo_pid: *mut pid,
    pub wo_info: *mut waitid_info,
    pub wo_stat: c_int,
    pub wo_rusage: *mut rusage,
    pub child_wait: wait_queue_entry_t,
    pub notask_error: c_int,
}

extern "C" {
    pub fn pid_child_should_wake(wo: *mut wait_opts, p: *mut task_struct) -> bool;
}
extern "C" {
    pub fn __do_wait(wo: *mut wait_opts) -> c_long;
}
