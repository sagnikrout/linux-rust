//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/semaphore.h
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
//
// Copyright (c) 2008 Intel Corporation
// Author: Matthew Wilcox <willy@linux.intel.com>
//
// Please see kernel/locking/semaphore.c for documentation of these functions
//

// Please don't access any members of this structure directly
#[repr(C)]
#[derive(Copy, Clone)]
pub struct semaphore {
    pub lock: raw_spinlock_t,
    pub count: c_uint,
    pub first_waiter: *mut semaphore_waiter,

    pub last_holder: c_ulong,

}

//
// Unlike mutexes, binary semaphores do not have an owner, so up() can
// be called in a different thread from the one which called down().
// It is also safe to call down_trylock() and up() from interrupt
// context.
//

// sem = (struct semaphore) __SEMAPHORE_INITIALIZER(*sem, val);
extern "C" {
    pub fn down(sem: *mut semaphore);
}
extern "C" {
    pub fn down_interruptible(sem: *mut semaphore) -> int __must_check;
}
extern "C" {
    pub fn down_killable(sem: *mut semaphore) -> int __must_check;
}
extern "C" {
    pub fn down_trylock(sem: *mut semaphore) -> int __must_check;
}
extern "C" {
    pub fn down_timeout(sem: *mut semaphore, jiffies: c_long) -> int __must_check;
}
extern "C" {
    pub fn up(sem: *mut semaphore);
}
extern "C" {
    pub fn sem_last_holder(sem: *mut semaphore) -> c_ulong;
}
