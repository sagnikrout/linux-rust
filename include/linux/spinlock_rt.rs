//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/spinlock_rt.h
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

extern "C" {
    pub fn rt_spin_lock(__acquires(lock: *mut *mut spinlock_t lock));
}
extern "C" {
    pub fn rt_spin_lock_nested(lock: *mut spinlock_t, __acquires(lock: int subclass));
}
extern "C" {
    pub fn rt_spin_lock_nest_lock(lock: *mut spinlock_t, __acquires(lock: *mut *mut lockdep_map nest_lock));
}
extern "C" {
    pub fn rt_spin_unlock(__releases(lock: *mut *mut spinlock_t lock));
}
extern "C" {
    pub fn rt_spin_lock_unlock(lock: *mut spinlock_t);
}
extern "C" {
    pub fn rt_spin_trylock_bh(__cond_acquires(true: *mut *mut spinlock_t lock), _arg: lock) -> c_int;
}
extern "C" {
    pub fn rt_spin_trylock(__cond_acquires(true: *mut *mut spinlock_t lock), _arg: lock) -> c_int;
}

//
// Always evaluate the 'subclass' argument to avoid that the compiler
// warns about set-but-not-used variables when building with
// CONFIG_DEBUG_LOCK_ALLOC=n and with W=1.
//

// Investigate: Drop bh when blocking ?

extern "C" {
    pub fn rt_spin_trylock(_arg: lock) -> return;
}

// flags = 0;
extern "C" {
    pub fn rt_spin_trylock(_arg: lock) -> return;
}

extern "C" {
    pub fn rt_mutex_base_is_locked(_arg: &lock->lock) -> return;
}

