//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/local_lock_internal.h
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

pub type local_lock_t = local_lock;
// local_trylock() and local_trylock_irqsave() only work with local_trylock_t

pub type local_trylock_t = local_trylock;

// preemption or migration must be disabled before calling __local_lock_is_locked

//
// On PREEMPT_RT local_lock maps to a per CPU spinlock, which protects the
// critical section while staying preemptible.
//
pub type local_lock_t = spinlock_t;
pub type local_trylock_t = spinlock_t;

// migration must be disabled before calling __local_lock_is_locked

//
// Because the compiler only knows about the base per-CPU variable, use this
// helper function to make the compiler think we lock/unlock the @base variable,
// and hide the fact we actually pass the per-CPU instance to lock/unlock
// functions.
//
extern "C" {
    pub fn this_cpu_ptr(_arg: base) -> return;
}

extern "C" {
    pub fn this_cpu_ptr(_arg: base) -> return;
}

