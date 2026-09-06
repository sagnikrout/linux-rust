//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/rcu_notifier.h
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
// Read-Copy Update notifiers, initially RCU CPU stall notifier.
// Separate from rcupdate.h to avoid #include loops.
//
// Copyright (C) 2023 Paul E. McKenney.
//
// Actions for RCU CPU stall notifier calls.
pub const RCU_STALL_NOTIFY_NORM: c_int = 1;
pub const RCU_STALL_NOTIFY_EXP: c_int = 2;

extern "C" {
    pub fn rcu_stall_chain_notifier_register(n: *mut notifier_block) -> c_int;
}
extern "C" {
    pub fn rcu_stall_chain_notifier_unregister(n: *mut notifier_block) -> c_int;
}

// No RCU CPU stall warnings in Tiny RCU.

