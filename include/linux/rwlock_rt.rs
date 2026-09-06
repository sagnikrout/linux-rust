//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/rwlock_rt.h
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
    pub fn rt_read_lock(__acquires_shared(rwlock: *mut *mut rwlock_t rwlock));
}
extern "C" {
    pub fn rt_read_trylock(__cond_acquires_shared(true: *mut *mut rwlock_t rwlock), _arg: rwlock) -> c_int;
}
extern "C" {
    pub fn rt_read_unlock(__releases_shared(rwlock: *mut *mut rwlock_t rwlock));
}
extern "C" {
    pub fn rt_write_lock(__acquires(rwlock: *mut *mut rwlock_t rwlock));
}
extern "C" {
    pub fn rt_write_lock_nested(rwlock: *mut rwlock_t, __acquires(rwlock: int subclass));
}
extern "C" {
    pub fn rt_write_trylock(__cond_acquires(true: *mut *mut rwlock_t rwlock), _arg: rwlock) -> c_int;
}
extern "C" {
    pub fn rt_write_unlock(__releases(rwlock: *mut *mut rwlock_t rwlock));
}

// flags = 0;
extern "C" {
    pub fn rt_write_trylock(_arg: rwlock) -> return;
}

