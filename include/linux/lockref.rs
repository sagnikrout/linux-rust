//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/lockref.h
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
// Locked reference counts.
//
// These are different from just plain atomic refcounts in that they
// are atomic with respect to the spinlock that goes with them.  In
// particular, there can be implementations that don't actually get
// the spinlock for the common decrement/increment operations, but they
// still have to check that the operation is done semantically as if
// the spinlock had been taken (using a cmpxchg operation that covers
// both the lock and the count word, or using memory transactions, for
// example).
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lockref {

    pub lock_count: aligned_u64,

    pub lock: spinlock_t,
    pub count: c_int,
}

//
// lockref_init - Initialize a lockref
// @lockref: pointer to lockref structure
//
// Initializes @lockref->count to 1.
//
extern "C" {
    pub fn lockref_get(lockref: *mut lockref);
}
extern "C" {
    pub fn lockref_put_return(lockref: *mut lockref) -> c_int;
}
extern "C" {
    pub fn lockref_get_not_zero(lockref: *mut lockref) -> bool;
}
extern "C" {
    pub fn lockref_put_or_lock(__cond_acquires(false: *mut *mut lockref lockref), _arg: &lockref->lock) -> bool;
}
extern "C" {
    pub fn lockref_mark_dead(lockref: *mut lockref);
}
extern "C" {
    pub fn lockref_get_not_dead(lockref: *mut lockref) -> bool;
}
// Must be called under spinlock for reliable results
