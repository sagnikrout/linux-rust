//! Automatically rewritten from C Header to Rust Module
//! Source: kernel/sched/autogroup.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct autogroup {
//
// Reference doesn't mean how many threads attach to this
// autogroup now. It just stands for the number of tasks
// which could use this autogroup.
//
    pub kref: kref,
    pub tg: *mut task_group,
    pub lock: rw_semaphore,
    pub id: c_ulong,
    pub nice: c_int,
}

extern "C" {
    pub fn autogroup_init(init_task: *mut task_struct);
}
extern "C" {
    pub fn autogroup_free(tg: *mut task_group);
}
extern "C" {
    pub fn task_wants_autogroup(p: *mut task_struct, tg: *mut task_group) -> bool;
}
extern "C" {
    pub fn autogroup_path(tg: *mut task_group, buf: *mut c_char, buflen: c_int) -> c_int;
}

