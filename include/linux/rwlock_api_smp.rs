//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/rwlock_api_smp.h
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
// include/linux/rwlock_api_smp.h
//
// spinlock API declarations on SMP (and debug)
// (implemented in kernel/spinlock.c)
//
// portions Copyright 2005, Red Hat, Inc., Ingo Molnar
// Released under the General Public License (GPL).
//
extern "C" {
    pub fn _raw_read_lock(__acquires_shared(lock: *mut *mut rwlock_t lock)) -> void __lockfunc;
}
extern "C" {
    pub fn _raw_write_lock(__acquires(lock: *mut *mut rwlock_t lock)) -> void __lockfunc;
}
extern "C" {
    pub fn _raw_write_lock_nested(lock: *mut rwlock_t, __acquires(lock: int subclass)) -> void __lockfunc;
}
extern "C" {
    pub fn _raw_read_lock_bh(__acquires_shared(lock: *mut *mut rwlock_t lock)) -> void __lockfunc;
}
extern "C" {
    pub fn _raw_write_lock_bh(__acquires(lock: *mut *mut rwlock_t lock)) -> void __lockfunc;
}
extern "C" {
    pub fn _raw_read_lock_irq(__acquires_shared(lock: *mut *mut rwlock_t lock)) -> void __lockfunc;
}
extern "C" {
    pub fn _raw_write_lock_irq(__acquires(lock: *mut *mut rwlock_t lock)) -> void __lockfunc;
}
extern "C" {
    pub fn _raw_read_trylock(__cond_acquires_shared(true: *mut *mut rwlock_t lock), _arg: lock) -> int __lockfunc;
}
extern "C" {
    pub fn _raw_write_trylock(__cond_acquires(true: *mut *mut rwlock_t lock), _arg: lock) -> int __lockfunc;
}
extern "C" {
    pub fn _raw_read_unlock(__releases_shared(lock: *mut *mut rwlock_t lock)) -> void __lockfunc;
}
extern "C" {
    pub fn _raw_write_unlock(__releases(lock: *mut *mut rwlock_t lock)) -> void __lockfunc;
}
extern "C" {
    pub fn _raw_read_unlock_bh(__releases_shared(lock: *mut *mut rwlock_t lock)) -> void __lockfunc;
}
extern "C" {
    pub fn _raw_write_unlock_bh(__releases(lock: *mut *mut rwlock_t lock)) -> void __lockfunc;
}
extern "C" {
    pub fn _raw_read_unlock_irq(__releases_shared(lock: *mut *mut rwlock_t lock)) -> void __lockfunc;
}
extern "C" {
    pub fn _raw_write_unlock_irq(__releases(lock: *mut *mut rwlock_t lock)) -> void __lockfunc;
}

//
// If lockdep is enabled then we use the non-preemption spin-ops
// even on CONFIG_PREEMPT, because lockdep assumes that interrupts are
// not re-enabled during lock-acquire (which the preempt-spin-ops do):
//

