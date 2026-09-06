//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/sched/user.h
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
// Some day this will be a full-fledged user tracking system..
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct user_struct {
    pub /: *mut *mut refcount_t __count; / reference count,

    pub /: *mut *mut percpu_counter epoll_watches; / The number of file descriptors currently watched,

    pub /: *mut *mut unsigned long unix_inflight; / How many files in flight in unix sockets,
    pub /: *mut *mut atomic_long_t pipe_bufs; / how many pages are allocated in pipe buffers,
// Hash table maintenance information
    pub uidhash_node: hlist_node,
    pub uid: kuid_t,

    pub locked_vm: atomic_long_t,

    pub /: *mut *mut atomic_t nr_watches; / The number of watches this user currently has,

// Miscellaneous per-user rate limit
    pub ratelimit: ratelimit_state,
}

extern "C" {
    pub fn uids_sysfs_init() -> c_int;
}

// per-UID process charging.
extern "C" {
    pub fn alloc_uid(_arg: kuid_t) -> *mut user_struct;
}
extern "C" {
    pub fn free_uid(: *mut user_struct);
}
