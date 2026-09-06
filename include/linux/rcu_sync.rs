//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/rcu_sync.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// RCU-based infrastructure for lightweight reader-writer locking
//
// Copyright (c) 2015, Red Hat, Inc.
//
// Author: Oleg Nesterov <oleg@redhat.com>
//

// Structure to mediate between updaters and fastpath-using readers.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rcu_sync {
    pub gp_state: c_int,
    pub gp_count: c_int,
    pub gp_wait: wait_queue_head_t,
    pub cb_head: rcu_head,
}

//
// rcu_sync_is_idle() - Are readers permitted to use their fastpaths?
// @rsp: Pointer to rcu_sync structure to use for synchronization
//
// Returns true if readers are permitted to use their fastpaths.  Must be
// invoked within some flavor of RCU read-side critical section.
//
extern "C" {
    pub fn rcu_sync_init(: *mut rcu_sync);
}
extern "C" {
    pub fn rcu_sync_enter(: *mut rcu_sync);
}
extern "C" {
    pub fn rcu_sync_exit(: *mut rcu_sync);
}
extern "C" {
    pub fn rcu_sync_dtor(: *mut rcu_sync);
}

