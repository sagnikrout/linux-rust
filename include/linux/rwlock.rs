//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/rwlock.h
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


//
// rwlock related methods
//
// split out from spinlock.h
//
// portions Copyright 2005, Red Hat, Inc., Ingo Molnar
// Released under the General Public License (GPL).
//

extern "C" {
    pub fn do_raw_read_lock(__acquires_shared(lock: *mut *mut rwlock_t lock));
}
extern "C" {
    pub fn do_raw_read_trylock(__cond_acquires_shared(true: *mut *mut rwlock_t lock), _arg: lock) -> c_int;
}
extern "C" {
    pub fn do_raw_read_unlock(__releases_shared(lock: *mut *mut rwlock_t lock));
}
extern "C" {
    pub fn do_raw_write_lock(__acquires(lock: *mut *mut rwlock_t lock));
}
extern "C" {
    pub fn do_raw_write_trylock(__cond_acquires(true: *mut *mut rwlock_t lock), _arg: lock) -> c_int;
}
extern "C" {
    pub fn do_raw_write_unlock(__releases(lock: *mut *mut rwlock_t lock));
}

extern "C" {
    pub fn arch_read_trylock(_arg: &(rwlock)->raw_lock) -> return;
}

extern "C" {
    pub fn arch_write_trylock(_arg: &(rwlock)->raw_lock) -> return;
}

//
// Define the various rw_lock methods.  Note we define these
// regardless of whether CONFIG_SMP or CONFIG_PREEMPT are set. The various
// methods are defined as nops in the case they are not required.
//

